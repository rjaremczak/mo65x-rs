use crate::mos6510::{addrmode::AddrMode, instruction::Instruction};
use std::fmt::Display;

#[derive(Debug)]
#[allow(dead_code)]
pub enum AppError {
    UndefinedSymbol(String),
    RedefinedSymbol(String, i32, i32),
    MissingOperand,
    NoOpCode(Instruction, AddrMode),
    SyntaxError(String),
    OriginTooLow(u16, u16),
    BranchTooFar(i32),
    InvalidMnemonic(String),
    ParseIntError(String, std::num::ParseIntError),
    IoError(std::io::Error),
    EmulatorAlreadyRunning,
    EmulatorNotRunning,
    InvalidOpCode(u16, u8),
    MiniFbError(minifb::Error),
    AsmLineError(usize, Box<AppError>),
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

impl From<minifb::Error> for AppError {
    fn from(err: minifb::Error) -> Self {
        Self::MiniFbError(err)
    }
}
