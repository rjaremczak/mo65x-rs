use super::error::AsmError;

pub const LO_BYTE_MODIFIER: char = '<';
pub const HI_BYTE_MODIFIER: char = '>';
pub const HEX_PREFIX: char = '$';
pub const BIN_PREFIX: char = '%';

pub type SymbolResolver = fn(symbol: &str) -> Option<i32>;

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

fn parse_int(str: &str, radix: u32) -> Result<i32, AsmError> {
    match i32::from_str_radix(str, radix) {
        Ok(num) => Ok(num),
        Err(perr) => Err(AsmError::MalformedOperand(perr)),
    }
}

fn resolve_raw(raw: &str, symbol_resolver: SymbolResolver) -> Result<i32, AsmError> {
    match raw.chars().next() {
        Some(c) => match c {
            HEX_PREFIX => parse_int(&raw[1..], 16),
            BIN_PREFIX => parse_int(&raw[1..], 2),
            _ => {
                if c.is_ascii_digit() || c == '+' || c == '-' {
                    parse_int(&raw, 10)
                } else if let Some(num) = symbol_resolver(raw) {
                    Ok(num)
                } else {
                    Err(AsmError::SymbolNotDefined)
                }
            }
        },
        None => Err(AsmError::MissingOperand),
    }
}

pub fn resolve_operand(opsrc: Option<&str>, symbol_resolver: SymbolResolver) -> Result<i32, AsmError> {
    match opsrc {
        Some(src) => {
            let modifier = Modifier::from(src);
            let rest = &src[modifier.len()..];
            match resolve_raw(rest, symbol_resolver) {
                Ok(num) => Ok(modifier.apply(num)),
                Err(err) => Err(err),
            }
        }
        None => Err(AsmError::MissingOperand),
    }
}

#[inline]
pub fn is_zero_page_operand(num: i32) -> bool {
    num >= 0 && num <= 256
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn test_resolver(s: &str) -> Option<i32> {
        match s {
            "label_1" => Some(0x2ffe),
            "label_2" => Some(0xac02),
            _ => None,
        }
    }

    fn assert_ok(s: &str, val: i32) {
        if let Ok(num) = resolve_operand(s, test_resolver) {
            assert_eq!(num, val);
        } else {
            assert!(false, "str: {}", s);
        }
    }

    fn assert_err(s: &str, exp: AsmError) {
        if let Err(err) = resolve_operand(s, test_resolver) {
            assert!(matches!(err, exp));
        } else {
            assert!(false);
        }
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
        assert_ok("$01f", 31);
        assert_ok("<$10ac", 0xac);
        assert_ok(">$10ac", 0x10);
    }

    #[test]
    fn symbols() {
        assert_ok("label_1", 0x2ffe);
        assert_ok("label_2", 0xac02);
        assert_ok("<label_1", 0xfe);
        assert_ok(">label_1", 0x2f);
        assert_err("labeloza", AsmError::SymbolNotDefined);
    }
}
