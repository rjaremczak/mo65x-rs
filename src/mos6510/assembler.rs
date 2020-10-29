mod code;
mod error;
mod operand;
mod patterns;
mod tokens;

use super::instruction::find_instruction;
use super::{addrmode::AddrMode, opcode::find_opcode};
use code::ObjectCode;
use error::AsmError;
use operand::{is_zero_page_operand, resolve_operand};
use regex::Regex;
use std::collections::HashMap;
use tokens::Tokens;

type Symbols = HashMap<String, i32>;
type Handler = fn(&mut Assembler, tokens: Tokens) -> AsmError;

pub struct Assembler {
    handlers: Vec<(Regex, Handler)>,
    symbols: Symbols,
    object_code: ObjectCode,
    op_list_separator: Regex,
}

impl Assembler {
    pub fn new(origin: u16) -> Assembler {
        Assembler {
            symbols: Symbols::new(),
            object_code: ObjectCode::new(origin),
            op_list_separator: Regex::new(&format!("(?i){}", patterns::SEPARATOR)).unwrap(),
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
                    self.symbols.insert(String::from(label), self.object_code.location_counter as i32);
                };
                return handler(self, tokens);
            }
        }
        AsmError::SyntaxError
    }

    fn preprocess(addrmode: AddrMode, opt_opval: Option<i32>) -> (AddrMode, i32) {
        match addrmode.zero_page_variant() {
            Some(zp_mode) => match opt_opval {
                Some(opval) => match is_zero_page_operand(opval) {
                    true => (zp_mode, opval),
                    false => (addrmode, opval),
                },
                None => (addrmode, 0),
            },
            None => (addrmode, opt_opval.unwrap_or(0)),
        }
    }

    fn parse_operand(&self, opstr: Option<&str>) -> Result<i32, AsmError> {
        resolve_operand(opstr, |s| self.symbols.get(s).map(|v| *v))
    }

    fn parse_operand_list(&self, oplist: Option<&str>) -> Result<Vec<i32>, AsmError> {
        match oplist {
            Some(oplist) => {
                let mut values: Vec<i32> = Vec::new();
                for opstr in self.op_list_separator.split(oplist) {
                    match self.parse_operand(Some(opstr)) {
                        Ok(opval) => values.push(opval),
                        Err(err) => return Err(err),
                    }
                }
                Ok(values)
            }
            None => Err(AsmError::MissingOperand),
        }
    }

    fn assemble(&mut self, addrmode: AddrMode, tokens: Tokens) -> AsmError {
        let operand = match addrmode {
            AddrMode::Implied => None,
            _ => match self.parse_operand(tokens.operand()) {
                Ok(opvalue) => Some(opvalue),
                Err(err) => return err,
            },
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

    pub fn handle_empty_line(&mut self, tokens: Tokens) -> AsmError {
        AsmError::Ok
    }

    pub fn handle_set_location_counter(&mut self, tokens: Tokens) -> AsmError {
        match self.parse_operand(tokens.operand()) {
            Ok(val) => self.object_code.set_location_counter(val as u16),
            Err(err) => err,
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
        AsmError::InvalidMnemonic
    }

    pub fn handle_implied(&mut self, tokens: Tokens) -> AsmError {
        self.assemble(AddrMode::Implied, tokens)
    }

    pub fn handle_immediate(&mut self, tokens: Tokens) -> AsmError {
        self.assemble(AddrMode::Immediate, tokens)
    }

    pub fn handle_branch(&mut self, tokens: Tokens) -> AsmError {
        self.assemble(AddrMode::Branch, tokens)
    }

    pub fn handle_absolute(&mut self, tokens: Tokens) -> AsmError {
        self.assemble(AddrMode::Absolute, tokens)
    }

    pub fn handle_absolute_indexed_x(&mut self, tokens: Tokens) -> AsmError {
        self.assemble(AddrMode::AbsoluteX, tokens)
    }

    pub fn handle_absolute_indexed_y(&mut self, tokens: Tokens) -> AsmError {
        self.assemble(AddrMode::AbsoluteY, tokens)
    }

    pub fn handle_indirect(&mut self, tokens: Tokens) -> AsmError {
        self.assemble(AddrMode::Indirect, tokens)
    }

    pub fn handle_indexed_indirect_x(&mut self, tokens: Tokens) -> AsmError {
        self.assemble(AddrMode::IndexedIndirectX, tokens)
    }

    pub fn handle_indirect_indexed_y(&mut self, tokens: Tokens) -> AsmError {
        self.assemble(AddrMode::IndirectIndexedY, tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_next(asm: &mut Assembler, line: &str, expected: &[u8]) {
        let r = asm.process_line(line);
        assert!(matches!(r, AsmError::Ok), "line: \"{}\" error: {:?}", line, r);
        let generated = &asm.object_code.data[(asm.object_code.data.len() - expected.len())..];
        assert_eq!(generated, expected, "generated code {:?} differs from {:?}", generated, expected);
    }

    fn assert_asm(line: &str, code: &[u8]) -> Assembler {
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
        assert!(asm.symbols.is_empty());
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
        asm.symbols.insert("c".to_string(), 0xfab0);
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
        assert_asm("LDA ($8c,X)  ;komentarz", &[0xa1, 0x8c]);
        // assert_asm("  ;  komentarz", &[]);
        // assert_asm("label: ;komentarz numer 2", &[0x70, 8]);
        // assert_asm("LSR $35f0,X ;comment", &[0x5e, 0xf0, 0x35]);
    }

    /*
        #[test]
    fn (AssemblerTest, testComment) {
        TEST_INST("  SEI   ;disable interrupts ");
        TEST_INST("; disable interrupts ");
        TEST_INST(" LDA #$20  ;comment");
        }

        #[test]
    fn (AssemblerTest, testEmptyLineLabel) {
        assembler.changeMode(Assembler::ProcessingMode::ScanForSymbols);
        TEST_INST("Label_001:");
        EXPECT_EQ(symbols.get("Label_001"), assembler.m_locationCounter);
        EXPECT_EQ(symbols.get("dummy"), std::nullopt);
        }

        #[test]
    fn (AssemblerTest, testSymbolPass) {
        assembler.init(1000);
        assembler.changeMode(Assembler::ProcessingMode::ScanForSymbols);
        TEST_INST("TestLabel_01:  SEI   ; disable interrupts ");
        TEST_INST("c:lda dziabaDucha");
        EXPECT_EQ(symbols.get("TestLabel_01"), 1000);
        EXPECT_EQ(assembler.bytesWritten(), 0);
        EXPECT_EQ(assembler.m_locationCounter, 1004);
        }

        #[test]
    fn (AssemblerTest, testAssemblyPass) {
        assembler.init(2002);
        TEST_INST("CLI");
        TEST_INST("TestLabel_11:  LDA #$20   ; this is a one weird comment  ");
        EXPECT_EQ(symbols.get("TestLabel_11"), std::nullopt);
        EXPECT_EQ(assembler.bytesWritten(), 3);
        EXPECT_EQ(assembler.m_locationCounter, 2005);
        }

        #[test]
    fn (AssemblerTest, testEmitBytes) {
        TEST_INST(".BYTE 20");
        EXPECT_EQ(assembler.bytesWritten(), 1);
        EXPECT_EQ(memory[assembler.m_lastLocationCounter], 20);
        TEST_INST(".BYTE $20 45 $4a");
        EXPECT_EQ(assembler.bytesWritten(), 4);
        EXPECT_EQ(memory[assembler.m_lastLocationCounter], 0x20);
        EXPECT_EQ(memory[assembler.m_lastLocationCounter + 1], 45);
        EXPECT_EQ(memory[assembler.m_lastLocationCounter + 2], 0x4a);
        TEST_INST(".BYTE $20, $3f,$4a ,$23 , 123");
        EXPECT_EQ(assembler.bytesWritten(), 9);
        EXPECT_EQ(memory[assembler.m_lastLocationCounter], 0x20);
        EXPECT_EQ(memory[assembler.m_lastLocationCounter + 1], 0x3f);
        EXPECT_EQ(memory[assembler.m_lastLocationCounter + 2], 0x4a);
        EXPECT_EQ(memory[assembler.m_lastLocationCounter + 3], 0x23);
        EXPECT_EQ(memory[assembler.m_lastLocationCounter + 4], 123);
        }

        #[test]
    fn (AssemblerTest, testEmitWords) {
        TEST_INST(".word $20ff $23af $fab0 ; test comment");
        EXPECT_EQ(assembler.bytesWritten(), 6);
        EXPECT_EQ(memory.word(assembler.m_lastLocationCounter), 0x20ff);
        EXPECT_EQ(memory.word(assembler.m_lastLocationCounter + 2), 0x23af);
        EXPECT_EQ(memory.word(assembler.m_lastLocationCounter + 4), 0xfab0);
        TEST_INST(".word $3000 $15ad 10230");
        EXPECT_EQ(assembler.bytesWritten(), 12);
        EXPECT_EQ(memory.word(assembler.m_lastLocationCounter), 0x3000);
        EXPECT_EQ(memory.word(assembler.m_lastLocationCounter + 2), 0x15ad);
        EXPECT_EQ(memory.word(assembler.m_lastLocationCounter + 4), 10230);
        }

        #[test]
    fn (AssemblerTest, testLowerCaseInstruction) {
        TEST_INST("cli");
        }

        #[test]
    fn (AssemblerTest, testDcb) {
        TEST_INST("dcb 0,0,0,0,0,0,0,0,0,$b,$b,$c,$f,$f,$f,$f");
        EXPECT_EQ(assembler.bytesWritten(), 16);
        EXPECT_EQ(memory[assembler.m_lastLocationCounter], 0);
        EXPECT_EQ(memory[assembler.m_lastLocationCounter + 15], 0xf);
        }

        #[test]
    fn (AssemblerTest, testLoBytePrefix) {
        TEST_INST_2("LDA #<$1afc", 0xa9, 0xfc);
        symbols.put("label", 0x2afe);
        TEST_INST_2("LDA #<label", 0xa9, 0xfe);
        TEST_INST("dcb <label, 2");
        EXPECT_EQ(memory[assembler.m_lastLocationCounter], 0xfe);
        EXPECT_EQ(memory[assembler.m_lastLocationCounter + 1], 2);
        }

        #[test]
    fn (AssemblerTest, testHiBytePrefix) {
        TEST_INST_2("LDA #>$1afc", 0xa9, 0x1a);
        symbols.put("label", 0x3afe);
        TEST_INST_2("LDA #>label", 0xa9, 0x3a);
        TEST_INST_2("dcb >label, 2", 0x3a, 2);
        }

        #[test]
    fn (AssemblerTest, testLoHiBytePrefix) {
        symbols.put("a", 0xfa20);
        symbols.put("b", 0x10a0);
        TEST_INST_2("dcb >a, <b", 0xfa, 0xa0);
        TEST_INST_2("dcb <a, >b", 0x20, 0x10);
        }

        #[test]
    fn (AssemblerTest, testSymbolDef) {
        assembler.changeMode(Assembler::ProcessingMode::ScanForSymbols);
        TEST_INST(".org $1000");
        TEST_INST("lda init");
        EXPECT_EQ(assembler.m_locationCounter, 0x1003);
        }
        */
}
