use crate::mos6510::memory::SP_BASE;

pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub pc: u16,
}

impl Registers {
    pub fn new(pc: u16, sp: u8) -> Self {
        Self { a: 0, x: 0, y: 0, pc, sp }
    }

    pub fn sp_address(&self) -> u16 {
        self.sp as u16 | SP_BASE
    }
}
