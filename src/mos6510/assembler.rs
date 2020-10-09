extern crate regex;

mod assembly;

use super::Memory;
use assembly::*;
use regex::Regex;
use std::collections::HashMap;

const LABEL_GROUP: usize = 1;
const OPERATION_GROUP: usize = 2;
const FIRST_OPERAND_GROUP: usize = 3;

type PatternHandler = fn(&mut AsmState) -> AsmResult;

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
        let hex_prefix: char = '$';
        let bin_prefix: char = '%';
        let lo_byte_prefix: char = '<';
        let hi_byte_prefix: char = '>';
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
        let lo_hi_prefix = format!("[{}|{}]?", lo_byte_prefix, hi_byte_prefix);
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

    fn process_line(&self, state: &mut AsmState, memory: &mut Memory, line: String) -> AsmResult {
        for pattern in self.patterns.iter() {
            match pattern.regex.captures(&line) {
                Some(captures) => {
                    state.label = Self::extract_group(&captures, LABEL_GROUP);
                    state.operand = Self::extract_group(&captures, OPERATION_GROUP);
                    state.operation = Self::extract_group(&captures, FIRST_OPERAND_GROUP);
                    (pattern.handler)(state);
                    return AsmResult::Ok;
                }
                None => {}
            }
        }
        AsmResult::SyntaxError
    }

    // pub fn scan(lines) -> AsmState {}
    // pub fn code(lines) -> AsmState {}
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
        let mut mem = Memory::new();
        let mut st = AsmState::new();
        let r = asm.process_line(&mut st, &mut mem, String::from(""));
        assert!(matches!(r, AsmResult::Ok));
        assert_eq!(st.location_counter_prev, 0);
        assert!(st.symbols.is_empty());
    }

    #[test]
    fn implied_mode() {
        let asm = Assembler::new();
        let mut mem = Memory::new();
        let mut st = AsmState::new();
        let r = asm.process_line(&mut st, &mut mem, String::from("SEI"));
        assert!(matches!(r, AsmResult::Ok));
        assert_eq!(st.location_counter_prev, 1);
        assert!(st.symbols.is_empty());
        assert!(mem.byte(0x0000) == 0);
    }
}
