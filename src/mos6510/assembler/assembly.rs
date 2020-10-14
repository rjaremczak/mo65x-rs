use super::asm_result::AsmError;
use super::object_code::ObjectCode;
use std::collections::HashMap;

pub enum AsmPhase {
    Scanning,
    Generating,
}

type Symbols = HashMap<String, u16>;

pub struct AsmState {
    pub label: Option<String>,
    pub operation: Option<String>,
    pub operand: Option<String>,
    pub phase: AsmPhase,
    pub location_counter: u16,
    pub location_counter_prev: u16,
    pub bytes_written: u32,
    pub symbols: Symbols,
    pub object_code: ObjectCode,
}

impl AsmState {
    pub fn new() -> AsmState {
        AsmState {
            label: Option::None,
            operation: Option::None,
            operand: Option::None,
            phase: AsmPhase::Scanning,
            location_counter: 0,
            location_counter_prev: 0,
            bytes_written: 0,
            symbols: Symbols::new(),
            object_code: ObjectCode::new(),
        }
    }

    fn resolve_operand_value(identifier: &str) -> Result<i32, AsmError> {
        Result::Err(AsmError::SyntaxError)
    }

    pub fn handle_symbol(&mut self, label: Option<String>) -> AsmError {
        match label {
            Some(symbol) => match self.phase {
                AsmPhase::Scanning => match self.symbols.insert(symbol, self.location_counter) {
                    Some(_) => AsmError::SymbolAlreadyDefined,
                    None => AsmError::Ok,
                },
                AsmPhase::Generating => AsmError::InvalidPhase,
            },
            None => AsmError::SymbolNotDefined,
        }
    }

    pub fn handle_no_operation(&mut self) -> AsmError {
        println!("empty line");
        AsmError::Ok
    }

    pub fn handle_set_location_counter(&mut self) -> AsmError {
        println!("set location counter");
        AsmError::Ok
    }

    pub fn handle_emit_bytes(&mut self) -> AsmError {
        AsmError::InvalidMnemonic
    }

    pub fn handle_emit_words(&mut self) -> AsmError {
        AsmError::InvalidMnemonic
    }

    pub fn handle_implied(&mut self) -> AsmError {
        AsmError::InvalidMnemonic
    }

    pub fn handle_immediate(&mut self) -> AsmError {
        AsmError::InvalidMnemonic
    }

    pub fn handle_branch(&mut self) -> AsmError {
        AsmError::InvalidMnemonic
    }

    pub fn handle_absolute(&mut self) -> AsmError {
        AsmError::InvalidMnemonic
    }

    pub fn handle_absolute_indexed_x(&mut self) -> AsmError {
        AsmError::InvalidMnemonic
    }

    pub fn handle_absolute_indexed_y(&mut self) -> AsmError {
        AsmError::InvalidMnemonic
    }

    pub fn handle_indirect(&mut self) -> AsmError {
        AsmError::InvalidMnemonic
    }

    pub fn handle_indexed_indirect_x(&mut self) -> AsmError {
        AsmError::InvalidMnemonic
    }

    pub fn handle_indirect_indexed_y(&mut self) -> AsmError {
        AsmError::InvalidMnemonic
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state() {
        let st = AsmState::new();
        assert!(matches!(st.phase, AsmPhase::Scanning));
        assert_eq!(st.label, None);
        assert_eq!(st.operation, None);
        assert_eq!(st.operand, None);
        assert_eq!(st.bytes_written, 0);
        assert_eq!(st.location_counter, 0);
        assert_eq!(st.location_counter_prev, 0);
        assert!(st.symbols.is_empty());
    }
}
