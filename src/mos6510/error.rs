use std::error;

use structopt::clap::App;

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
    EmulatorIsRunning,
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}
