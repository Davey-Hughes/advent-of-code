use core::fmt;
use std::{
    error::Error,
    fs,
    ops::{Deref, DerefMut, Index, IndexMut},
};

use tokio::sync::mpsc;

use crate::{
    instruction::Instruction,
    opcode::{ModeOpt, Opcode},
};

pub struct Memory(Vec<i64>);

#[allow(dead_code)]
impl Memory {
    fn get_mut(&mut self, addr: usize) -> Result<&mut i64, Box<dyn Error + Send + Sync>> {
        if addr < self.0.len() {
            return Ok(self
                .0
                .get_mut(addr)
                .ok_or("Could not access memory address 0")?);
        }

        self.0.resize(addr + 1, 0);
        Ok(self
            .0
            .get_mut(addr)
            .ok_or("Could not access memory address 2")?)
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl Index<usize> for Memory {
    type Output = i64;
    fn index(&self, i: usize) -> &i64 {
        if i < self.0.len() {
            return &self.0[i];
        }

        &0
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, i: usize) -> &mut i64 {
        if i >= self.0.len() {
            self.0.resize(i + 1, 0);
        }

        &mut self.0[i]
    }
}

impl Deref for Memory {
    type Target = Vec<i64>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Memory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.0.chunks(8) {
            let mut out_line = vec![];
            for val in line {
                out_line.push(format!("{val:8}\t"));
            }
            writeln!(f, "{}", out_line.join("\t"))?;
        }

        Ok(())
    }
}

pub struct Executor {
    pub memory: Memory,
    pub pc: usize,
    pub rel: isize,

    input_rx: mpsc::Receiver<i64>,
    input_history: Vec<i64>,

    output_tx: Option<mpsc::UnboundedSender<i64>>,
    output_history: Vec<i64>,
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
        output_tx: mpsc::UnboundedSender<i64>,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let contents = fs::read_to_string(file)?;

        Ok(Self {
            memory: Memory(Self::parse_input(&contents)?),
            pc: 0,
            rel: 0,

            input_rx,
            input_history: vec![],

            output_tx: Some(output_tx),
            output_history: vec![],
        })
    }

    pub fn output_history(&self) -> &[i64] {
        &self.output_history
    }

    /// # Panics
    ///
    /// Panics if the input channel is closed but the program expected input
    pub async fn exec_one(&mut self) -> Result<Option<i64>, Box<dyn Error + Send + Sync>> {
        enum Access {
            Fetch,
            Store,
        }

        let ins = Instruction::new(&self.memory.0, self.pc)?;
        let params = &ins.parameters[1..];

        let get_param_value = |i: usize, a: Access| -> Result<i64, Box<dyn Error + Send + Sync>> {
            Ok(
                match (a, ins.modes.get(i).ok_or("Index doesn't exist for mode")?) {
                    (Access::Fetch, ModeOpt::Position) => self.memory[usize::try_from(params[i])?],
                    (Access::Store, ModeOpt::Position) | (_, ModeOpt::Immediate) => params[i],
                    (Access::Fetch, ModeOpt::Relative) => {
                        self.memory[usize::try_from(self.rel + isize::try_from(params[i])?)?]
                    }
                    (Access::Store, ModeOpt::Relative) => {
                        i64::try_from(self.rel + isize::try_from(params[i])?)?
                    }
                },
            )
        };

        match ins.opcode {
            Opcode::Add => {
                let location = usize::try_from(get_param_value(2, Access::Store)?)?;
                self.memory[location] =
                    get_param_value(0, Access::Fetch)? + get_param_value(1, Access::Fetch)?;
            }
            Opcode::Mul => {
                let location = usize::try_from(get_param_value(2, Access::Store)?)?;
                self.memory[location] =
                    get_param_value(0, Access::Fetch)? * get_param_value(1, Access::Fetch)?;
            }
            Opcode::In => {
                if let Some(message) = self.input_rx.recv().await {
                    let location = usize::try_from(get_param_value(0, Access::Store)?)?;
                    self.memory[location] = message;
                    self.input_history.push(message);
                } else {
                    panic!("Input channel closed but program expects input");
                }
            }
            Opcode::Out => {
                let message = get_param_value(0, Access::Fetch)?;
                self.output_tx
                    .as_mut()
                    .expect("Tried to output when program already halted")
                    .send(message)
                    .expect("Tried to output when program already halted");
                self.output_history.push(message);
            }
            Opcode::Jt => {
                if get_param_value(0, Access::Fetch)? != 0 {
                    self.pc = usize::try_from(get_param_value(1, Access::Fetch)?)?;
                    return Ok(None);
                }
            }
            Opcode::Jf => {
                if get_param_value(0, Access::Fetch)? == 0 {
                    self.pc = usize::try_from(get_param_value(1, Access::Fetch)?)?;
                    return Ok(None);
                }
            }
            Opcode::Lt => {
                let val = i64::from(
                    get_param_value(0, Access::Fetch)? < get_param_value(1, Access::Fetch)?,
                );
                let location = usize::try_from(get_param_value(2, Access::Store)?)?;
                self.memory[location] = val;
            }
            Opcode::Eq => {
                let val = i64::from(
                    get_param_value(0, Access::Fetch)? == get_param_value(1, Access::Fetch)?,
                );
                let location = usize::try_from(get_param_value(2, Access::Store)?)?;
                self.memory[location] = val;
            }
            Opcode::Rel => {
                self.rel += isize::try_from(get_param_value(0, Access::Fetch)?)?;
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
            let pc_indicator = if addr == self.pc { "> " } else { "  " };

            writeln!(f, "{pc_indicator}{addr:08x}:\t{ins}")?;
            addr += ins.opcode.len();
        }

        Ok(())
    }
}
