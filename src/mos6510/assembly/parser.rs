use super::error::*;
use super::operand::{HI_BYTE_MODIFIER, LO_BYTE_MODIFIER};
use super::processor::*;
use regex::Regex;

type PatternHandler = fn(&mut AsmProcessor) -> AsmError;

pub struct Pattern {
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
}

pub fn create_patterns() -> Vec<Pattern> {
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
    vec![
        Pattern::new("", AsmProcessor::handle_empty_line),
        Pattern::new(&format!("{}{}", org_cmd, operand), AsmProcessor::handle_set_location_counter),
        Pattern::new(&format!("{}{}", byte_cmd, operand_list), AsmProcessor::handle_emit_bytes),
        Pattern::new(&format!("{}{}", word_cmd, operand_list), AsmProcessor::handle_emit_words),
        Pattern::new(&format!("{}", mnemonic), AsmProcessor::handle_implied),
        Pattern::new(&format!("{}#{}", mnemonic, operand), AsmProcessor::handle_immediate),
        Pattern::new(&format!("{}{}", branch_mnemonic, branch_target), AsmProcessor::handle_branch),
        Pattern::new(&format!("{}{}", mnemonic, operand), AsmProcessor::handle_absolute),
        Pattern::new(&format!("{}{},x", mnemonic, operand), AsmProcessor::handle_absolute_indexed_x),
        Pattern::new(&format!("{}{},y", mnemonic, operand), AsmProcessor::handle_absolute_indexed_y),
        Pattern::new(&format!("{}\\({}\\)", mnemonic, operand), AsmProcessor::handle_indirect),
        Pattern::new(&format!("{}\\({},x\\)", mnemonic, operand), AsmProcessor::handle_indexed_indirect_x),
        Pattern::new(&format!("{}\\({}\\),y", mnemonic, operand), AsmProcessor::handle_indirect_indexed_y),
    ]
}

pub struct Parser {
    patterns: Vec<Pattern>,
}

pub struct ParsedLine {
    pub symbol: Option<String>,
    pub operation: Option<String>,
    pub operand: Option<String>,
    pub handler: PatternHandler,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            patterns: create_patterns(),
        }
    }

    fn extract_group(captures: &regex::Captures, i: usize) -> Option<String> {
        captures.get(i).map_or(None, |m| Some(String::from(m.as_str())))
    }

    pub fn parse_line(&self, line: &str) -> Result<ParsedLine, AsmError> {
        for pattern in self.patterns.iter() {
            if let Some(captures) = pattern.regex.captures(&line) {
                return Ok(ParsedLine {
                    symbol: Self::extract_group(&captures, 1),
                    operation: Self::extract_group(&captures, 2),
                    operand: Self::extract_group(&captures, 3),
                    handler: pattern.handler,
                });
            }
        }
        Err(AsmError::SyntaxError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let parser = Parser::new();
        assert!(parser.patterns.len() == 13);
    }
}
