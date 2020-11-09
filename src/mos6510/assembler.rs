mod code;
mod operand;
mod patterns;
mod tokens;

use super::{addr, addrmode::*, error::AsmError, instruction::find_instruction, opcode::find_opcode};
use code::ObjectCode;
use operand::OperandParser;
use regex::Regex;
use tokens::Tokens;

type Handler = fn(&mut Assembler, tokens: Tokens) -> AsmError;

pub struct Assembler {
    handlers: Vec<(Regex, Handler)>,
    operand_parser: OperandParser,
    object_code: ObjectCode,
    op_list_separator: Regex,
}

impl Assembler {
    pub fn new(origin: u16) -> Assembler {
        Assembler {
            operand_parser: OperandParser::new(),
            object_code: ObjectCode::new(origin),
            op_list_separator: Regex::new("(?:\\s*,\\s*)|(?:\\s+)").unwrap(),
            handlers: {
                let p = patterns::AsmPatterns::new();
                vec![
                    (p.empty_line, Assembler::handle_empty_line),
                    (p.cmd_set_location_counter, Assembler::handle_set_location_counter),
                    (p.cmd_emit_bytes, Assembler::handle_emit_bytes),
                    (p.cmd_emit_words, Assembler::handle_emit_words),
                    (p.ins_implied, Assembler::handle_implied),
                    (p.ins_immediate, Assembler::handle_immediate),
                    (p.ins_branch, Assembler::handle_branch),
                    (p.ins_absolute, Assembler::handle_absolute),
                    (p.ins_absolute_indexed_x, Assembler::handle_absolute_indexed_x),
                    (p.ins_absolute_indexed_y, Assembler::handle_absolute_indexed_y),
                    (p.ins_indirect, Assembler::handle_indirect),
                    (p.ins_indexed_indirect_x, Assembler::handle_indexed_indirect_x),
                    (p.ins_indirect_indexed_y, Assembler::handle_indirect_indexed_y),
                ]
            },
        }
    }

    pub fn process_line(&mut self, line: &str) -> AsmError {
        for (regex, handler) in self.handlers.iter() {
            if let Some(captures) = regex.captures(&line) {
                let tokens = Tokens::new(captures);
                if let Some(label) = tokens.label() {
                    self.operand_parser.define_symbol(label, self.object_code.location_counter as i32);
                };
                return handler(self, tokens);
            }
        }
        AsmError::SyntaxError
    }

