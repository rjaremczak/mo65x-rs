pub enum AsmError {
    Ok,
    SymbolNotDefined,
    MissingOperand,
    NumericOperandRequired,
    SyntaxError,
    CommandProcessingError,
    ValueOutOfRange,
    InvalidMnemonic,
    InvalidInstructionFormat,
    InvalidPhase,
    MalformedOperand(std::num::ParseIntError),
}
