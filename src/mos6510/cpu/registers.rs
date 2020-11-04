pub struct Registers {
    a: u8,
    x: u8,
    y: u8,
    sp: u8,
    pc: u16,
}

impl Registers {
    const SP_BASE: u16 = 0x0100;

    pub fn sp_address(&self) -> u16 {
        self.sp as u16 | Self::SP_BASE
    }
}
