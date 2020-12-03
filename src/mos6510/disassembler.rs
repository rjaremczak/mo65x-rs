use super::{memory::Memory, operation::Operation};

pub fn disassemble_instr(memory: &Memory, pc: u16, buf: &mut String) -> u16 {
    let operation = Operation::get(memory[pc]);
    let instrdef = operation.instruction.mnemonic();
    // for addr in pc+1..pc+opcode.
    0
}
