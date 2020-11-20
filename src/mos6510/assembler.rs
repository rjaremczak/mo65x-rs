pub mod error;

mod operand;
mod patterns;
mod tokens;

#[cfg(test)]
mod assembler_tests;

use super::{addr, addrmode::*, instruction::InstructionDef, opcode::OpCode};
use error::AsmError;
use operand::OperandParser;
use regex::Regex;
use tokens::Tokens;

type Handler = fn(&mut Assembler, tokens: Tokens) -> AsmError;

pub struct Assembler {
    handlers: Vec<(Regex, Handler)>,
    operand_parser: OperandParser,
    origin: Option<u16>,
    code: Vec<u8>,
    generate_code: bool,
    location_counter: u16,
    op_list_separator: Regex,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            code: Vec::new(),
            origin: None,
            location_counter: 0,
            generate_code: false,
            operand_parser: OperandParser::new(),
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
                    self.operand_parser.define_symbol(label, self.location_counter as i32);
                };
                return handler(self, tokens);
            }
        }
        AsmError::SyntaxError
    }

    fn preprocess(addrmode: AddrMode, opt_opval: Option<i32>) -> (&'static AddrModeDef, i32) {
        let addrmode_def = addrmode.def();
        match addrmode_def.zp_mode {
            Some(zp_mode) => match opt_opval {
                Some(opval) => match addr::is_zero_page(opval) {
                    true => (zp_mode.def(), opval),
                    false => (addrmode_def, opval),
                },
                None => (addrmode_def, 0),
            },
            None => (addrmode_def, opt_opval.unwrap_or(0)),
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

    fn assemble<'a>(&mut self, addrmode: AddrMode, tokens: Tokens) -> AsmError {
        let operand = if addrmode == AddrMode::Implied {
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
        let (addrmode_def, opvalue) = Self::preprocess(addrmode, operand);
        match tokens.operation() {
            Some(operation) => match InstructionDef::by_mnemonic(operation) {
                Some(instruction_def) => match OpCode::find(instruction_def.id, addrmode_def.id) {
                    Some(opcode) => {
                        self.emit_byte(opcode.code);
                        if addrmode_def.op_size == 1 {
                            self.emit_byte(opvalue as u8);
                        } else if addrmode_def.op_size == 2 {
                            self.emit_word(opvalue as u16);
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

    pub fn set_generate_code(&mut self, on: bool) {
        self.generate_code = on;
    }

    pub fn code(&self) -> &Vec<u8> {
        &self.code
    }

    pub fn origin(&self) -> u16 {
        self.origin.unwrap_or(self.location_counter)
    }

    pub fn handle_empty_line(&mut self, _: Tokens) -> AsmError {
        AsmError::Ok
    }

    pub fn handle_set_location_counter(&mut self, tokens: Tokens) -> AsmError {
        match tokens.operand() {
            Some(opstr) => match self.operand_parser.resolve(opstr) {
                Ok(val) => self.set_location_counter(val as u16),
                Err(err) => err,
            },
            None => AsmError::MissingOperand,
        }
    }

    pub fn handle_emit_bytes(&mut self, tokens: Tokens) -> AsmError {
        match self.parse_operand_list(tokens.operand()) {
            Ok(values) => {
                values.iter().for_each(|v| self.emit_byte(*v as u8));
                AsmError::Ok
            }
            Err(err) => err,
        }
    }

    pub fn handle_emit_words(&mut self, tokens: Tokens) -> AsmError {
        match self.parse_operand_list(tokens.operand()) {
            Ok(values) => {
                values.iter().for_each(|v| self.emit_word(*v as u16));
                AsmError::Ok
            }
            Err(err) => err,
        }
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

    pub fn emit_byte(&mut self, byte: u8) {
        self.location_counter += 1;
        if self.generate_code {
            self.code.push(byte);
        }
    }

    pub fn emit_word(&mut self, word: u16) {
        self.emit_byte(word as u8);
        self.emit_byte((word >> 8) as u8);
    }

    pub fn set_location_counter(&mut self, addr: u16) -> AsmError {
        if self.origin.is_none() {
            self.origin = Some(addr);
            self.location_counter = addr;
            AsmError::Ok
        } else if addr >= self.location_counter {
            let lc = self.location_counter;
            self.location_counter = addr;
            if self.generate_code {
                for _ in lc..self.location_counter {
                    self.code.push(0)
                }
            }
            AsmError::Ok
        } else {
            AsmError::ValueOutOfRange
        }
    }
}

pub fn process_lines() -> Result<(u16, Vec<u8>), AsmError> {
    let asm = Assembler::new();
    Ok((asm.origin(), asm.code().to_vec()))
}
