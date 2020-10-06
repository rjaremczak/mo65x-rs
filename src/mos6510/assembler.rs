extern crate regex;

use regex::Regex;

const LABEL_GROUP: usize = 1;
const OPERATION_GROUP: usize = 2;
const FIRST_OPERAND_GROUP: usize = 3;

struct Pattern {
    regex: Regex,
    handler: fn(&mut Assembler),
}

impl Pattern {
    pub fn new(pattern: &str, handler: fn(&mut Assembler)) -> Pattern {
        Pattern {
            regex: Regex::new(pattern).unwrap(),
            handler,
        }
    }
    pub fn from(pattern: String, handler: fn(&mut Assembler)) -> Pattern {
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
        let org_cmd = String::from("((?:\\.ORG\\s+)|(?:\\*\\s*\\=\\s*))");
        let byte_cmd = String::from("(\\.BYTE|DCB)\\s+");
        let word_cmd = String::from("(\\.WORD)\\s+");
        let hex_num = String::from("\\$[\\d|a-h]{1,4}");
        let dec_num = String::from("\\d{1,5}");
        let bin_num = String::from("\\%[01]{1,16}");
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
                Pattern::new("", Assembler::handle_no_operation),
                Pattern::from(format!("{}{}", org_cmd, operand), Assembler::handle_set_location_counter),
                Pattern::from(format!("{}{}", byte_cmd, operand_list), Assembler::handle_emit_bytes),
                Pattern::from(format!("{}{}", word_cmd, operand_list), Assembler::handle_emit_words),
                Pattern::from(format!("{}", mnemonic), Assembler::handle_implied),
                Pattern::from(format!("{}#{}", mnemonic, operand), Assembler::handle_immediate),
                Pattern::from(format!("{}{}", branch_mnemonic, branch_target), Assembler::handle_branch),
                Pattern::from(format!("{}{}", mnemonic, operand), Assembler::handle_absolute),
                Pattern::from(format!("{}{},x", mnemonic, operand), Assembler::handle_absolute_indexed_x),
                Pattern::from(format!("{}{},y", mnemonic, operand), Assembler::handle_absolute_indexed_y),
                Pattern::from(format!("{}\\({}\\)", mnemonic, operand), Assembler::handle_indirect),
                Pattern::from(format!("{}\\({},x\\)", mnemonic, operand), Assembler::handle_indexed_indirect_x),
                Pattern::from(format!("{}\\({}\\),y", mnemonic, operand), Assembler::handle_indirect_indexed_y),
            ],
        }
    }

    fn handle_no_operation(&mut self) {}
    fn handle_set_location_counter(&mut self) {}
    fn handle_emit_bytes(&mut self) {}
    fn handle_emit_words(&mut self) {}
    fn handle_implied(&mut self) {}
    fn handle_immediate(&mut self) {}
    fn handle_branch(&mut self) {}
    fn handle_absolute(&mut self) {}
    fn handle_absolute_indexed_x(&mut self) {}
    fn handle_absolute_indexed_y(&mut self) {}
    fn handle_indirect(&mut self) {}
    fn handle_indexed_indirect_x(&mut self) {}
    fn handle_indirect_indexed_y(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let mut asm: Assembler;
    }
}