use std::collections::HashMap;
use std::ops::RangeInclusive;

pub enum ProcessingStatus {
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
}

pub enum Phase {
    Scanning,
    Generating,
}

pub struct AssemblerSession {
    symbols: HashMap<String, u16>,
    operation: String,
    operand: String,
    phase: Phase,
    location_counter: u16,
    location_counter_prev: u16,
    bytes_written: u32,
    address_range: RangeInclusive<u16>,
}

impl AssemblerSession {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            operation: String::new(),
            operand: String::new(),
            phase: Phase::Scanning,
            location_counter: 0,
            location_counter_prev: 0,
            bytes_written: 0,
            address_range: RangeInclusive::new(1, 0),
        }
    }
    pub fn start_code_generation() {}
}
