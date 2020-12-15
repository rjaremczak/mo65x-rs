use super::{addrmode::AddrMode, instruction::Instruction};
use crossterm::ErrorKind;

#[derive(Debug)]
pub enum AppError {
    UndefinedSymbol(String),
    MissingOperand,
    NoOpCode(Instruction, AddrMode),
    SyntaxError,
    AddrOutOfRange(u16, u16),
    InvalidMnemonic(String),
    ParseIntError(String, std::num::ParseIntError),
    IoError(std::io::Error),
    EmulatorAlreadyRunning,
    GeneralError(String),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<ErrorKind> for AppError {
    fn from(err: ErrorKind) -> Self {
        Self::GeneralError(err.to_string())
    }
}
