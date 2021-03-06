mod operand;
mod patterns;
mod tokens;

#[cfg(test)]
mod assembler_tests;

use self::operand::Operand;
use super::{addrmode::*, instruction::Instruction, operation::find_opcode};
use crate::error::AppError;
use operand::Resolver;
use regex::Regex;
use std::convert::TryFrom;
use std::io::Read;
use std::{collections::HashMap, fs::File, path::Path};
use tokens::Tokens;
use AddrMode::Implied;
use Instruction::{Jmp, Jsr};

type Handler = fn(&mut Assembler, tokens: Tokens) -> Result<(), AppError>;

const DEFAULT_LOCATION_COUNTER: u16 = 0;

pub struct Assembler {
    handlers: Vec<(Regex, Handler)>,
    resolver: Resolver,
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
            resolver: Resolver::new(),
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
                    (p.ins_branch, Assembler::handle_relative),
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
                    self.resolver.define_symbol(label, self.location_counter as i32)?;
                };
                return handler(self, tokens);
            }
        }
        Err(AppError::SyntaxError(String::from(line)))
    }

    fn parse_operand_list(&self, oplist: Option<&str>) -> Result<Vec<i32>, AppError> {
        match oplist {
            Some(oplist) => {
                let mut values: Vec<i32> = Vec::new();
                for opstr in self.op_list_separator.split(oplist) {
                    match self.resolver.resolve(opstr, self.generate_code) {
                        Ok(operand) => values.push(operand.value),
                        Err(err) => return Err(err),
                    }
                }
                Ok(values)
            }
            None => Err(AppError::MissingOperand),
        }
    }

    fn prepare_operand(&mut self, addrmode: AddrMode, opstr: Option<&str>) -> Result<Operand, AppError> {
        if addrmode == AddrMode::Implied {
            Ok(Operand::literal(0))
        } else {
            let opstr = opstr.ok_or(AppError::MissingOperand)?;
            let mut operand = self.resolver.resolve(opstr, self.generate_code)?;
            if self.generate_code && addrmode == AddrMode::Relative && operand.symbolic {
                let diff = operand.value - self.location_counter as i32 - 2;
                let displacement = i8::try_from(diff).map_err(|_| AppError::BranchTooFar(diff))?;
                operand.value = displacement as i32;
            }
            Ok(operand)
        }
    }

    fn assemble(&mut self, addrmode: AddrMode, tokens: Tokens) -> Result<(), AppError> {
        let operand = self.prepare_operand(addrmode, tokens.operand())?;
        let mnemonic = tokens.operation().ok_or(AppError::SyntaxError(tokens.to_string()))?;
        let instruction = Instruction::parse(mnemonic)?;
        let addrmode = optimize_addrmode(instruction, addrmode, operand);
        let opcode = find_opcode(instruction, addrmode)?;
        self.emit_byte(opcode);
        match addrmode.len() {
            1 => self.emit_byte(operand.value as u8),
            2 => self.emit_word(operand.value as u16),
            _ => {}
        }
        Ok(())
    }

    pub fn init_pass(&mut self, generate_code: bool) {
        self.generate_code = generate_code;
        self.origin = None;
        self.location_counter = DEFAULT_LOCATION_COUNTER;
        self.code.clear();
    }

    pub fn code(&self) -> &Vec<u8> {
        &self.code
    }

    pub fn symbols(&self) -> &HashMap<String, i32> {
        self.resolver.symbols()
    }

    pub fn origin(&self) -> u16 {
        self.origin.unwrap_or(self.location_counter)
    }

    fn handle_empty_line(&mut self, _: Tokens) -> Result<(), AppError> {
        Ok(())
    }

    fn handle_set_location_counter(&mut self, tokens: Tokens) -> Result<(), AppError> {
        let str = tokens.operand().ok_or(AppError::MissingOperand)?;
        let operand = self.resolver.resolve(str, false)?;
        self.set_location_counter(operand.value as u16)
    }

    fn handle_emit_bytes(&mut self, tokens: Tokens) -> Result<(), AppError> {
        let values = self.parse_operand_list(tokens.operand())?;
        values.iter().for_each(|v| self.emit_byte(*v as u8));
        Ok(())
    }

    fn handle_emit_words(&mut self, tokens: Tokens) -> Result<(), AppError> {
        let values = self.parse_operand_list(tokens.operand())?;
        values.iter().for_each(|v| self.emit_word(*v as u16));
        Ok(())
    }

    fn handle_implied(&mut self, tokens: Tokens) -> Result<(), AppError> {
        self.assemble(AddrMode::Implied, tokens)
    }

    fn handle_immediate(&mut self, tokens: Tokens) -> Result<(), AppError> {
        self.assemble(AddrMode::Immediate, tokens)
    }

    fn handle_relative(&mut self, tokens: Tokens) -> Result<(), AppError> {
        self.assemble(AddrMode::Relative, tokens)
    }

    fn handle_absolute(&mut self, tokens: Tokens) -> Result<(), AppError> {
        self.assemble(AddrMode::Absolute, tokens)
    }

    fn handle_absolute_indexed_x(&mut self, tokens: Tokens) -> Result<(), AppError> {
        self.assemble(AddrMode::AbsoluteX, tokens)
    }

    fn handle_absolute_indexed_y(&mut self, tokens: Tokens) -> Result<(), AppError> {
        self.assemble(AddrMode::AbsoluteY, tokens)
    }

    fn handle_indirect(&mut self, tokens: Tokens) -> Result<(), AppError> {
        self.assemble(AddrMode::Indirect, tokens)
    }

    fn handle_indexed_indirect_x(&mut self, tokens: Tokens) -> Result<(), AppError> {
        self.assemble(AddrMode::IndexedIndirectX, tokens)
    }

    fn handle_indirect_indexed_y(&mut self, tokens: Tokens) -> Result<(), AppError> {
        self.assemble(AddrMode::IndirectIndexedY, tokens)
    }

    fn emit_byte(&mut self, byte: u8) {
        if self.generate_code {
            self.code.push(byte);
        }
        self.location_counter = self.location_counter.wrapping_add(1);
    }

    fn emit_word(&mut self, word: u16) {
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
            Err(AppError::OriginTooLow(addr, self.location_counter))
        }
    }

    fn process_file(&mut self, generate_code: bool, strbuf: &String) -> Result<(), AppError> {
        self.init_pass(generate_code);
        for (num, line) in strbuf.lines().enumerate() {
            self.process_line(line).map_err(|e| AppError::AsmLineError(num + 1, Box::from(e)))?;
        }
        Ok(())
    }
}

fn optimize_addrmode(instruction: Instruction, addrmode: AddrMode, operand: Operand) -> AddrMode {
    if addrmode == Implied || instruction == Jsr || instruction == Jmp || operand.symbolic {
        addrmode
    } else {
        addrmode.optimized(operand.value)
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
