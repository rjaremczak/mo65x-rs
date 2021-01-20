use super::addrmode::{AddrMode, AddrMode::*};
use super::instruction::Instruction::{self, *};
use crate::error::AppError;

#[derive(Debug)]
pub struct Operation {
    pub instruction: Instruction,
    pub addrmode: AddrMode,
    pub cycles: u8,
}

impl Operation {
    const fn new(instruction: Instruction, addrmode: AddrMode, cycles: u8) -> Self {
        Self {
            instruction,
            addrmode,
            cycles,
        }
    }

    pub fn matches(&self, instruction: Instruction, addrmode: AddrMode) -> bool {
        self.instruction == instruction && self.addrmode == addrmode
    }

    #[inline]
    pub fn get(code: u8) -> &'static Operation {
        OPCODE_MAP.get(&code).unwrap_or(&OPCODE_KIL)
    }

    #[inline]
    pub fn len(&self) -> u8 {
        self.addrmode.len() + 1
    }
}

pub fn find_opcode(instruction: Instruction, addrmode: AddrMode) -> Result<u8, AppError> {
    OPCODE_MAP
        .iter()
        .find(|kv| kv.1.matches(instruction, addrmode))
        .map(|kv| *kv.0)
        .ok_or(AppError::NoOpCode(instruction, addrmode))
}

use std::collections::BTreeMap;

