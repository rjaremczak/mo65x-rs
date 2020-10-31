use std::collections::HashMap;

use crate::mos6510::error::AsmError;

pub const LO_BYTE_MODIFIER: char = '<';
pub const HI_BYTE_MODIFIER: char = '>';
pub const HEX_PREFIX: char = '$';
pub const BIN_PREFIX: char = '%';

enum Modifier {
    None,
    LoByte,
    HiByte,
}

impl Modifier {
    pub fn from(str: &str) -> Modifier {
        if str.starts_with(LO_BYTE_MODIFIER) {
            Modifier::LoByte
        } else if str.starts_with(HI_BYTE_MODIFIER) {
            Modifier::HiByte
        } else {
            Modifier::None
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Modifier::None => 0,
            Modifier::LoByte => 1,
            Modifier::HiByte => 1,
        }
    }

    pub fn apply(&self, val: i32) -> i32 {
        match self {
            Modifier::None => val,
            Modifier::LoByte => val & 0x00ff,
            Modifier::HiByte => val >> 8,
        }
    }
}

pub struct OperandParser {
    symbols: HashMap<String, i32>,
}

impl OperandParser {
    pub fn new() -> Self {
        Self { symbols: HashMap::new() }
    }

    pub fn resolve(&self, txt: &str) -> Result<i32, AsmError> {
        let modifier = Modifier::from(txt);
        let rest = &txt[modifier.len()..];
        self.resolve_raw(rest).and_then(|num| Ok(modifier.apply(num)))
    }

    pub fn define_symbol(&mut self, key: &str, val: i32) {
        self.symbols.insert(String::from(key), val);
    }

    pub fn symbols(&self) -> impl Iterator<Item = (&String, &i32)> {
        self.symbols.iter()
    }

    fn resolve_raw(&self, raw: &str) -> Result<i32, AsmError> {
        match raw.chars().next() {
            Some(c) => match c {
                HEX_PREFIX => parse_int(&raw[1..], 16),
                BIN_PREFIX => parse_int(&raw[1..], 2),
                _ => {
                    if c.is_ascii_digit() || c == '+' || c == '-' {
                        parse_int(&raw, 10)
                    } else if let Some(num) = self.symbols.get(raw) {
                        Ok(*num)
                    } else {
                        Err(AsmError::SymbolNotDefined)
                    }
                }
            },
            None => Err(AsmError::MissingOperand),
        }
    }
}

fn parse_int(str: &str, radix: u32) -> Result<i32, AsmError> {
    match i32::from_str_radix(str, radix) {
        Ok(num) => Ok(num),
        Err(perr) => Err(AsmError::MalformedOperand(perr)),
    }
}

#[inline]
pub fn is_zero_page(num: i32) -> bool {
    num >= 0 && num <= 256
}

#[cfg(test)]
mod tests {
    use super::*;

    fn operand_parser() -> OperandParser {
        let mut op = OperandParser::new();
        op.define_symbol("label_1", 0x2ffe);
        op.define_symbol("label_2", 0xac02);
        op
    }

    fn assert_err(txt: &str, experr: AsmError) {
        match operand_parser().resolve(txt) {
            Ok(_) => assert!(false),
            Err(err) => assert!(matches!(err, experr)),
        }
    }

    fn assert_ok(txt: &str, val: i32) {
        match operand_parser().resolve(txt) {
            Ok(num) => assert_eq!(num, val),
            Err(_) => assert!(false, "txt: {}", txt),
        }
    }

    #[test]
    fn no_modifier() {
        let p = Modifier::from("$1230");
        assert!(matches!(p, Modifier::None));
        assert_eq!(p.len(), 0);
    }

    #[test]
    fn lo_byte_modifier() {
        let p = Modifier::from("<$1230");
        assert!(matches!(p, Modifier::LoByte));
        assert_eq!(p.len(), 1);
    }

    #[test]
    fn hi_byte_modifier() {
        let p = Modifier::from(">$1230");
        assert!(matches!(p, Modifier::HiByte));
        assert_eq!(p.len(), 1);
    }

    #[test]
    fn bin_numbers() {
        assert_ok("%10000011", 131);
        assert_ok("<%0011110010100101", 0b10100101);
        assert_ok(">%1100000000000110", 0b11000000);
    }

    #[test]
    fn dec_numbers() {
        assert_ok("65000", 65000);
        assert_ok("-201", -201);
        assert_ok("<32769", 0x01);
        assert_ok(">32769", 0x80);
    }

    #[test]
    fn hex_numbers() {
        assert_ok("$-100", -256);
        assert_ok("$1f", 31);
        assert_ok("<$10ac", 0xac);
        assert_ok(">$10ac", 0x10);
    }

    #[test]
    fn define_symbols() {
        assert_ok("label_1", 0x2ffe);
        assert_ok("label_2", 0xac02);
        assert_ok("<label_1", 0xfe);
        assert_ok(">label_1", 0x2f);
        assert_err("labeloza", AsmError::SymbolNotDefined);
    }

    fn list() {
        // assert_ok!("")
    }
}
