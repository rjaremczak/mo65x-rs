mod error;
mod object_code;
mod operand;
mod parser;
mod processor;

use error::*;
use parser::*;
use processor::*;

pub struct Assembler {
    parser: Parser,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler { parser: Parser::new() }
    }

    fn process_line(&self, ap: &mut CodeGeneration, line: &str) -> AsmError {
        match self.parser.parse_line(line) {
            Ok(pl) => {
                ap.handle_symbol(pl.symbol);
                ap.instruction = pl.operation;
                ap.operand = pl.operand;
                (pl.handler)(ap)
            }
            Err(err) => err,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_line() {
        let asm = Assembler::new();
        let mut ap = CodeGeneration::new();
        let r = asm.process_line(&mut ap, "");
        assert!(matches!(r, AsmError::Ok));
        assert_eq!(ap.location_counter_prev, 0);
        assert!(ap.symbols.is_empty());
    }

    #[test]
    fn implied_mode() {
        let asm = Assembler::new();
        let mut ap = CodeGeneration::new();
        let r = asm.process_line(&mut ap, "SEI");
        assert!(matches!(r, AsmError::Ok));
        assert_eq!(ap.location_counter_prev, 1);
        assert!(ap.symbols.is_empty());
        assert!(ap.object_code.data.is_empty());
    }
}