    fn preprocess<'a>(addrmode: &'a AddrMode, opt_opval: Option<i32>) -> (&'a AddrMode<'a>, i32) {
        match addrmode.zp_mode {
            Some(zp_mode) => match opt_opval {
                Some(opval) => match addr::is_zero_page(opval) {
                    true => (zp_mode, opval),
                    false => (addrmode, opval),
                },
                None => (addrmode, 0),
            },
            None => (addrmode, opt_opval.unwrap_or(0)),
        }
    }

    fn parse_operand_list(&self, oplist: Option<&str>) -> Result<Vec<i32>, AsmError> {
        match oplist {
            Some(oplist) => {
                let mut values: Vec<i32> = Vec::new();
                for opstr in self.op_list_separator.split(oplist) {
                    match self.operand_parser.resolve(opstr) {
                        Ok(opval) => values.push(opval),
                        Err(err) => return Err(err),
                    }
                }
                Ok(values)
            }
            None => Err(AsmError::MissingOperand),
        }
    }

    fn assemble(&mut self, addrmode: &AddrMode, tokens: Tokens) -> AsmError {
        let operand = if addrmode == &IMPLIED {
            None
        } else {
            match tokens.operand() {
                Some(opstr) => match self.operand_parser.resolve(opstr) {
                    Ok(opval) => Some(opval),
                    Err(err) => return err,
                },
                None => return AsmError::MissingOperand,
            }
        };
        let (opt_addrmode, opvalue) = Self::preprocess(addrmode, operand);
        match tokens.operation() {
            Some(operation) => match find_instruction(operation) {
                Some(instruction) => match find_opcode(instruction, opt_addrmode) {
                    Some(opcode) => {
                        self.object_code.emit_byte(opcode.code);
                        if opcode.size == 2 {
                            self.object_code.emit_byte(opvalue as u8);
                        } else if opcode.size == 3 {
                            self.object_code.emit_word(opvalue as u16);
                        }
                        AsmError::Ok
                    }
                    None => AsmError::InvalidInstructionFormat,
                },
                None => AsmError::InvalidMnemonic,
            },
            None => AsmError::SyntaxError,
        }
    }

    pub fn generate_code(&mut self, on: bool) {
        self.object_code.write_enabled = on;
    }

    pub fn handle_empty_line(&mut self, _: Tokens) -> AsmError {
        AsmError::Ok
    }

    pub fn handle_set_location_counter(&mut self, tokens: Tokens) -> AsmError {
        match tokens.operand() {
            Some(opstr) => match self.operand_parser.resolve(opstr) {
                Ok(val) => self.object_code.set_location_counter(val as u16),
                Err(err) => err,
            },
            None => AsmError::MissingOperand,
        }
    }

    pub fn handle_emit_bytes(&mut self, tokens: Tokens) -> AsmError {
        match self.parse_operand_list(tokens.operand()) {
            Ok(values) => {
                values.iter().for_each(|v| self.object_code.emit_byte(*v as u8));
                AsmError::Ok
            }
            Err(err) => err,
        }
    }

    pub fn handle_emit_words(&mut self, tokens: Tokens) -> AsmError {
        match self.parse_operand_list(tokens.operand()) {
            Ok(values) => {
                values.iter().for_each(|v| self.object_code.emit_word(*v as u16));
                AsmError::Ok
            }
            Err(err) => err,
        }
    }

    pub fn handle_implied(&mut self, tokens: Tokens) -> AsmError {
        self.assemble(&IMPLIED, tokens)
    }

    pub fn handle_immediate(&mut self, tokens: Tokens) -> AsmError {
        self.assemble(&IMMEDIATE, tokens)
    }

    pub fn handle_branch(&mut self, tokens: Tokens) -> AsmError {
        self.assemble(&BRANCH, tokens)
    }

    pub fn handle_absolute(&mut self, tokens: Tokens) -> AsmError {
        self.assemble(&ABSOLUTE, tokens)
    }

    pub fn handle_absolute_indexed_x(&mut self, tokens: Tokens) -> AsmError {
        self.assemble(&ABSOLUTE_X, tokens)
    }

    pub fn handle_absolute_indexed_y(&mut self, tokens: Tokens) -> AsmError {
        self.assemble(&ABSOLUTE_Y, tokens)
    }

    pub fn handle_indirect(&mut self, tokens: Tokens) -> AsmError {
        self.assemble(&INDIRECT, tokens)
    }

    pub fn handle_indexed_indirect_x(&mut self, tokens: Tokens) -> AsmError {
        self.assemble(&INDEXED_INDIRECT_X, tokens)
    }

    pub fn handle_indirect_indexed_y(&mut self, tokens: Tokens) -> AsmError {
        self.assemble(&INDIRECT_INDEXED_Y, tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_next(asm: &mut Assembler, line: &str, expected: &[u8]) {
        let r = asm.process_line(line);
        assert!(matches!(r, AsmError::Ok), "line \"{}\" : {:?}", line, r);
        assert!(asm.object_code.data.len() >= expected.len(), "line \"{}\" : code too short", line);
        let generated = &asm.object_code.data[(asm.object_code.data.len() - expected.len())..];
        assert_eq!(generated, expected, "generated code {:?} differs from {:?}", generated, expected);
    }

    fn assert_asm<'a>(line: &str, code: &[u8]) -> Assembler {
        let mut asm = Assembler::new(0);
        asm.generate_code(true);
        assert_next(&mut asm, line, code);
        asm
    }

    #[test]
    fn init() {
        let asm = Assembler::new(0);
        assert_eq!(asm.object_code.write_enabled, false);
        assert_eq!(asm.object_code.location_counter, 0);
        assert_eq!(asm.operand_parser.symbols().count(), 0);
    }

    #[test]
    fn test_list_separator() {
        let asm = Assembler::new(0);
        let sl: Vec<&str> = asm.op_list_separator.split("20 30 40").collect();
        assert_eq!(sl.as_slice(), &["20", "30", "40"]);
        let sl: Vec<&str> = asm.op_list_separator.split("18").collect();
        assert_eq!(sl.as_slice(), &["18"]);
        let sl: Vec<&str> = asm.op_list_separator.split("120, 0x30 40, 023").collect();
        assert_eq!(sl.as_slice(), &["120", "0x30", "40", "023"]);
    }

    #[test]
    fn empty_line() {
        assert_asm("", &[]);
    }

    #[test]
    fn implied_mode() {
        assert_asm("SEI", &[0x78]);
        assert_asm("ASL", &[0x0a]);
    }

    #[test]
    fn immediate_mode() {
        assert_asm("LDA #%00110101", &[0xa9, 0b00110101]);
        assert_asm("LDX #123", &[0xa2, 123]);
        assert_asm("LDY #255", &[0xa0, 0xff]);
    }

    #[test]
    fn zero_page_mode() {
        assert_asm("LDY $8f", &[0xa4, 0x8f]);
    }

    #[test]
    fn zero_page_x_mode() {
        assert_asm("LDA $a0,X", &[0xb5, 0xa0]);
    }

    #[test]
    fn zero_page_y_mode() {
        assert_asm("STX $7a,Y", &[0x96, 0x7a]);
    }

    #[test]
    fn absolute_mode() {
        let mut asm = assert_asm("ROR $3400", &[0x6e, 0x00, 0x34]);
        assert_next(&mut asm, "jmp $2000", &[0x4c, 0x00, 0x20]);
        asm.operand_parser.define_symbol("c", 0xfab0);
        assert_next(&mut asm, "jmp c", &[0x4c, 0xb0, 0xfa]);
    }

    #[test]
    fn absolute_mode_x() {
        assert_asm("LSR $35f0,X", &[0x5e, 0xf0, 0x35]);
    }

    #[test]
    fn absolute_mode_y() {
        assert_asm("EOR $f7a0,Y", &[0x59, 0xa0, 0xf7]);
    }

    #[test]
    fn indirect_mode() {
        assert_asm("JMP ($ffa0)", &[0x6c, 0xa0, 0xff]);
    }

    #[test]
    fn indirect_mode_x() {
        assert_asm("LDA ($8c,X)", &[0xa1, 0x8c]);
    }

    #[test]
    fn indirect_indexed_y() {
        assert_asm("ORA ($a7),Y", &[0x11, 0xa7]);
    }

    #[test]
    fn relative_mode() {
        assert_asm("BCC -1", &[0x90, u8::from_ne_bytes((-1 as i8).to_ne_bytes())]);
        assert_asm("BVS +8", &[0x70, 8]);
    }

    #[test]
    fn set_location_counter() {
        let mut asm = assert_asm("  .ORG $3000 ;origin", &[]);
        assert_eq!(asm.object_code.location_counter, 0x3000);
        assert_next(&mut asm, "  .ORG $4000 ;origin", &[]);
        assert_eq!(asm.object_code.location_counter, 0x4000);
        assert_eq!(asm.object_code.data.len(), 0x4000);
        assert_next(&mut asm, "  *= $5000 ;origin", &[]);
        assert_eq!(asm.object_code.location_counter, 0x5000);
    }

    #[test]
    fn test_comments() {
        assert_asm("lda ($8c,X)  ;komentarz", &[0xa1, 0x8c]);
        assert_asm("  ;  komentarz", &[]);
        assert_asm("label: ;komentarz numer 2", &[]);
        assert_asm("LSR $35f0,X ;comment", &[0x5e, 0xf0, 0x35]);
    }

    #[test]
    fn test_label() {
        let mut asm = assert_asm("Label_001:", &[]);
        assert_next(&mut asm, "LDA ($8c,X)", &[0xa1, 0x8c]);
        assert_next(&mut asm, "Label_002:", &[]);
        assert_eq!(asm.operand_parser.get_symbol("Label_001").unwrap(), 0);
        assert_eq!(asm.operand_parser.get_symbol("Label_002").unwrap(), 2);
    }

    #[test]
    fn test_symbols() {
        let mut asm = Assembler::new(1000);
        asm.generate_code(true);
        asm.operand_parser.define_symbol("dziabaDucha", 0xaf02);
        assert_next(&mut asm, "TestLabel_01:  SEI   ; disable interrupts ", &[0x78]);
        assert_next(&mut asm, "c:lda dziabaDucha", &[0xad, 0x02, 0xaf]);
        assert_eq!(asm.operand_parser.get_symbol("TestLabel_01").unwrap(), 1000);
        assert_eq!(asm.operand_parser.get_symbol("TestLabel_02"), None);
        assert_eq!(asm.object_code.data.len(), 4);
        assert_eq!(asm.object_code.location_counter, 1004);
    }

    #[test]
    fn emit_bytes() {
        assert_asm(".BYTE 20", &[20]);
        assert_asm(".BYTE $20 45 $4a", &[0x20, 45, 0x4a]);
        assert_asm(".BYTE $20, $3f,$4a ,$23 , 123", &[0x20, 0x3f, 0x4a, 0x23, 123]);
        assert_asm(
            "dcb 0,0,0,0,0,0,0,0,0,$b,$b,$c,$f,$f,$f,$f",
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0xb, 0xb, 0xc, 0xf, 0xf, 0xf, 0xf],
        );
    }

    #[test]
    fn emit_words() {
        assert_asm(".word $20ff $23af $fab0 ;komm", &[0xff, 0x20, 0xaf, 0x23, 0xb0, 0xfa]);
        assert_asm(".word $3000 $15ad 1024", &[0x00, 0x30, 0xad, 0x15, 0x00, 0x04]);
    }

    #[test]
    fn lo_byte_modifier() {
        let mut asm = assert_asm("LDA #<$1afc", &[0xa9, 0xfc]);
        asm.operand_parser.define_symbol("label", 0x2afe);
        assert_next(&mut asm, "LDA #<label", &[0xa9, 0xfe]);
        assert_next(&mut asm, "dcb <label, 2", &[0xfe, 2]);
    }

    #[test]
    fn hi_byte_modifier() {
        let mut asm = assert_asm("LDA #>$1afc", &[0xa9, 0x1a]);
        asm.operand_parser.define_symbol("label", 0x3afe);
        assert_next(&mut asm, "LDA #>label", &[0xa9, 0x3a]);
        assert_next(&mut asm, "dcb >label, 2", &[0x3a, 2]);
        asm.operand_parser.define_symbol("a", 0xfa20);
        asm.operand_parser.define_symbol("b", 0x10a0);
        assert_next(&mut asm, "dcb >a, <b", &[0xfa, 0xa0]);
        assert_next(&mut asm, "dcb <a, >b", &[0x20, 0x10]);
    }

    #[test]
    fn define_symbol() {
        let mut asm = assert_asm(".org $1000", &[]);
        asm.operand_parser.define_symbol("init", 0x1234);
        assert_next(&mut asm, "lda init", &[0xad, 0x34, 0x12]);
        assert_eq!(asm.object_code.location_counter, 0x1003);
    }
}
