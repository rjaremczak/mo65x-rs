mod patterns;
mod tokens;

use crate::mos6510::instruction::parse_instruction;

use self::tokens::Tokens;

use super::super::opcode::*;
use super::error::AsmError;
use super::object_code::ObjectCode;
use super::*;
use super::{super::addrmode::AddrMode, operand::resolve_operand};
use super::{super::instruction::Instruction, operand::is_zero_page_operand};
use regex::{Captures, Match, Regex};
use std::collections::HashMap;

type Symbols = HashMap<String, i32>;
type Handler = fn(&mut Assembler, tokens: Tokens) -> AsmError;

pub struct Assembler {
    pub handlers: Vec<(Regex, Handler)>,
    pub code_generation: bool,
    pub location_counter: u16,
    pub location_counter_prev: u16,
    pub bytes_written: u32,
    pub symbols: Symbols,
    pub object_code: ObjectCode,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            code_generation: false,
            location_counter: 0,
            location_counter_prev: 0,
            bytes_written: 0,
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
                if !self.code_generation {
                    if let Some(label) = tokens.label() {
                        self.symbols.insert(String::from(label), self.location_counter as i32);
                    };
                }
                return handler(self, tokens);
            }
        }
        AsmError::SyntaxError
    }

    fn preprocess(addrmode: AddrMode, operand: Result<i32, AsmError>) -> (AddrMode, i32) {
        match addrmode.zero_page_variant() {
            Some(zp_mode) => match operand {
                Ok(opvalue) => {
                    if is_zero_page_operand(opvalue) {
                        (zp_mode, opvalue)
                    } else {
                        (addrmode, opvalue)
                    }
                }
                Err(_) => (addrmode, 0),
            },
            None => (addrmode, 0),
        }
    }

    fn assemble(&mut self, addrmode: AddrMode, tokens: Tokens) -> AsmError {
        let operand = resolve_operand(tokens.operand(), |s| Some(1));
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
        println!("empty line");
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
        AsmError::InvalidMnemonic
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

    #[test]
    fn initial_state() {
        let asm = Assembler::new();
        assert_eq!(asm.code_generation, false);
        assert_eq!(asm.bytes_written, 0);
        assert_eq!(asm.location_counter, 0);
        assert_eq!(asm.location_counter_prev, 0);
        assert!(asm.symbols.is_empty());
    }

    #[test]
    fn empty_line() {
        let asm = Assembler::new();
        let mut ap = Assembler::new();
        let r = asm.process_line("");
        assert!(matches!(r, AsmError::Ok));
        assert_eq!(ap.location_counter_prev, 0);
        assert!(ap.symbols.is_empty());
    }

    #[test]
    fn implied_mode() {
        let asm = Assembler::new();
        let r = asm.process_line("SEI");
        assert!(matches!(r, AsmError::Ok));
        assert_eq!(asm.location_counter_prev, 1);
        assert!(asm.symbols.is_empty());
        assert!(asm.object_code.data.is_empty());
    }
}
