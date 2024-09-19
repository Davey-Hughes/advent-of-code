use crate::instruction::Instruction;
use crate::opcode::{ModeOpt, Opcode};
use core::fmt;
use std::{error::Error, fs};
use tokio::sync::mpsc;

struct IO {
    rx: mpsc::Receiver<i64>,
    tx: Option<mpsc::Sender<i64>>,
    history: Vec<i64>,
}

pub struct Interpreter {
    program: Vec<i64>,
    input: IO,
    output: IO,

    pc: usize,
}

#[allow(clippy::missing_errors_doc)]
impl Interpreter {
    fn parse_input(file_string: &str) -> Result<Vec<i64>, Box<dyn Error>> {
        Ok(file_string
            .lines()
            .next()
            .ok_or("Error reading line from input file")?
            .split(',')
            .map(str::parse::<i64>)
            .collect::<Result<Vec<_>, _>>()?)
    }

    pub async fn from_file(file: &str, input: Vec<i64>) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(file)?;
        let (output_tx, output_rx) = mpsc::channel(32);
        let (input_tx, input_rx) = mpsc::channel(32);

        for i in input.clone() {
            input_tx.send(i).await?;
        }

        Ok(Self {
            program: Self::parse_input(&contents)?,
            input: IO {
                rx: input_rx,
                tx: Some(input_tx),
                history: input,
            },
            output: IO {
                rx: output_rx,
                tx: Some(output_tx),
                history: vec![],
            },
            pc: 0,
        })
    }

    #[must_use]
    pub async fn output(&mut self) -> Option<i64> {
        self.output.rx.recv().await
    }

    /// # Panics
    ///
    /// Panics if the input channel is closed but the program expected input
    pub async fn exec_one(&mut self) -> Result<Option<()>, Box<dyn Error>> {
        let ins = Instruction::new(&self.program, self.pc)?;
        let params = &ins.parameters[1..];

        let get_param_value = |i| -> Result<i64, Box<dyn Error>> {
            Ok(
                match ins.modes.get(i).ok_or("Index doesn't exist for mode")? {
                    ModeOpt::Position => self.program[usize::try_from(params[i])?],
                    ModeOpt::Immediate => params[i],
                },
            )
        };

        match ins.opcode {
            Opcode::Add => {
                self.program[usize::try_from(params[2])?] =
                    get_param_value(0)? + get_param_value(1)?;
            }
            Opcode::Mul => {
                self.program[usize::try_from(params[2])?] =
                    get_param_value(0)? * get_param_value(1)?;
            }
            Opcode::In => {
                if let Some(message) = self.input.rx.recv().await {
                    self.program[usize::try_from(params[0])?] = message;
                } else {
                    panic!("Input channel closed but program expects input");
                }
            }
            Opcode::Out => {
                let message = self.program[usize::try_from(params[0])?];
                self.output
                    .tx
                    .as_mut()
                    .ok_or("Tried to output when program already halted")?
                    .send(message)
                    .await?;
                self.output.history.push(message);
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
                self.program[usize::try_from(params[2])?] = val;
            }
            Opcode::Eq => {
                let val = i64::from(get_param_value(0)? == get_param_value(1)?);
                self.program[usize::try_from(params[2])?] = val;
            }
            Opcode::Halt => {
                // manually drops the input and output senders
                self.input.tx = None;
                self.output.tx = None;
                return Ok(Some(()));
            }
        }

        self.pc += ins.opcode.len();

        Ok(None)
    }

    pub async fn exec(&mut self) -> Result<(), Box<dyn Error>> {
        while self.pc < self.program.len() {
            if let Some(_output) = self.exec_one().await? {
                return Ok(());
            }
        }
        Err("Program did not halt")?
    }
}
impl fmt::Debug for Interpreter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "input:\t{:?}", self.input.history)?;
        writeln!(f, "output:\t{:?}", self.output.history)?;
        writeln!(f, "pc:\t{:?}", self.pc)?;

        writeln!(f, "\nprogram: ")?;
        for chunk in self.program.chunks(16) {
            writeln!(f, "\t{chunk:?}")?;
        }

        Ok(())
    }
}

impl fmt::Display for Interpreter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut addr = 0;
        loop {
            let Ok(ins) = Instruction::new(&self.program, addr) else {
                writeln!(
                    f,
                    "\tInvalid instruction: {} at: {:#x}",
                    self.program[addr], addr
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

            if addr >= self.program.len() || ins.opcode == Opcode::Halt {
                break;
            }
        }

        Ok(())
    }
}
