pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub pc: u16,
}

impl Registers {
    const SP_BASE: u16 = 0x0100;

    pub fn sp_address(&self) -> u16 {
        self.sp as u16 | Self::SP_BASE
    }
}
