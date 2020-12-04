use super::{memory::Memory, operation::Operation};

pub fn disassemble_instr(memory: &Memory, pc: u16, buf: &mut String) -> u16 {
    let operation = Operation::get(memory[pc]);
    buf.push_str(operation.instruction.mnemonic());
    buf.push(' ');
    let opsize = operation.addrmode.len();
    for i in 1..opsize {}
    0
}
