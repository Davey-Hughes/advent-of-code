use core::fmt;
use std::{error::Error, fs};

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
    Mov = 3,
    Out = 4,
    Halt = 99,
}

impl Opcode {
    const fn len(&self) -> usize {
        match self {
            Self::Add | Self::Mul => 3,
            Self::Mov | Self::Out => 2,
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
            3 => Ok(Self::Mov),
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

        for (i, p) in self.parameters.iter().enumerate() {
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

#[derive(Debug)]
pub struct Interpreter {
    program: Vec<i64>,
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

    pub fn from_file(file: &str) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(file)?;

        Ok(Self {
            program: Self::parse_input(&contents)?,
            output: vec![],
            pc: 0,
        })
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

            writeln!(f, "{addr:#08x}:\t{ins}")?;
            addr += ins.opcode.len();

            if addr >= self.program.len() || ins.opcode == Opcode::Halt {
                break;
            }
        }

        Ok(())
    }
}
