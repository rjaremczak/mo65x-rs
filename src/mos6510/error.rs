use super::{addrmode::AddrMode, instruction::Instruction};

#[derive(Debug)]
pub enum AppError {
    UndefinedSymbol(String),
    MissingOperand,
    OpcodeNotFound(Instruction, AddrMode),
    SyntaxError,
    AddrOutOfRange(u16, u16),
    InvalidMnemonic(String),
    ParseIntError(String, std::num::ParseIntError),
    IoError(std::io::Error),
}

impl AppError {
    pub fn from_io(ioerr: std::io::Error) -> Self {
        Self::IoError(ioerr)
    }
}
