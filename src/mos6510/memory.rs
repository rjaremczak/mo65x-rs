pub struct Memory {
    data: [u8; u16::MAX as usize + 1],
}

impl Memory {
    #[inline]
    pub fn byte(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    #[inline]
    pub fn set_byte(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }

    #[inline]
    pub fn word(&self, address: u16) -> u16 {
        self.byte(address) as u16 | (self.byte(address + 1) << 8) as u16
    }

    #[inline]
    pub fn set_word(&mut self, address: u16, value: u16) {
        self.set_byte(address, value as u8);
        self.set_byte(address.wrapping_add(1), (value >> 8) as u8);
    }
}

mod tests {
    #[test]
    fn byte_access() {
        for i in 0..9 {}
    }
}
