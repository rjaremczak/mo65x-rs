use std::{fs::File, io::Read, path::Path};

use super::{addrmode::AddrMode, error::AppError, memory::Memory, operation::Operation};

pub fn disassemble(memory: &Memory, pc: &mut u16) -> String {
    let mut buf = format!("{:04X} ", pc);
    let opcode = memory[*pc];
    let operation = Operation::get(opcode);
    let opsize = operation.len() as u16;
    for i in 0..3 {
        if i < opsize {
            buf.push_str(&format!("{:02X} ", memory[*pc + i]));
        } else {
            buf.push_str("   ");
        }
    }
    buf.push_str(&format!(" {} ", operation.instruction.mnemonic()));
    let opaddr = *pc + 1;
    buf.push_str(&match operation.addrmode {
        AddrMode::Implied => String::from(""),
        AddrMode::Relative => format!("${:04X}", opaddr as i32 + (memory[opaddr] as i8 + 2) as i32),
        AddrMode::Immediate => format!("#${:02X}", memory[opaddr]),
        AddrMode::ZeroPage => format!("${:02X}", memory[opaddr]),
        AddrMode::ZeroPageX => format!("${:02X},X", memory[opaddr]),
        AddrMode::ZeroPageY => format!("${:02X},Y", memory[opaddr]),
        AddrMode::IndexedIndirectX => format!("(${:02X},X)", memory[opaddr]),
        AddrMode::IndirectIndexedY => format!("(${:02X}),Y", memory[opaddr]),
        AddrMode::Indirect => format!("(${:04X})", memory.word(opaddr)),
        AddrMode::Absolute => format!("${:04X}", memory.word(opaddr)),
        AddrMode::AbsoluteX => format!("${:04X},X", memory.word(opaddr)),
        AddrMode::AbsoluteY => format!("${:04X},Y", memory.word(opaddr)),
    });
    *pc += opsize;
    buf
}

pub fn disassemble_file<F: AsRef<Path>>(start_addr: u16, end_addr: Option<u16>, fpath: F) -> Result<Vec<String>, AppError> {
    let mut buf = Vec::new();
    let fsize = File::open(&fpath)?.read_to_end(&mut buf)?;
    let end_addr = end_addr.unwrap_or(start_addr.saturating_add(fsize as u16));
    let mut memory = Memory::new();
    memory.set_block(start_addr, &buf);
    let mut pc = start_addr;
    let mut lines = Vec::new();
    while pc < end_addr {
        lines.push(disassemble(&memory, &mut pc));
    }
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_absolute() {
        let mut memory = Memory::new();
        let mut pc: u16 = 0x1000;
        memory[pc] = 0xad;
        memory.set_word(pc + 1, 0x1234);
        assert_eq!(disassemble(&memory, &mut pc), "1000 AD 34 12  LDA $1234");
    }
}
