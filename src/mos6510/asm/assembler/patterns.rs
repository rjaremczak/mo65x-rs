use super::operand::{HI_BYTE_MODIFIER, LO_BYTE_MODIFIER};
use regex::Regex;

const SYMBOL: &str = "[a-z]\\w*";
const LABEL: &str = "^(?:([a-z]\\w*):)?\\s*";
const COMMENT: &str = "(?:;.*)?$";

pub struct AsmPatterns {
    pub empty_line: Regex,
    pub cmd_set_location_counter: Regex,
    pub cmd_emit_bytes: Regex,
    pub cmd_emit_words: Regex,
    pub ins_implied: Regex,
    pub ins_immediate: Regex,
    pub ins_branch: Regex,
    pub ins_absolute: Regex,
    pub ins_absolute_indexed_x: Regex,
    pub ins_absolute_indexed_y: Regex,
    pub ins_indirect: Regex,
    pub ins_indexed_indirect_x: Regex,
    pub ins_indirect_indexed_y: Regex,
}

fn rx(pattern: &str) -> Regex {
    Regex::new(&format!("{}{}{}", LABEL, pattern, COMMENT)).unwrap()
}

impl AsmPatterns {
    pub fn new() -> AsmPatterns {
        let org_cmd = String::from("((?:\\.ORG\\s+)|(?:\\*\\s*=\\s*))");
        let byte_cmd = String::from("(\\.BYTE|DCB)\\s+");
        let word_cmd = String::from("(\\.WORD)\\s+");
        let hex_num = String::from("\\$[\\d|a-h]{1,4}");
        let dec_num = String::from("\\d{1,5}");
        let bin_num = String::from("%[01]{1,16}");
        let mnemonic = String::from("([a-zA-Z]{3})\\s*");
        let num_or_symbol = format!("(?:{})|(?:{})|(?:{})|(?:{})", hex_num, dec_num, bin_num, SYMBOL);
        let lo_hi_prefix = format!("[{}|{}]?", LO_BYTE_MODIFIER, HI_BYTE_MODIFIER);
        let operand = format!("({}(?:{}))\\s*", lo_hi_prefix, num_or_symbol);
        let operand_separator = String::from("\\s*,?\\s*");
        let operand_list = format!("((?:(?:{}(?:{})){})+)\\s*", lo_hi_prefix, num_or_symbol, operand_separator);
        let branch_mnemonic = String::from("(BCC|BCS|BNE|BEQ|BMI|BPL|BVC|BVS)\\s*");
        let branch_target = format!("((?:[+|-]?\\d{{1,3}})|(?:{}))\\s*", SYMBOL);
        AsmPatterns {
            empty_line: rx(""),
            cmd_set_location_counter: rx(&format!("{}{}", org_cmd, operand)),
            cmd_emit_bytes: rx(&format!("{}{}", byte_cmd, operand_list)),
            cmd_emit_words: rx(&format!("{}{}", word_cmd, operand_list)),
            ins_implied: rx(&format!("{}", mnemonic)),
            ins_immediate: rx(&format!("{}#{}", mnemonic, operand)),
            ins_branch: rx(&format!("{}{}", branch_mnemonic, branch_target)),
            ins_absolute: rx(&format!("{}{}", mnemonic, operand)),
            ins_absolute_indexed_x: rx(&format!("{}{},x", mnemonic, operand)),
            ins_absolute_indexed_y: rx(&format!("{}{},y", mnemonic, operand)),
            ins_indirect: rx(&format!("{}\\({}\\)", mnemonic, operand)),
            ins_indexed_indirect_x: rx(&format!("{}\\({},x\\)", mnemonic, operand)),
            ins_indirect_indexed_y: rx(&format!("{}\\({}\\),y", mnemonic, operand)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_line(regex: &Regex, line: &str, label: Option<&str>, operation: Option<&str>, operand: Option<&str>) {
        match regex.captures(line) {
            Some(caps) => {
                assert_eq!(caps.get(1).map(|m| m.as_str()).as_deref(), label.as_deref());
                assert_eq!(caps.get(2).map(|m| m.as_str()).as_deref(), operation.as_deref());
                assert_eq!(caps.get(3).map(|m| m.as_str()).as_deref(), operand.as_deref());
            }
            None => assert!(false, "matching failed"),
        }
    }

    #[test]
    fn match_label() {
        assert_line(&AsmPatterns::new().empty_line, "label:", Some("label"), None, None);
    }

    #[test]
    fn match_implied() {
        let ap = AsmPatterns::new();
        assert_line(&ap.ins_implied, "    inc", None, Some("inc"), None);
        assert_line(&ap.ins_implied, "SEI", None, Some("SEI"), None);
    }
}
