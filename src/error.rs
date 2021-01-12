use std::fmt::Display;

use crate::mos6510::{addrmode::AddrMode, instruction::Instruction};
use crossterm::ErrorKind;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    UndefinedSymbol(String),
    MissingOperand,
    NoOpCode(Instruction, AddrMode),
    SyntaxError(String),
    AddrOutOfRange(u16, u16),
    InvalidMnemonic(String),
    ParseIntError(String, std::num::ParseIntError),
    IoError(std::io::Error),
    EmulatorAlreadyRunning,
    EmulatorNotRunning,
    InvalidOpCode(u16, u8),
    GeneralError(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", *self)
    }
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

impl From<minifb::Error> for AppError {
    fn from(err: minifb::Error) -> Self {
        Self::GeneralError(err.to_string())
    }
}
