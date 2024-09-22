use crate::opcode::{ModeOpt, Opcode};
use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub modes: Vec<ModeOpt>,
    pub parameters: Vec<i64>,
}

impl Instruction {
    pub fn new(program: &[i64], addr: usize) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let op = program
            .get(addr)
            .ok_or("Address {addr} does not exist in the program.")?;

        let chars = format!("{op:05}").chars().collect::<Vec<_>>();

        let opcode: Opcode = chars[chars.len() - 2..chars.len()]
            .iter()
            .collect::<String>()
            .parse::<u8>()?
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

        if (addr + opcode.len()) > program.len() {
            return Err("Program does not contain enough data for the instruction.")?;
        }

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

            params.push(format!("{}{}{}", brackets.0, p, brackets.1));
        }

        write!(f, "\t{}", params.join(", "))?;

        Ok(())
    }
}
