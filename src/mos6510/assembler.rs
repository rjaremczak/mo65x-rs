mod operand;
mod patterns;
mod tokens;

#[cfg(test)]
mod assembler_tests;

use super::{
    addrmode::*,
    instruction::{Instruction, InstructionDef},
    opcode::OpCode,
};
use crate::mos6510::error::AppError;
use operand::OperandParser;
use regex::Regex;
use std::io::Read;
use std::{collections::HashMap, fs::File, path::Path};
use tokens::Tokens;

type Handler = fn(&mut Assembler, tokens: Tokens) -> Result<(), AppError>;

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

    pub fn process_line(&mut self, line: &str) -> Result<(), AppError> {
        for (regex, handler) in self.handlers.iter() {
            if let Some(captures) = regex.captures(&line) {
                let tokens = Tokens::new(captures);
                if let Some(label) = tokens.label() {
                    self.operand_parser.define_symbol(label, self.location_counter as i32);
                };
                return handler(self, tokens);
            }
        }
        Err(AppError::SyntaxError)
    }

    fn preprocess(instruction: Instruction, addrmode: AddrMode, opt_opval: Option<i32>) -> (&'static AddrModeDef, i32) {
        let jmp = instruction == Instruction::Jsr || instruction == Instruction::Jmp;
        let addrmode_def = addrmode.def();
        match addrmode_def.zp_mode.and_then(|o| if jmp { None } else { Some(o) }) {
            Some(zp_mode) => match opt_opval {
                Some(opval) => match is_zero_page(opval) {
                    true => (zp_mode.def(), opval),
                    false => (addrmode_def, opval),
                },
                None => (addrmode_def, 0),
            },
            None => (addrmode_def, opt_opval.unwrap_or(0)),
        }
    }

    fn parse_operand_list(&self, oplist: Option<&str>) -> Result<Vec<i32>, AppError> {
        match oplist {
            Some(oplist) => {
                let mut values: Vec<i32> = Vec::new();
                for opstr in self.op_list_separator.split(oplist) {
                    match self.operand_parser.resolve(opstr, self.generate_code) {
                        Ok(opval) => values.push(opval),
                        Err(err) => return Err(err),
                    }
                }
                Ok(values)
            }
            None => Err(AppError::MissingOperand),
        }
    }

    fn assemble<'a>(&mut self, addrmode: AddrMode, tokens: Tokens) -> Result<(), AppError> {
        let operand = if addrmode == AddrMode::Implied {
            None
        } else {
            let str = tokens.operand().ok_or(AppError::MissingOperand)?;
            Some(self.operand_parser.resolve(str, self.generate_code)?)
        };
        let operation = tokens.operation().ok_or(AppError::SyntaxError)?;
        let instruction_def = InstructionDef::by_mnemonic(operation).ok_or(AppError::InvalidMnemonic(String::from(operation)))?;
        let (addrmode_def, opvalue) = Self::preprocess(instruction_def.id, addrmode, operand);
        let opcode =
            OpCode::find(instruction_def.id, addrmode_def.id).ok_or(AppError::OpcodeNotFound(instruction_def.id, addrmode_def.id))?;
        self.emit_byte(opcode.code);
        if addrmode_def.op_size == 1 {
            self.emit_byte(opvalue as u8);
        } else if addrmode_def.op_size == 2 {
            self.emit_word(opvalue as u16);
        }
        Ok(())
    }

    pub fn reset_phase(&mut self, generate_code: bool) {
        self.generate_code = generate_code;
        self.origin = None;
        self.location_counter = 0;
        self.code.clear();
    }

    pub fn code(&self) -> &Vec<u8> {
        &self.code
    }

    pub fn symbols(&self) -> &HashMap<String, i32> {
        self.operand_parser.symbols()
    }

    pub fn origin(&self) -> u16 {
        self.origin.unwrap_or(self.location_counter)
    }

    pub fn handle_empty_line(&mut self, _: Tokens) -> Result<(), AppError> {
        Ok(())
    }

    pub fn handle_set_location_counter(&mut self, tokens: Tokens) -> Result<(), AppError> {
        let str = tokens.operand().ok_or(AppError::MissingOperand)?;
        let addr = self.operand_parser.resolve(str, false)?;
        self.set_location_counter(addr as u16)
    }

    pub fn handle_emit_bytes(&mut self, tokens: Tokens) -> Result<(), AppError> {
        let values = self.parse_operand_list(tokens.operand())?;
        values.iter().for_each(|v| self.emit_byte(*v as u8));
        Ok(())
    }

    pub fn handle_emit_words(&mut self, tokens: Tokens) -> Result<(), AppError> {
        let values = self.parse_operand_list(tokens.operand())?;
        values.iter().for_each(|v| self.emit_word(*v as u16));
        Ok(())
    }

    pub fn handle_implied(&mut self, tokens: Tokens) -> Result<(), AppError> {
        self.assemble(AddrMode::Implied, tokens)
    }

    pub fn handle_immediate(&mut self, tokens: Tokens) -> Result<(), AppError> {
        self.assemble(AddrMode::Immediate, tokens)
    }

    pub fn handle_branch(&mut self, tokens: Tokens) -> Result<(), AppError> {
        self.assemble(AddrMode::Branch, tokens)
    }

    pub fn handle_absolute(&mut self, tokens: Tokens) -> Result<(), AppError> {
        self.assemble(AddrMode::Absolute, tokens)
    }

    pub fn handle_absolute_indexed_x(&mut self, tokens: Tokens) -> Result<(), AppError> {
        self.assemble(AddrMode::AbsoluteX, tokens)
    }

    pub fn handle_absolute_indexed_y(&mut self, tokens: Tokens) -> Result<(), AppError> {
        self.assemble(AddrMode::AbsoluteY, tokens)
    }

    pub fn handle_indirect(&mut self, tokens: Tokens) -> Result<(), AppError> {
        self.assemble(AddrMode::Indirect, tokens)
    }

    pub fn handle_indexed_indirect_x(&mut self, tokens: Tokens) -> Result<(), AppError> {
        self.assemble(AddrMode::IndexedIndirectX, tokens)
    }

    pub fn handle_indirect_indexed_y(&mut self, tokens: Tokens) -> Result<(), AppError> {
        self.assemble(AddrMode::IndirectIndexedY, tokens)
    }

    pub fn emit_byte(&mut self, byte: u8) {
        self.location_counter = self.location_counter.wrapping_add(1);
        if self.generate_code {
            self.code.push(byte);
        }
    }

    pub fn emit_word(&mut self, word: u16) {
        self.emit_byte(word as u8);
        self.emit_byte((word >> 8) as u8);
    }

    pub fn set_location_counter(&mut self, addr: u16) -> Result<(), AppError> {
        if self.origin.is_none() {
            self.origin = Some(addr);
            self.location_counter = addr;
            Ok(())
        } else if addr >= self.location_counter {
            let lc = self.location_counter;
            self.location_counter = addr;
            if self.generate_code {
                for _ in lc..self.location_counter {
                    self.code.push(0)
                }
            }
            Ok(())
        } else {
            Err(AppError::AddrOutOfRange(addr, self.location_counter))
        }
    }

    fn process_file(&mut self, generate_code: bool, strbuf: &String) -> Result<(), AppError> {
        self.reset_phase(generate_code);
        for line in strbuf.lines() {
            self.process_line(line)?;
        }
        Ok(())
    }
}

pub fn assemble_file<F: AsRef<Path>>(fname: F) -> Result<(u16, Vec<u8>, HashMap<String, i32>), AppError> {
    let mut src = String::new();
    File::open(&fname)?.read_to_string(&mut src)?;
    let mut asm = Assembler::new();
    asm.process_file(false, &src)?;
    asm.process_file(true, &src)?;
    Ok((asm.origin(), asm.code().to_vec(), asm.symbols().clone()))
}
