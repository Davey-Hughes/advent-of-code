use core::fmt;
use std::{collections::VecDeque, error::Error, fs};

#[derive(Debug, Default, PartialEq)]
enum ModeOpt {
    #[default]
    Position = 0,
    Immediate = 1,
}

impl TryFrom<u32> for ModeOpt {
    type Error = &'static str;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Position),
            1 => Ok(Self::Immediate),
            _ => Err("Invalid mode"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Opcode {
    Add = 1,
    Mul = 2,
    In = 3,
    Out = 4,
    Halt = 99,
}

impl Opcode {
    const fn len(&self) -> usize {
        match self {
            Self::Add | Self::Mul => 4,
            Self::In | Self::Out => 2,
            Self::Halt => 1,
        }
    }
}

impl TryFrom<u32> for Opcode {
    type Error = &'static str;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Add),
            2 => Ok(Self::Mul),
            3 => Ok(Self::In),
            4 => Ok(Self::Out),
            99 => Ok(Self::Halt),
            _ => Err("Invalid opcode"),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    modes: Vec<ModeOpt>,
    parameters: Vec<i64>,
}

impl Instruction {
    fn new(program: &[i64], addr: usize) -> Result<Self, Box<dyn Error>> {
        let op = program
            .get(addr)
            .ok_or("Address {addr} does not exist in the program.")?;

        let chars = format!("{op:05}").chars().collect::<Vec<_>>();

        let opcode: Opcode = chars[chars.len() - 2..chars.len()]
            .iter()
            .map(|c| c.to_digit(10))
            .collect::<Option<Vec<_>>>()
            .ok_or("Error converting opcode to digits")?
            .into_iter()
            .fold(0, |acc, x| acc * 10 + x)
            .try_into()?;

        let modes = chars[0..3]
            .iter()
            .rev()
            .map(|c| c.to_digit(10))
            .collect::<Option<Vec<_>>>()
            .ok_or("Error converting modes to digits")?
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>, _>>()?;

        let parameters = program[addr..addr + opcode.len()].to_vec();

        Ok(Self {
            opcode,
            modes,
            parameters,
        })
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.opcode)?;

        if self.parameters.is_empty() {
            return Ok(());
        }

        let mut params = vec![];

        for (i, p) in self.parameters[1..].iter().enumerate() {
            let brackets = if self.modes[i] == ModeOpt::Position {
                ("[", "]")
            } else {
                ("", "")
            };

            params.push(format!("{}{:?}{}", brackets.0, p, brackets.1));
        }

        write!(f, "\t{}", params.join(" "))?;

        Ok(())
    }
}

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
                    "Error reading instruction: {} at address: {}",
                    self.program[addr], addr
                )?;
                break;
            };

            if addr == self.pc {
                write!(f, "> ")?;
            }

            writeln!(f, "{addr:#08x}:\t{ins}")?;
            addr += ins.opcode.len();

            if addr >= self.program.len() || ins.opcode == Opcode::Halt {
                break;
            }
        }

        Ok(())
    }
}
