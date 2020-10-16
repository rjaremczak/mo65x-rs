pub enum AddrMode {
    Implied,
    Branch,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    IndexedIndirectX,
    IndirectIndexedY,
    Indirect,
    Absolute,
    AbsoluteX,
    AbsoluteY,
}

use AddrMode::*;

impl AddrMode {
    pub const fn operand_size(&self) -> u8 {
        match self {
            Implied => 0,
            Branch | Immediate | ZeroPage | ZeroPageX | ZeroPageY | IndexedIndirectX | IndirectIndexedY => 1,
            Indirect | Absolute | AbsoluteX | AbsoluteY => 2,
        }
    }
}
