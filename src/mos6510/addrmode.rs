use AddrMode::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum AddrMode {
    Implied,
    Relative,
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

impl AddrMode {
    pub fn len(&self) -> u8 {
        match self {
            Implied => 0,
            Relative | Immediate | ZeroPage | ZeroPageX | ZeroPageY | IndexedIndirectX | IndirectIndexedY => 1,
            Indirect | Absolute | AbsoluteX | AbsoluteY => 2,
        }
    }

    pub fn optimized(&self, opvalue: i32) -> AddrMode {
        if opvalue < 0 || opvalue > 255 {
            *self
        } else {
            match self {
                Absolute => ZeroPage,
                AbsoluteX => ZeroPageX,
                AbsoluteY => ZeroPageY,
                _ => *self,
            }
        }
    }
}
