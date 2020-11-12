#[derive(PartialEq)]
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

impl AddrMode {
    pub fn def(&self) -> &'static AddrModeDef {
        ADDR_MODE_DEFS.iter().find(|e| e.id == *self).unwrap()
    }
}

pub struct AddrModeDef {
    pub id: AddrMode,
    pub op_size: u8,
    pub zp_mode: Option<AddrMode>,
}

impl AddrModeDef {
    fn new(id: AddrMode, op_size: u8, zp_mode: Option<AddrMode>) -> Self {
        Self { id, op_size, zp_mode }
    }
}

use AddrMode::*;

pub static ADDR_MODE_DEFS: [AddrModeDef; 12] = [
    AddrModeDef::new(Implied, 0, None),
    AddrModeDef::new(Branch, 1, None),
    AddrModeDef::new(Immediate, 1, None),
    AddrModeDef::new(ZeroPage, 1, None),
    AddrModeDef::new(ZeroPageX, 1, None),
    AddrModeDef::new(ZeroPageY, 1, None),
    AddrModeDef::new(IndexedIndirectX, 1, None),
    AddrModeDef::new(IndirectIndexedY, 1, None),
    AddrModeDef::new(Indirect, 2, None),
    AddrModeDef::new(Absolute, 2, Some(ZeroPage)),
    AddrModeDef::new(AbsoluteX, 2, Some(ZeroPageX)),
    AddrModeDef::new(AbsoluteY, 2, Some(ZeroPageY)),
];
