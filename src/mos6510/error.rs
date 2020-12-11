use std::error::{self, Error};

use structopt::clap::App;

use super::{addrmode::AddrMode, instruction::Instruction};

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
