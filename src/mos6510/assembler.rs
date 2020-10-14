mod asm_result;
mod assembly;
mod object_code;
mod operand;

use asm_result::*;
use assembly::*;
use operand::*;
use regex::Regex;

type PatternHandler = fn(&mut AsmState) -> AsmError;

struct Pattern {
    regex: Regex,
    handler: PatternHandler,
}

impl Pattern {
    pub fn new(pattern: &str, handler: PatternHandler) -> Pattern {
        Pattern {
            regex: Regex::new(pattern).unwrap(),
            handler,
        }
    }
    pub fn from(pattern: String, handler: PatternHandler) -> Pattern {
        Pattern::new(&pattern, handler)
    }
}

pub struct Assembler {
    patterns: Vec<Pattern>,
}

impl Assembler {
    pub fn new() -> Assembler {
        let symbol = String::from("[a-z]\\w*");
        let label = format!("^(?:({}):)?\\s*", symbol);
        let comment = String::from("(?:;.*)?$");
        let org_cmd = String::from("((?:\\.ORG\\s+)|(?:\\*\\s*=\\s*))");
        let byte_cmd = String::from("(\\.BYTE|DCB)\\s+");
        let word_cmd = String::from("(\\.WORD)\\s+");
        let hex_num = String::from("\\$[\\d|a-h]{1,4}");
        let dec_num = String::from("\\d{1,5}");
        let bin_num = String::from("%[01]{1,16}");
        let mnemonic = String::from("([a-z]{3})\\s*");
        let num_or_symbol = format!("(?:{})|(?:{})|(?:{})|(?:{})", hex_num, dec_num, bin_num, symbol);
        let lo_hi_prefix = format!("[{}|{}]?", LO_BYTE_MODIFIER, HI_BYTE_MODIFIER);
        let operand = format!("({}(?:{}))\\s*", lo_hi_prefix, num_or_symbol);
        let operand_separator = String::from("\\s*,?\\s*");
        let operand_list = format!("((?:(?:{}(?:{})){})+)\\s*", lo_hi_prefix, num_or_symbol, operand_separator);
        let branch_mnemonic = String::from("(BCC|BCS|BNE|BEQ|BMI|BPL|BVC|BVS)\\s*");
        let branch_target = format!("((?:[+|-]?\\d{{1,3}})|(?:{}))\\s*", symbol);
        Assembler {
            patterns: vec![
                Pattern::new("", AsmState::handle_no_operation),
                Pattern::from(format!("{}{}", org_cmd, operand), AsmState::handle_set_location_counter),
                Pattern::from(format!("{}{}", byte_cmd, operand_list), AsmState::handle_emit_bytes),
                Pattern::from(format!("{}{}", word_cmd, operand_list), AsmState::handle_emit_words),
                Pattern::from(format!("{}", mnemonic), AsmState::handle_implied),
                Pattern::from(format!("{}#{}", mnemonic, operand), AsmState::handle_immediate),
                Pattern::from(format!("{}{}", branch_mnemonic, branch_target), AsmState::handle_branch),
                Pattern::from(format!("{}{}", mnemonic, operand), AsmState::handle_absolute),
                Pattern::from(format!("{}{},x", mnemonic, operand), AsmState::handle_absolute_indexed_x),
                Pattern::from(format!("{}{},y", mnemonic, operand), AsmState::handle_absolute_indexed_y),
                Pattern::from(format!("{}\\({}\\)", mnemonic, operand), AsmState::handle_indirect),
                Pattern::from(format!("{}\\({},x\\)", mnemonic, operand), AsmState::handle_indexed_indirect_x),
                Pattern::from(format!("{}\\({}\\),y", mnemonic, operand), AsmState::handle_indirect_indexed_y),
            ],
        }
    }

    fn extract_group(captures: &regex::Captures, i: usize) -> Option<String> {
        captures.get(i).map_or(None, |m| Some(String::from(m.as_str())))
    }

    fn process_line(&self, state: &mut AsmState, line: String) -> AsmError {
        for pattern in self.patterns.iter() {
            match pattern.regex.captures(&line) {
                Some(captures) => {
                    state.handle_symbol(Self::extract_group(&captures, 1));
                    state.operand = Self::extract_group(&captures, 2);
                    state.operation = Self::extract_group(&captures, 3);
                    (pattern.handler)(state);
                    return AsmError::Ok;
                }
                None => {}
            }
        }
        AsmError::SyntaxError
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let asm = Assembler::new();
        assert!(asm.patterns.len() == 13);
    }

    #[test]
    fn empty_line() {
        let asm = Assembler::new();
        let mut st = AsmState::new();
        let r = asm.process_line(&mut st, String::from(""));
        assert!(matches!(r, AsmError::Ok));
        assert_eq!(st.location_counter_prev, 0);
        assert!(st.symbols.is_empty());
    }

    //    #[test]
    fn implied_mode() {
        let asm = Assembler::new();
        let mut st = AsmState::new();
        let r = asm.process_line(&mut st, String::from("SEI"));
        assert!(matches!(r, AsmError::Ok));
        assert_eq!(st.location_counter_prev, 1);
        assert!(st.symbols.is_empty());
        assert!(st.object_code.data.is_empty());
    }
}
