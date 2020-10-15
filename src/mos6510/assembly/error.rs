pub enum AsmError {
    Ok,
    SymbolAlreadyDefined,
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
