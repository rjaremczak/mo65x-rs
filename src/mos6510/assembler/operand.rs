use crate::mos6510::error::AppError;
use std::collections::HashMap;

pub const LO_BYTE_MODIFIER: char = '<';
pub const HI_BYTE_MODIFIER: char = '>';
pub const HEX_PREFIX: char = '$';
pub const BIN_PREFIX: char = '%';

#[derive(Copy, Clone)]
pub struct Operand {
    pub value: i32,
    pub is_symbol: bool,
}

impl Operand {
    pub fn literal(value: i32) -> Self {
        Self { value, is_symbol: false }
    }
    pub fn symbol(value: i32) -> Self {
        Self { value, is_symbol: true }
    }
    pub fn modified(&self, modifier: Modifier) -> Self {
        Self {
            value: match modifier {
                Modifier::None => self.value,
                Modifier::LoByte => self.value & 0x00ff,
                Modifier::HiByte => self.value >> 8,
            },
            is_symbol: self.is_symbol,
        }
    }
}

pub enum Modifier {
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
}

pub struct Resolver {
    symbols: HashMap<String, i32>,
}

impl Resolver {
    pub fn new() -> Self {
        Self { symbols: HashMap::new() }
    }

    pub fn resolve(&self, txt: &str, no_symbol_fail: bool) -> Result<Operand, AppError> {
        let modifier = Modifier::from(txt);
        let rest = &txt[modifier.len()..];
        self.resolve_raw(rest, no_symbol_fail).and_then(|op| Ok(op.modified(modifier)))
    }

    pub fn define_symbol(&mut self, key: &str, val: i32) -> Result<(), AppError> {
        match self.symbols.insert(String::from(key), val) {
            Some(old) => {
                if old != val {
                    Err(AppError::GeneralError(format!(
                        "symbol {} redefinition, was {} changed to {}",
                        key, old, val
                    )))
                } else {
                    Ok(())
                }
            }
            None => Ok(()),
        }
    }

    pub fn symbols(&self) -> &HashMap<String, i32> {
        &self.symbols
    }

    fn resolve_raw(&self, raw: &str, no_symbol_fail: bool) -> Result<Operand, AppError> {
        match raw.chars().next() {
            Some(c) => match c {
                HEX_PREFIX => parse_int(&raw[1..], 16),
                BIN_PREFIX => parse_int(&raw[1..], 2),
                _ => {
                    if c.is_ascii_digit() || c == '+' || c == '-' {
                        parse_int(&raw, 10)
                    } else if let Some(num) = self.symbols.get(raw) {
                        Ok(Operand::symbol(*num))
                    } else if no_symbol_fail {
                        Err(AppError::UndefinedSymbol(raw.to_string()))
                    } else {
                        Ok(Operand::symbol(0))
                    }
                }
            },
            None => Err(AppError::MissingOperand),
        }
    }
}

fn parse_int(str: &str, radix: u32) -> Result<Operand, AppError> {
    match i32::from_str_radix(str, radix) {
        Ok(num) => Ok(Operand::literal(num)),
        Err(perr) => Err(AppError::ParseIntError(String::from(str), perr)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn operand_parser() -> Resolver {
        let mut op = Resolver::new();
        op.define_symbol("label_1", 0x2ffe);
        op.define_symbol("label_2", 0xac02);
        op
    }

    fn assert_err(txt: &str, experr: AppError) {
        match operand_parser().resolve(txt, true) {
            Ok(_) => assert!(false),
            Err(err) => assert!(matches!(err, experr)),
        }
    }

    fn assert_ok(txt: &str, val: i32) {
        match operand_parser().resolve(txt, true) {
            Ok(operand) => assert_eq!(operand.value, val),
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
        assert_err("labeloza", AppError::UndefinedSymbol(String::from("labeloza")));
    }
}
