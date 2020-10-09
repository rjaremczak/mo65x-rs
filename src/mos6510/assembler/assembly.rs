use std::collections::HashMap;
use std::ops::RangeInclusive;

pub enum AsmPhase {
    Scanning,
    Generating,
}

pub enum AsmResult {
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

pub struct AsmState {
    pub symbols: HashMap<String, u16>,
    pub operation: String,
    pub operand: String,
    pub phase: AsmPhase,
    pub location_counter: u16,
    pub location_counter_prev: u16,
    pub bytes_written: u32,
    pub address_range: RangeInclusive<u16>,
}

impl AsmState {
    pub fn new() -> AsmState {
        AsmState {
            symbols: HashMap::new(),
            operation: String::new(),
            operand: String::new(),
            phase: AsmPhase::Scanning,
            location_counter: 0,
            location_counter_prev: 0,
            bytes_written: 0,
            address_range: RangeInclusive::new(1, 0),
        }
    }

    pub fn handle_no_operation(&mut self) -> AsmResult {
        println!("empty line");
        AsmResult::Ok
    }
    pub fn handle_set_location_counter(&mut self) -> AsmResult {
        println!("set location counter");
        AsmResult::Ok
    }
    pub fn handle_emit_bytes(&mut self) -> AsmResult {
        AsmResult::InvalidMnemonic
    }
    pub fn handle_emit_words(&mut self) -> AsmResult {
        AsmResult::InvalidMnemonic
    }
    pub fn handle_implied(&mut self) -> AsmResult {
        AsmResult::InvalidMnemonic
    }
    pub fn handle_immediate(&mut self) -> AsmResult {
        AsmResult::InvalidMnemonic
    }
    pub fn handle_branch(&mut self) -> AsmResult {
        AsmResult::InvalidMnemonic
    }
    pub fn handle_absolute(&mut self) -> AsmResult {
        AsmResult::InvalidMnemonic
    }
    pub fn handle_absolute_indexed_x(&mut self) -> AsmResult {
        AsmResult::InvalidMnemonic
    }
    pub fn handle_absolute_indexed_y(&mut self) -> AsmResult {
        AsmResult::InvalidMnemonic
    }
    pub fn handle_indirect(&mut self) -> AsmResult {
        AsmResult::InvalidMnemonic
    }
    pub fn handle_indexed_indirect_x(&mut self) -> AsmResult {
        AsmResult::InvalidMnemonic
    }
    pub fn handle_indirect_indexed_y(&mut self) -> AsmResult {
        AsmResult::InvalidMnemonic
    }
}
