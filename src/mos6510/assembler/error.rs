#[derive(Debug)]
pub enum AsmError {
    Ok,
    SymbolNotDefined,
    MissingOperand,
    SyntaxError,
    ValueOutOfRange,
    InvalidMnemonic,
    InvalidInstructionFormat,
    MalformedOperand(String, std::num::ParseIntError),
}