lazy_static! {
    static ref OPCODE_MAP: BTreeMap<u8, Operation> = {
        let mut m = BTreeMap::new();
        m.insert(0x00, Operation::new(Brk, Implied, 7));
        m.insert(0x01, Operation::new(Ora, IndexedIndirectX, 6));
        m.insert(0x05, Operation::new(Ora, ZeroPage, 3));
        m.insert(0x06, Operation::new(Asl, ZeroPage, 5));
        m.insert(0x08, Operation::new(Php, Implied, 3));
        m.insert(0x09, Operation::new(Ora, Immediate, 2));
        m.insert(0x0a, Operation::new(Asl, Implied, 2));
        m.insert(0x0d, Operation::new(Ora, Absolute, 4));
        m.insert(0x0e, Operation::new(Asl, Absolute, 6));
        m.insert(0x10, Operation::new(Bpl, Relative, 2));
        m.insert(0x11, Operation::new(Ora, IndirectIndexedY, 5));
        m.insert(0x15, Operation::new(Ora, ZeroPageX, 4));
        m.insert(0x16, Operation::new(Asl, ZeroPageX, 6));
        m.insert(0x18, Operation::new(Clc, Implied, 2));
        m.insert(0x19, Operation::new(Ora, AbsoluteY, 4));
        m.insert(0x1d, Operation::new(Ora, AbsoluteX, 4));
        m.insert(0x1e, Operation::new(Asl, AbsoluteX, 7));
        m.insert(0x20, Operation::new(Jsr, Absolute, 6));
        m.insert(0x21, Operation::new(And, IndexedIndirectX, 6));
        m.insert(0x24, Operation::new(Bit, ZeroPage, 3));
        m.insert(0x25, Operation::new(And, ZeroPage, 3));
        m.insert(0x26, Operation::new(Rol, ZeroPage, 5));
        m.insert(0x28, Operation::new(Plp, Implied, 4));
        m.insert(0x29, Operation::new(And, Immediate, 2));
        m.insert(0x2a, Operation::new(Rol, Implied, 2));
        m.insert(0x2c, Operation::new(Bit, Absolute, 4));
        m.insert(0x2d, Operation::new(And, Absolute, 4));
        m.insert(0x2e, Operation::new(Rol, Absolute, 6));
        m.insert(0x30, Operation::new(Bmi, Relative, 2));
        m.insert(0x31, Operation::new(And, IndirectIndexedY, 5));
        m.insert(0x35, Operation::new(And, ZeroPageX, 4));
        m.insert(0x36, Operation::new(Rol, ZeroPageX, 6));
        m.insert(0x38, Operation::new(Sec, Implied, 2));
        m.insert(0x39, Operation::new(And, AbsoluteY, 4));
        m.insert(0x3d, Operation::new(And, AbsoluteX, 4));
        m.insert(0x3e, Operation::new(Rol, AbsoluteX, 7));
        m.insert(0x40, Operation::new(Rti, Implied, 6));
        m.insert(0x41, Operation::new(Eor, IndexedIndirectX, 6));
        m.insert(0x45, Operation::new(Eor, ZeroPage, 3));
        m.insert(0x46, Operation::new(Lsr, ZeroPage, 5));
        m.insert(0x48, Operation::new(Pha, Implied, 3));
        m.insert(0x49, Operation::new(Eor, Immediate, 2));
        m.insert(0x4a, Operation::new(Lsr, Implied, 2));
        m.insert(0x4c, Operation::new(Jmp, Absolute, 3));
        m.insert(0x4d, Operation::new(Eor, Absolute, 4));
        m.insert(0x4e, Operation::new(Lsr, Absolute, 6));
        m.insert(0x50, Operation::new(Bvc, Relative, 2));
        m.insert(0x51, Operation::new(Eor, IndirectIndexedY, 5));
        m.insert(0x55, Operation::new(Eor, ZeroPageX, 4));
        m.insert(0x56, Operation::new(Lsr, ZeroPageX, 6));
        m.insert(0x58, Operation::new(Cli, Implied, 2));
        m.insert(0x59, Operation::new(Eor, AbsoluteY, 4));
        m.insert(0x5d, Operation::new(Eor, AbsoluteX, 4));
        m.insert(0x5e, Operation::new(Lsr, AbsoluteX, 7));
        m.insert(0x60, Operation::new(Rts, Implied, 6));
        m.insert(0x61, Operation::new(Adc, IndexedIndirectX, 6));
        m.insert(0x65, Operation::new(Adc, ZeroPage, 3));
        m.insert(0x66, Operation::new(Ror, ZeroPage, 5));
        m.insert(0x68, Operation::new(Pla, Implied, 4));
        m.insert(0x69, Operation::new(Adc, Immediate, 2));
        m.insert(0x6a, Operation::new(Ror, Implied, 2));
        m.insert(0x6c, Operation::new(Jmp, Indirect, 5));
        m.insert(0x6d, Operation::new(Adc, Absolute, 4));
        m.insert(0x6e, Operation::new(Ror, Absolute, 6));
        m.insert(0x70, Operation::new(Bvs, Relative, 2));
        m.insert(0x71, Operation::new(Adc, IndirectIndexedY, 5));
        m.insert(0x75, Operation::new(Adc, ZeroPageX, 4));
        m.insert(0x76, Operation::new(Ror, ZeroPageX, 6));
        m.insert(0x78, Operation::new(Sei, Implied, 2));
        m.insert(0x79, Operation::new(Adc, AbsoluteY, 4));
        m.insert(0x7d, Operation::new(Adc, AbsoluteX, 4));
        m.insert(0x7e, Operation::new(Ror, AbsoluteX, 7));
        m.insert(0x81, Operation::new(Sta, IndexedIndirectX, 6));
        m.insert(0x84, Operation::new(Sty, ZeroPage, 3));
        m.insert(0x85, Operation::new(Sta, ZeroPage, 3));
        m.insert(0x86, Operation::new(Stx, ZeroPage, 3));
        m.insert(0x88, Operation::new(Dey, Implied, 2));
        m.insert(0x8a, Operation::new(Txa, Implied, 2));
        m.insert(0x8c, Operation::new(Sty, Absolute, 4));
        m.insert(0x8d, Operation::new(Sta, Absolute, 4));
        m.insert(0x8e, Operation::new(Stx, Absolute, 4));
        m.insert(0x90, Operation::new(Bcc, Relative, 2));
        m.insert(0x91, Operation::new(Sta, IndirectIndexedY, 6));
        m.insert(0x94, Operation::new(Sty, ZeroPageX, 4));
        m.insert(0x95, Operation::new(Sta, ZeroPageX, 4));
        m.insert(0x96, Operation::new(Stx, ZeroPageY, 4));
        m.insert(0x98, Operation::new(Tya, Implied, 2));
        m.insert(0x99, Operation::new(Sta, AbsoluteY, 5));
        m.insert(0x9a, Operation::new(Txs, Implied, 2));
        m.insert(0x9d, Operation::new(Sta, AbsoluteX, 5));
        m.insert(0xa0, Operation::new(Ldy, Immediate, 2));
        m.insert(0xa1, Operation::new(Lda, IndexedIndirectX, 6));
        m.insert(0xa2, Operation::new(Ldx, Immediate, 2));
        m.insert(0xa4, Operation::new(Ldy, ZeroPage, 3));
        m.insert(0xa5, Operation::new(Lda, ZeroPage, 3));
        m.insert(0xa6, Operation::new(Ldx, ZeroPage, 3));
        m.insert(0xa8, Operation::new(Tay, Implied, 2));
        m.insert(0xa9, Operation::new(Lda, Immediate, 2));
        m.insert(0xaa, Operation::new(Tax, Implied, 2));
        m.insert(0xac, Operation::new(Ldy, Absolute, 4));
        m.insert(0xad, Operation::new(Lda, Absolute, 4));
        m.insert(0xae, Operation::new(Ldx, Absolute, 4));
        m.insert(0xb0, Operation::new(Bcs, Relative, 2));
        m.insert(0xb1, Operation::new(Lda, IndirectIndexedY, 5));
        m.insert(0xb4, Operation::new(Ldy, ZeroPageX, 4));
        m.insert(0xb5, Operation::new(Lda, ZeroPageX, 4));
        m.insert(0xb6, Operation::new(Ldx, ZeroPageY, 4));
        m.insert(0xb8, Operation::new(Clv, Implied, 2));
        m.insert(0xb9, Operation::new(Lda, AbsoluteY, 4));
        m.insert(0xba, Operation::new(Tsx, Implied, 2));
        m.insert(0xbc, Operation::new(Ldy, AbsoluteX, 4));
        m.insert(0xbd, Operation::new(Lda, AbsoluteX, 4));
        m.insert(0xbe, Operation::new(Ldx, AbsoluteY, 4));
        m.insert(0xc0, Operation::new(Cpy, Immediate, 2));
        m.insert(0xc1, Operation::new(Cmp, IndexedIndirectX, 6));
        m.insert(0xc4, Operation::new(Cpy, ZeroPage, 3));
        m.insert(0xc5, Operation::new(Cmp, ZeroPage, 3));
        m.insert(0xc6, Operation::new(Dec, ZeroPage, 5));
        m.insert(0xc8, Operation::new(Iny, Implied, 2));
        m.insert(0xc9, Operation::new(Cmp, Immediate, 2));
        m.insert(0xca, Operation::new(Dex, Implied, 2));
        m.insert(0xcc, Operation::new(Cpy, Absolute, 4));
        m.insert(0xcd, Operation::new(Cmp, Absolute, 4));
        m.insert(0xce, Operation::new(Dec, Absolute, 6));
        m.insert(0xd0, Operation::new(Bne, Relative, 2));
        m.insert(0xd1, Operation::new(Cmp, IndirectIndexedY, 5));
        m.insert(0xd5, Operation::new(Cmp, ZeroPageX, 4));
        m.insert(0xd6, Operation::new(Dec, ZeroPageX, 6));
        m.insert(0xd8, Operation::new(Cld, Implied, 2));
        m.insert(0xd9, Operation::new(Cmp, AbsoluteY, 4));
        m.insert(0xde, Operation::new(Cmp, AbsoluteX, 4));
        m.insert(0xdf, Operation::new(Dec, AbsoluteX, 7));
        m.insert(0xe0, Operation::new(Cpx, Immediate, 2));
        m.insert(0xe1, Operation::new(Sbc, IndexedIndirectX, 6));
        m.insert(0xe4, Operation::new(Cpx, ZeroPage, 3));
        m.insert(0xe5, Operation::new(Sbc, ZeroPage, 3));
        m.insert(0xe6, Operation::new(Inc, ZeroPage, 5));
        m.insert(0xe8, Operation::new(Inx, Implied, 2));
        m.insert(0xe9, Operation::new(Sbc, Immediate, 2));
        m.insert(0xea, Operation::new(Nop, Implied, 2));
        m.insert(0xec, Operation::new(Cpx, Absolute, 4));
        m.insert(0xed, Operation::new(Sbc, Absolute, 4));
        m.insert(0xee, Operation::new(Inc, Absolute, 6));
        m.insert(0xf0, Operation::new(Beq, Relative, 2));
        m.insert(0xf1, Operation::new(Sbc, IndirectIndexedY, 5));
        m.insert(0xf5, Operation::new(Sbc, ZeroPageX, 4));
        m.insert(0xf6, Operation::new(Inc, ZeroPageX, 6));
        m.insert(0xf8, Operation::new(Sed, Implied, 2));
        m.insert(0xf9, Operation::new(Sbc, AbsoluteY, 4));
        m.insert(0xfd, Operation::new(Sbc, AbsoluteX, 4));
        m.insert(0xfe, Operation::new(Inc, AbsoluteX, 7));
        m.insert(0xff, Operation::new(Kil, Implied, 0));
        m
    };
}

static OPCODE_KIL: Operation = Operation::new(Kil, Implied, 0);

#[cfg(test)]
mod tests {
    use super::*;

    fn find(instruction: Instruction, addrmode: AddrMode) -> Option<(u8, &'static Operation)> {
        OPCODE_MAP
            .iter()
            .find(|kv| kv.1.matches(instruction, addrmode))
            .map(|kv| (*kv.0, kv.1))
    }

    #[test]
    fn test_find_operation() {
        match find(Jmp, Absolute) {
            Some(kv) => {
                assert_eq!(kv.0, 0x4c);
                assert_eq!(kv.1.instruction, Jmp);
                assert_eq!(kv.1.addrmode, Absolute);
            }
            None => assert!(false, "opcode not found"),
        }
    }

    #[test]
    fn test_unsupported_opcode() {
        let oc = Operation::get(0x02);
        assert_eq!(oc.instruction, Kil);
        assert_eq!(oc.addrmode, Implied);
        assert_eq!(oc.cycles, 0);
    }

    #[test]
    fn test_supported_opcode() {
        let oc = Operation::get(0xf0);
        assert_eq!(oc.instruction, Beq);
        assert_eq!(oc.addrmode, Relative);
        assert_eq!(oc.cycles, 2);
    }
}
