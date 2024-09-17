use crate::instruction::Instruction;
use crate::opcode::{ModeOpt, Opcode};
use core::fmt;
use std::{collections::VecDeque, error::Error, fs};

pub struct Interpreter {
    program: Vec<i64>,
    input: VecDeque<i64>,
    output: Vec<i64>,

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

    pub fn from_file(file: &str, input: Vec<i64>) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(file)?;

        Ok(Self {
            program: Self::parse_input(&contents)?,
            input: VecDeque::from(input),
            output: vec![],
            pc: 0,
        })
    }

    #[must_use]
    pub fn output(&self) -> &[i64] {
        &self.output
    }

    pub fn exec_one(&mut self) -> Result<Option<Vec<i64>>, Box<dyn Error>> {
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
                self.program[usize::try_from(params[0])?] = self
                    .input
                    .pop_front()
                    .ok_or("No input for In instruction")?;
            }
            Opcode::Out => {
                self.output.push(self.program[usize::try_from(params[0])?]);
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
            Opcode::Halt => return Ok(Some(self.output.clone())),
        }

        self.pc += ins.opcode.len();

        Ok(None)
    }

    pub fn exec(&mut self) -> Result<Vec<i64>, Box<dyn Error>> {
        while self.pc < self.program.len() {
            if let Some(output) = self.exec_one()? {
                return Ok(output);
            }
        }
        Err("Program did not halt")?
    }
}
impl fmt::Debug for Interpreter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "input:\t{:?}", self.input)?;
        writeln!(f, "output:\t{:?}", self.output)?;
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
