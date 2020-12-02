use super::addrmode::{AddrMode, AddrMode::*};
use super::instruction::Instruction::{self, *};

#[derive(Debug)]
pub struct OpCode {
    pub code: u8,
    pub instruction: Instruction,
    pub addrmode: AddrMode,
    pub cycles: u8,
}

impl OpCode {
    const fn new(code: u8, instruction: Instruction, addrmode: AddrMode, cycles: u8) -> OpCode {
        OpCode {
            code,
            addrmode,
            instruction,
            cycles,
        }
    }

    pub fn matches(&self, instruction: Instruction, addrmode: AddrMode) -> bool {
        self.instruction == instruction && self.addrmode == addrmode
    }

    pub fn find(instruction: Instruction, addrmode: AddrMode) -> Option<&'static OpCode> {
        OPCODES.iter().find(|oc| oc.matches(instruction, addrmode))
    }
}

pub static OPCODES: [OpCode; 256] = [
    OpCode::new(0x00, Brk, Implied, 7),
    OpCode::new(0x01, Ora, IndexedIndirectX, 6),
    OpCode::new(0x02, Kil, Implied, 0),
    OpCode::new(0x03, Kil, Implied, 0),
    OpCode::new(0x04, Kil, Implied, 0),
    OpCode::new(0x05, Ora, ZeroPage, 3),
    OpCode::new(0x06, Asl, ZeroPage, 5),
    OpCode::new(0x07, Kil, Implied, 0),
    OpCode::new(0x08, Php, Implied, 3),
    OpCode::new(0x09, Ora, Immediate, 2),
    OpCode::new(0x0a, Asl, Implied, 2),
    OpCode::new(0x0b, Kil, Implied, 0),
    OpCode::new(0x0c, Kil, Implied, 0),
    OpCode::new(0x0d, Ora, Absolute, 4),
    OpCode::new(0x0e, Asl, Absolute, 6),
    OpCode::new(0x0f, Kil, Implied, 0),
    OpCode::new(0x10, Bpl, Branch, 2),
    OpCode::new(0x11, Ora, IndirectIndexedY, 5),
    OpCode::new(0x12, Kil, Implied, 0),
    OpCode::new(0x13, Kil, Implied, 0),
    OpCode::new(0x14, Kil, Implied, 0),
    OpCode::new(0x15, Ora, ZeroPageX, 4),
    OpCode::new(0x16, Asl, ZeroPageX, 6),
    OpCode::new(0x17, Kil, Implied, 0),
    OpCode::new(0x18, Clc, Implied, 2),
    OpCode::new(0x19, Ora, AbsoluteY, 4),
    OpCode::new(0x1a, Kil, Implied, 0),
    OpCode::new(0x1b, Kil, Implied, 0),
    OpCode::new(0x1c, Kil, Implied, 0),
    OpCode::new(0x1d, Ora, AbsoluteX, 4),
    OpCode::new(0x1e, Asl, AbsoluteX, 7),
    OpCode::new(0x1f, Kil, Implied, 0),
    OpCode::new(0x20, Jsr, Absolute, 6),
    OpCode::new(0x21, And, IndexedIndirectX, 6),
    OpCode::new(0x22, Kil, Implied, 0),
    OpCode::new(0x23, Kil, Implied, 0),
    OpCode::new(0x24, Bit, ZeroPage, 3),
    OpCode::new(0x25, And, ZeroPage, 3),
    OpCode::new(0x26, Rol, ZeroPage, 5),
    OpCode::new(0x27, Kil, Implied, 0),
    OpCode::new(0x28, Plp, Implied, 4),
    OpCode::new(0x29, And, Immediate, 2),
    OpCode::new(0x2a, Rol, Implied, 2),
    OpCode::new(0x2b, Kil, Implied, 0),
    OpCode::new(0x2c, Bit, Absolute, 4),
    OpCode::new(0x2d, And, Absolute, 4),
    OpCode::new(0x2e, Rol, Absolute, 6),
    OpCode::new(0x2f, Kil, Implied, 0),
    OpCode::new(0x30, Bmi, Branch, 2),
    OpCode::new(0x31, And, IndirectIndexedY, 5),
    OpCode::new(0x32, Kil, Implied, 0),
    OpCode::new(0x33, Kil, Implied, 0),
    OpCode::new(0x34, Kil, Implied, 0),
    OpCode::new(0x35, And, ZeroPageX, 4),
    OpCode::new(0x36, Rol, ZeroPageX, 6),
    OpCode::new(0x37, Kil, Implied, 0),
    OpCode::new(0x38, Sec, Implied, 2),
    OpCode::new(0x39, And, AbsoluteY, 4),
    OpCode::new(0x3a, Kil, Implied, 0),
    OpCode::new(0x3b, Kil, Implied, 0),
    OpCode::new(0x3c, Kil, Implied, 0),
    OpCode::new(0x3d, And, AbsoluteX, 4),
    OpCode::new(0x3e, Rol, AbsoluteX, 7),
    OpCode::new(0x3f, Kil, Implied, 0),
    OpCode::new(0x40, Rti, Implied, 6),
    OpCode::new(0x41, Eor, IndexedIndirectX, 6),
    OpCode::new(0x42, Kil, Implied, 0),
    OpCode::new(0x43, Kil, Implied, 0),
    OpCode::new(0x44, Kil, Implied, 0),
    OpCode::new(0x45, Eor, ZeroPage, 3),
    OpCode::new(0x46, Lsr, ZeroPage, 5),
    OpCode::new(0x47, Kil, Implied, 0),
    OpCode::new(0x48, Pha, Implied, 3),
    OpCode::new(0x49, Eor, Immediate, 2),
    OpCode::new(0x4a, Lsr, Implied, 2),
    OpCode::new(0x4b, Kil, Implied, 0),
    OpCode::new(0x4c, Jmp, Absolute, 3),
    OpCode::new(0x4d, Eor, Absolute, 4),
    OpCode::new(0x4e, Lsr, Absolute, 6),
    OpCode::new(0x4f, Kil, Implied, 0),
    OpCode::new(0x50, Bvc, Branch, 2),
    OpCode::new(0x51, Eor, IndirectIndexedY, 5),
    OpCode::new(0x52, Kil, Implied, 0),
    OpCode::new(0x53, Kil, Implied, 0),
    OpCode::new(0x54, Kil, Implied, 0),
    OpCode::new(0x55, Eor, ZeroPageX, 4),
    OpCode::new(0x56, Lsr, ZeroPageX, 6),
    OpCode::new(0x57, Kil, Implied, 0),
    OpCode::new(0x58, Cli, Implied, 2),
    OpCode::new(0x59, Eor, AbsoluteY, 4),
    OpCode::new(0x5a, Kil, Implied, 0),
    OpCode::new(0x5b, Kil, Implied, 0),
    OpCode::new(0x5c, Kil, Implied, 0),
    OpCode::new(0x5d, Eor, AbsoluteX, 4),
    OpCode::new(0x5e, Lsr, AbsoluteX, 7),
    OpCode::new(0x5f, Kil, Implied, 0),
    OpCode::new(0x60, Rts, Implied, 6),
    OpCode::new(0x61, Adc, IndexedIndirectX, 6),
    OpCode::new(0x62, Kil, Implied, 0),
    OpCode::new(0x63, Kil, Implied, 0),
    OpCode::new(0x64, Kil, Implied, 0),
    OpCode::new(0x65, Adc, ZeroPage, 3),
    OpCode::new(0x66, Ror, ZeroPage, 5),
    OpCode::new(0x67, Kil, Implied, 0),
    OpCode::new(0x68, Pla, Implied, 4),
    OpCode::new(0x69, Adc, Immediate, 2),
    OpCode::new(0x6a, Ror, Implied, 2),
    OpCode::new(0x6b, Kil, Implied, 0),
    OpCode::new(0x6c, Jmp, Indirect, 5),
    OpCode::new(0x6d, Adc, Absolute, 4),
    OpCode::new(0x6e, Ror, Absolute, 6),
    OpCode::new(0x6f, Kil, Implied, 0),
    OpCode::new(0x70, Bvs, Branch, 2),
    OpCode::new(0x71, Adc, IndirectIndexedY, 5),
    OpCode::new(0x72, Kil, Implied, 0),
    OpCode::new(0x73, Kil, Implied, 0),
    OpCode::new(0x74, Kil, Implied, 0),
    OpCode::new(0x75, Adc, ZeroPageX, 4),
    OpCode::new(0x76, Ror, ZeroPageX, 6),
    OpCode::new(0x77, Kil, Implied, 0),
    OpCode::new(0x78, Sei, Implied, 2),
    OpCode::new(0x79, Adc, AbsoluteY, 4),
    OpCode::new(0x7a, Kil, Implied, 0),
    OpCode::new(0x7b, Kil, Implied, 0),
    OpCode::new(0x7c, Kil, Implied, 0),
    OpCode::new(0x7d, Adc, AbsoluteX, 4),
    OpCode::new(0x7e, Ror, AbsoluteX, 7),
    OpCode::new(0x7f, Kil, Implied, 0),
    OpCode::new(0x80, Kil, Implied, 0),
    OpCode::new(0x81, Sta, IndexedIndirectX, 6),
    OpCode::new(0x82, Kil, Implied, 0),
    OpCode::new(0x83, Kil, Implied, 0),
    OpCode::new(0x84, Sty, ZeroPage, 3),
    OpCode::new(0x85, Sta, ZeroPage, 3),
    OpCode::new(0x86, Stx, ZeroPage, 3),
    OpCode::new(0x87, Kil, Implied, 0),
    OpCode::new(0x88, Dey, Implied, 2),
    OpCode::new(0x89, Kil, Implied, 0),
    OpCode::new(0x8a, Txa, Implied, 2),
    OpCode::new(0x8b, Kil, Implied, 0),
    OpCode::new(0x8c, Sty, Absolute, 4),
    OpCode::new(0x8d, Sta, Absolute, 4),
    OpCode::new(0x8e, Stx, Absolute, 4),
    OpCode::new(0x8f, Kil, Implied, 0),
    OpCode::new(0x90, Bcc, Branch, 2),
    OpCode::new(0x91, Sta, IndirectIndexedY, 6),
    OpCode::new(0x92, Kil, Implied, 0),
    OpCode::new(0x93, Kil, Implied, 0),
    OpCode::new(0x94, Sty, ZeroPageX, 4),
    OpCode::new(0x95, Sta, ZeroPageX, 4),
    OpCode::new(0x96, Stx, ZeroPageY, 4),
    OpCode::new(0x97, Kil, Implied, 0),
    OpCode::new(0x98, Tya, Implied, 2),
    OpCode::new(0x99, Sta, AbsoluteY, 5),
    OpCode::new(0x9a, Txs, Implied, 2),
    OpCode::new(0x9b, Kil, Implied, 0),
    OpCode::new(0x9c, Kil, Implied, 0),
    OpCode::new(0x9d, Sta, AbsoluteX, 5),
    OpCode::new(0x9e, Kil, Implied, 0),
    OpCode::new(0x9f, Kil, Implied, 0),
    OpCode::new(0xa0, Ldy, Immediate, 2),
    OpCode::new(0xa1, Lda, IndexedIndirectX, 6),
    OpCode::new(0xa2, Ldx, Immediate, 2),
    OpCode::new(0xa3, Kil, Implied, 0),
    OpCode::new(0xa4, Ldy, ZeroPage, 3),
    OpCode::new(0xa5, Lda, ZeroPage, 3),
    OpCode::new(0xa6, Ldx, ZeroPage, 3),
    OpCode::new(0xa7, Kil, Implied, 0),
    OpCode::new(0xa8, Tay, Implied, 2),
    OpCode::new(0xa9, Lda, Immediate, 2),
    OpCode::new(0xaa, Tax, Implied, 2),
    OpCode::new(0xab, Kil, Implied, 0),
    OpCode::new(0xac, Ldy, Absolute, 4),
    OpCode::new(0xad, Lda, Absolute, 4),
    OpCode::new(0xae, Ldx, Absolute, 4),
    OpCode::new(0xaf, Kil, Implied, 0),
    OpCode::new(0xb0, Bcs, Branch, 2),
    OpCode::new(0xb1, Lda, IndirectIndexedY, 5),
    OpCode::new(0xb2, Kil, Implied, 0),
    OpCode::new(0xb3, Kil, Implied, 0),
    OpCode::new(0xb4, Ldy, ZeroPageX, 4),
    OpCode::new(0xb5, Lda, ZeroPageX, 4),
    OpCode::new(0xb6, Ldx, ZeroPageY, 4),
    OpCode::new(0xb7, Kil, Implied, 0),
    OpCode::new(0xb8, Clv, Implied, 2),
    OpCode::new(0xb9, Lda, AbsoluteY, 4),
    OpCode::new(0xba, Tsx, Implied, 2),
    OpCode::new(0xbb, Kil, Implied, 0),
    OpCode::new(0xbc, Ldy, AbsoluteX, 4),
    OpCode::new(0xbd, Lda, AbsoluteX, 4),
    OpCode::new(0xbe, Ldx, AbsoluteY, 4),
    OpCode::new(0xbf, Kil, Implied, 0),
    OpCode::new(0xc0, Cpy, Immediate, 2),
    OpCode::new(0xc1, Cmp, IndexedIndirectX, 6),
    OpCode::new(0xc2, Kil, Implied, 0),
    OpCode::new(0xc3, Kil, Implied, 0),
    OpCode::new(0xc4, Cpy, ZeroPage, 3),
    OpCode::new(0xc5, Cmp, ZeroPage, 3),
    OpCode::new(0xc6, Dec, ZeroPage, 5),
    OpCode::new(0xc7, Kil, Implied, 0),
    OpCode::new(0xc8, Iny, Implied, 2),
    OpCode::new(0xc9, Cmp, Immediate, 2),
    OpCode::new(0xca, Dex, Implied, 2),
    OpCode::new(0xcb, Kil, Implied, 0),
    OpCode::new(0xcc, Cpy, Absolute, 4),
    OpCode::new(0xcd, Cmp, Absolute, 4),
    OpCode::new(0xce, Dec, Absolute, 6),
    OpCode::new(0xcf, Kil, Implied, 0),
    OpCode::new(0xd0, Bne, Branch, 2),
    OpCode::new(0xd1, Cmp, IndirectIndexedY, 5),
    OpCode::new(0xd2, Kil, Implied, 0),
    OpCode::new(0xd3, Kil, Implied, 0),
    OpCode::new(0xd4, Kil, Implied, 0),
    OpCode::new(0xd5, Cmp, ZeroPageX, 4),
    OpCode::new(0xd6, Dec, ZeroPageX, 6),
    OpCode::new(0xd7, Kil, Implied, 0),
    OpCode::new(0xd8, Cld, Implied, 2),
    OpCode::new(0xd9, Cmp, AbsoluteY, 4),
    OpCode::new(0xda, Kil, Implied, 0),
    OpCode::new(0xdb, Kil, Implied, 0),
    OpCode::new(0xdc, Kil, Implied, 0),
    OpCode::new(0xdd, Kil, Implied, 0),
    OpCode::new(0xde, Cmp, AbsoluteX, 4),
    OpCode::new(0xdf, Dec, AbsoluteX, 7),
    OpCode::new(0xe0, Cpx, Immediate, 2),
    OpCode::new(0xe1, Sbc, IndexedIndirectX, 6),
    OpCode::new(0xe2, Kil, Implied, 0),
    OpCode::new(0xe3, Kil, Implied, 0),
    OpCode::new(0xe4, Cpx, ZeroPage, 3),
    OpCode::new(0xe5, Sbc, ZeroPage, 3),
    OpCode::new(0xe6, Inc, ZeroPage, 5),
    OpCode::new(0xe7, Kil, Implied, 0),
    OpCode::new(0xe8, Inx, Implied, 2),
    OpCode::new(0xe9, Sbc, Immediate, 2),
    OpCode::new(0xea, Nop, Implied, 2),
    OpCode::new(0xeb, Kil, Implied, 0),
    OpCode::new(0xec, Cpx, Absolute, 4),
    OpCode::new(0xed, Sbc, Absolute, 4),
    OpCode::new(0xee, Inc, Absolute, 6),
    OpCode::new(0xef, Kil, Implied, 0),
    OpCode::new(0xf0, Beq, Branch, 2),
    OpCode::new(0xf1, Sbc, IndirectIndexedY, 5),
    OpCode::new(0xf2, Kil, Implied, 0),
    OpCode::new(0xf3, Kil, Implied, 0),
    OpCode::new(0xf4, Kil, Implied, 0),
    OpCode::new(0xf5, Sbc, ZeroPageX, 4),
    OpCode::new(0xf6, Inc, ZeroPageX, 6),
    OpCode::new(0xf7, Kil, Implied, 0),
    OpCode::new(0xf8, Sed, Implied, 2),
    OpCode::new(0xf9, Sbc, AbsoluteY, 4),
    OpCode::new(0xfa, Kil, Implied, 0),
    OpCode::new(0xfb, Kil, Implied, 0),
    OpCode::new(0xfc, Kil, Implied, 0),
    OpCode::new(0xfd, Sbc, AbsoluteX, 4),
    OpCode::new(0xfe, Inc, AbsoluteX, 7),
    OpCode::new(0xff, Kil, Implied, 0),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_opcode() {
        match OpCode::find(Jmp, Absolute) {
            Some(oc) => assert_eq!(oc.code, 0x4c),
            None => assert!(false, "opcode not found"),
        }
    }
}
