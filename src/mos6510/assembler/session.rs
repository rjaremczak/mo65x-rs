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

pub struct AsmSession {
    pub symbols: HashMap<String, u16>,
    pub operation: String,
    pub operand: String,
    pub phase: AsmPhase,
    pub location_counter: u16,
    pub location_counter_prev: u16,
    pub bytes_written: u32,
    pub address_range: RangeInclusive<u16>,
}

impl AsmSession {
    pub fn new() -> Self {
        Self {
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
    pub fn handle_emit_bytes(&self, session: &mut AsmSession) -> AsmResult {
        AsmResult::InvalidMnemonic
    }
    fn handle_emit_words(&self, session: &mut AsmSession) {}
    fn handle_implied(&self, session: &mut AsmSession) {}
    fn handle_immediate(&self, session: &mut AsmSession) {}
    fn handle_branch(&self, session: &mut AsmSession) {}
    fn handle_absolute(&self, session: &mut AsmSession) {}
    fn handle_absolute_indexed_x(&self, session: &mut AsmSession) {}
    fn handle_absolute_indexed_y(&self, session: &mut AsmSession) {}
    fn handle_indirect(&self, session: &mut AsmSession) {}
    fn handle_indexed_indirect_x(&self, session: &mut AsmSession) {}
    fn handle_indirect_indexed_y(&self, session: &mut AsmSession) {}
}
