use num_enum::TryFromPrimitive;

#[derive(Debug, Default, PartialEq, Eq)]
pub enum ModeOpt {
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

#[derive(Debug, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum Opcode {
    Add = 1,
    Mul = 2,
    In = 3,
    Out = 4,
    Jt = 5,
    Jf = 6,
    Lt = 7,
    Eq = 8,
    Halt = 99,
}

impl Opcode {
    pub const fn len(&self) -> usize {
        match self {
            Self::Add | Self::Mul | Self::Lt | Self::Eq => 4,
            Self::Jt | Self::Jf => 3,
            Self::In | Self::Out => 2,
            Self::Halt => 1,
        }
    }
}
