#[derive(Debug)]
pub enum AppError {
    SymbolNotDefined,
    MissingOperand,
    MissingOperation,
    SyntaxError,
    ValueOutOfRange,
    InvalidMnemonic,
    ParseIntError(String, std::num::ParseIntError),
    IoError(std::io::Error),
}
