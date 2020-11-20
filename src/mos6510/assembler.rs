mod object_code;
mod operand;
mod patterns;
mod tokens;

#[cfg(test)]
mod tests;

use super::{addr, addrmode::*, error::AsmError, instruction::InstructionDef, opcode::OpCode};
use object_code::ObjectCode;
use object_code::ObjectCodeBuilder;
use operand::OperandParser;
use regex::Regex;
use tokens::Tokens;

type Handler = fn(&mut Assembler, tokens: Tokens) -> AsmError;

pub struct Assembler {
    handlers: Vec<(Regex, Handler)>,
    operand_parser: OperandParser,
    object_code_builder: ObjectCodeBuilder,
    op_list_separator: Regex,
}

impl Assembler {
    pub fn new(origin: u16) -> Assembler {
        Assembler {
            operand_parser: OperandParser::new(),
            object_code_builder: ObjectCodeBuilder::new(origin),
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

    pub fn object_code(&self) -> &ObjectCode {
        &self.object_code_builder.object_code
    }

    pub fn process_line(&mut self, line: &str) -> AsmError {
        for (regex, handler) in self.handlers.iter() {
            if let Some(captures) = regex.captures(&line) {
                let tokens = Tokens::new(captures);
                if let Some(label) = tokens.label() {
                    self.operand_parser
                        .define_symbol(label, self.object_code_builder.location_counter as i32);
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
                        self.object_code_builder.emit_byte(opcode.code);
                        if addrmode_def.op_size == 1 {
                            self.object_code_builder.emit_byte(opvalue as u8);
                        } else if addrmode_def.op_size == 2 {
                            self.object_code_builder.emit_word(opvalue as u16);
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
        self.object_code_builder.write_enabled = on;
    }

    pub fn handle_empty_line(&mut self, _: Tokens) -> AsmError {
        AsmError::Ok
    }

    pub fn handle_set_location_counter(&mut self, tokens: Tokens) -> AsmError {
        match tokens.operand() {
            Some(opstr) => match self.operand_parser.resolve(opstr) {
                Ok(val) => self.object_code_builder.set_location_counter(val as u16),
                Err(err) => err,
            },
            None => AsmError::MissingOperand,
        }
    }

    pub fn handle_emit_bytes(&mut self, tokens: Tokens) -> AsmError {
        match self.parse_operand_list(tokens.operand()) {
            Ok(values) => {
                values.iter().for_each(|v| self.object_code_builder.emit_byte(*v as u8));
                AsmError::Ok
            }
            Err(err) => err,
        }
    }

    pub fn handle_emit_words(&mut self, tokens: Tokens) -> AsmError {
        match self.parse_operand_list(tokens.operand()) {
            Ok(values) => {
                values.iter().for_each(|v| self.object_code_builder.emit_word(*v as u16));
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
}
