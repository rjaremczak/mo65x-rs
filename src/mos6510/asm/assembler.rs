mod patterns;
mod tokens;

use self::tokens::Tokens;
use super::error::AsmError;
use super::object_code::ObjectCode;
use super::*;
use super::{super::addrmode::AddrMode, operand::resolve_operand};
use super::{super::opcode::*, operand::is_zero_page_operand};
use crate::mos6510::instruction::parse_instruction;
use regex::Regex;
use std::collections::HashMap;

type Symbols = HashMap<String, i32>;
type Handler = fn(&mut Assembler, tokens: Tokens) -> AsmError;

pub struct Assembler {
    handlers: Vec<(Regex, Handler)>,
    symbols: Symbols,
    object_code: ObjectCode,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            symbols: Symbols::new(),
            object_code: ObjectCode::new(),
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
                if !self.object_code.write_enabled {
                    if let Some(label) = tokens.label() {
                        self.symbols.insert(String::from(label), self.object_code.location_counter as i32);
                    };
                }
                return handler(self, tokens);
            }
        }
        AsmError::SyntaxError
    }

    fn preprocess(addrmode: AddrMode, operand: Option<i32>) -> (AddrMode, i32) {
        match addrmode.zero_page_variant() {
            Some(zp_mode) => match operand {
                Some(opvalue) => match is_zero_page_operand(opvalue) {
                    true => (zp_mode, opvalue),
                    false => (addrmode, opvalue),
                },
                None => (addrmode, 0),
            },
            None => (addrmode, operand.unwrap_or(0)),
        }
    }

    fn assemble(&mut self, addrmode: AddrMode, tokens: Tokens) -> AsmError {
        let operand = match addrmode {
            AddrMode::Implied => match resolve_operand(tokens.operand(), |s| self.symbols.get(s).map(|v| *v)) {
                Ok(opvalue) => Some(opvalue),
                Err(err) => return err,
            },
            _ => None,
        };
        let (opt_addrmode, opvalue) = Self::preprocess(addrmode, operand);
        match tokens.operation() {
            Some(operation) => match parse_instruction(operation) {
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

    pub fn handle_empty_line(&mut self, tokens: Tokens) -> AsmError {
        AsmError::Ok
    }

    pub fn handle_set_location_counter(&mut self, tokens: Tokens) -> AsmError {
        println!("set location counter");
        AsmError::Ok
    }

    pub fn handle_emit_bytes(&mut self, tokens: Tokens) -> AsmError {
        AsmError::InvalidMnemonic
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
        AsmError::InvalidMnemonic
    }

    pub fn handle_absolute(&mut self, tokens: Tokens) -> AsmError {
        AsmError::InvalidMnemonic
    }

    pub fn handle_absolute_indexed_x(&mut self, tokens: Tokens) -> AsmError {
        AsmError::InvalidMnemonic
    }

    pub fn handle_absolute_indexed_y(&mut self, tokens: Tokens) -> AsmError {
        AsmError::InvalidMnemonic
    }

    pub fn handle_indirect(&mut self, tokens: Tokens) -> AsmError {
        AsmError::InvalidMnemonic
    }

    pub fn handle_indexed_indirect_x(&mut self, tokens: Tokens) -> AsmError {
        AsmError::InvalidMnemonic
    }

    pub fn handle_indirect_indexed_y(&mut self, tokens: Tokens) -> AsmError {
        AsmError::InvalidMnemonic
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_asm(asm: &mut Assembler, line: &str, code: &[u8]) {
        let r = asm.process_line(line);
        assert!(matches!(r, AsmError::Ok), "line {} assembly error: {:?}", line, r);
        assert_eq!(
            asm.object_code.location_counter,
            code.len() as u16,
            "generated code size is {} but should be {}",
            asm.object_code.location_counter,
            code.len() as u16
        );
        assert_eq!(
            asm.object_code.data.as_slice(),
            code,
            "generated code {:?} differs from {:?}",
            asm.object_code.data.as_slice(),
            code
        );
    }

    fn assert_once(line: &str, code: &[u8]) -> Assembler {
        let mut asm = Assembler::new();
        asm.object_code.write_enabled = true;
        assert_asm(&mut asm, line, code);
        asm
    }

    #[test]
    fn init() {
        let asm = Assembler::new();
        assert_eq!(asm.object_code.write_enabled, false);
        assert_eq!(asm.object_code.location_counter, 0);
        assert!(asm.symbols.is_empty());
    }

    #[test]
    fn empty_line() {
        assert_once("", &[]);
    }

    #[test]
    fn implied_mode() {
        assert_once("SEI", &[0x78]);
    }

    #[test]
    fn immediate_mode() {
        assert_once("LDA #%00110101", &[0xa9, 0b00110101]);
        //assert_once("LDY #255", &[0xa4, 0xff]);
        //assert_once("LDX #123", &[0xa2, 123]);
    }
}
