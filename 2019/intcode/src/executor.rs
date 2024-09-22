use core::fmt;
use std::{error::Error, fs};

use tokio::sync::mpsc;

use crate::{
    instruction::Instruction,
    opcode::{ModeOpt, Opcode},
};

pub struct Executor {
    memory: Vec<i64>,
    pc: usize,

    input_rx: mpsc::Receiver<i64>,
    output_tx: Option<mpsc::Sender<i64>>,
}

impl Executor {
    fn parse_input(file_string: &str) -> Result<Vec<i64>, Box<dyn Error + Send + Sync>> {
        Ok(file_string
            .lines()
            .next()
            .ok_or("Error reading line from input file")?
            .split(',')
            .map(str::parse::<i64>)
            .collect::<Result<Vec<_>, _>>()?)
    }

    pub fn from_file(
        file: &str,
        input_rx: mpsc::Receiver<i64>,
        output_tx: mpsc::Sender<i64>,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let contents = fs::read_to_string(file)?;

        Ok(Self {
            memory: Self::parse_input(&contents)?,
            pc: 0,

            input_rx,
            output_tx: Some(output_tx),
        })
    }

    /// # Panics
    ///
    /// Panics if the input channel is closed but the program expected input
    pub async fn exec_one(&mut self) -> Result<Option<i64>, Box<dyn Error + Send + Sync>> {
        let ins = Instruction::new(&self.memory, self.pc)?;
        let params = &ins.parameters[1..];

        let get_param_value = |i| -> Result<i64, Box<dyn Error + Send + Sync>> {
            Ok(
                match ins.modes.get(i).ok_or("Index doesn't exist for mode")? {
                    ModeOpt::Position => self.memory[usize::try_from(params[i])?],
                    ModeOpt::Immediate => params[i],
                },
            )
        };

        match ins.opcode {
            Opcode::Add => {
                self.memory[usize::try_from(params[2])?] =
                    get_param_value(0)? + get_param_value(1)?;
            }
            Opcode::Mul => {
                self.memory[usize::try_from(params[2])?] =
                    get_param_value(0)? * get_param_value(1)?;
            }
            Opcode::In => {
                if let Some(message) = self.input_rx.recv().await {
                    self.memory[usize::try_from(params[0])?] = message;
                } else {
                    panic!("Input channel closed but program expects input");
                }
            }
            Opcode::Out => {
                let message = self.memory[usize::try_from(params[0])?];
                self.output_tx
                    .as_mut()
                    .expect("Tried to output when program already halted")
                    .send(message)
                    .await
                    .expect("Tried to output when program already halted");
                // self.output.history.push(message);
            }
            Opcode::Jt => {
                if get_param_value(0)? != 0 {
                    self.pc = usize::try_from(get_param_value(1)?)?;
                    return Ok(None);
                }
            }
            Opcode::Jf => {
                if get_param_value(0)? == 0 {
                    self.pc = usize::try_from(get_param_value(1)?)?;
                    return Ok(None);
                }
            }
            Opcode::Lt => {
                let val = i64::from(get_param_value(0)? < get_param_value(1)?);
                self.memory[usize::try_from(params[2])?] = val;
            }
            Opcode::Eq => {
                let val = i64::from(get_param_value(0)? == get_param_value(1)?);
                self.memory[usize::try_from(params[2])?] = val;
            }
            Opcode::Halt => {
                // manually drops the output senders
                self.output_tx = None;

                return Ok(Some(0));
            }
        }

        self.pc += ins.opcode.len();

        Ok(None)
    }

    pub async fn exec(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        while self.pc < self.memory.len() {
            if let Some(_output) = self.exec_one().await? {
                return Ok(());
            }
        }
        Err("Program did not halt")?
    }
}

impl fmt::Debug for Executor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // writeln!(f, "input:\t{:?}", self.input.history)?;
        // writeln!(f, "output:\t{:?}", self.output.history)?;
        writeln!(f, "pc:\t{:?}", self.pc)?;

        writeln!(f, "\nprogram: ")?;
        for chunk in self.memory.chunks(16) {
            writeln!(f, "\t{chunk:?}")?;
        }

        Ok(())
    }
}

impl fmt::Display for Executor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut addr = 0;
        while addr < self.memory.len() {
            let Ok(ins) = Instruction::new(&self.memory, addr) else {
                writeln!(
                    f,
                    "\tInvalid instruction: {} at: {:#x}",
                    self.memory[addr], addr
                )?;

                // try from next address
                addr += 1;
                continue;
            };

            // if the parsed instruction includes the program counter, skip to the program counter
            // and parse the instruction again
            if (addr + 1..addr + ins.opcode.len()).contains(&self.pc) {
                addr = self.pc;
                continue;
            }

            // show which line the program counter is on
            if addr == self.pc {
                write!(f, "> ")?;
            }

            writeln!(f, "{addr:08x}:\t{ins}")?;
            addr += ins.opcode.len();
        }

        Ok(())
    }
}
