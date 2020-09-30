use super::addressing_mode::*;
use super::instruction::*;

use AddressingMode::*;
use Instruction::*;

pub struct OpCode {
    pub instruction: Instruction,
    pub addressing_mode: AddressingMode,
    pub size: u8,
    pub cycles: u8,
}

const fn oc(ins: Instruction, am: AddressingMode, s: u8, c: u8) -> OpCode {
    OpCode {
        instruction: ins,
        addressing_mode: am,
        size: s,
        cycles: c,
    }
}

pub static OPCODES: [OpCode; 1] = [oc(BRK, Implied, 1, 1)];
