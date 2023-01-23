#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Regs {
    ACC = 1,
    R1 = 2,
    R2 = 3,
    R3 = 4,
    R4 = 5,
    R5 = 6,
    R6 = 7,
    R7 = 8,
    R8 = 9,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LVal {
    Mem(u16),
    Hex(u16),
    Reg(Regs),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RVal {
    Reg(Regs),
    Mem(u16),
}

#[derive(Debug, PartialEq)]
pub enum ASMCommand {
    Mov(LVal, RVal),
}

#[derive(Debug)]
pub enum ArgType {
    Hex(u16),
    Mem(u16),
    Reg(Regs),
}

impl From<ArgType> for LVal {
    fn from(value: ArgType) -> Self {
        match value {
            ArgType::Hex(val) => LVal::Hex(val),
            ArgType::Mem(val) => LVal::Mem(val),
            ArgType::Reg(reg) => LVal::Reg(reg),
        }
    }
}

impl TryFrom<ArgType> for RVal {
    type Error = String;

    fn try_from(value: ArgType) -> Result<Self, Self::Error> {
        match value {
            ArgType::Hex(_) => Err("Not suted for rval".to_owned()),
            ArgType::Mem(val) => Ok(RVal::Mem(val)),
            ArgType::Reg(reg) => Ok(RVal::Reg(reg)),
        }
    }
}

pub fn parse_u16(val: &u16) -> [u8; 2] {
    [(val >> 8) as u8, (val & 0x00ff) as u8]
}
