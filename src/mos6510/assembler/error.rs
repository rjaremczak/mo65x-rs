#[derive(Debug)]
pub enum AsmError {
    SymbolNotDefined,
    MissingOperand,
    SyntaxError,
    ValueOutOfRange,
    InvalidMnemonic,
    InvalidInstructionFormat,
    MalformedOperand(String, std::num::ParseIntError),
}
