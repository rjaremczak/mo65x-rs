pub const MEMORY_SIZE: usize = u16::MAX as usize + 1;
pub const IO_PORT_CONFIG: u16 = 0x0000;
pub const IO_PORT_DATA: u16 = 0x0001;
pub const SP_BASE: u16 = 0x0100;
pub const NMI_VECTOR: u16 = 0xfffa;
pub const RESET_VECTOR: u16 = 0xfffc;
pub const IRQ_VECTOR: u16 = 0xfffe;

pub struct Memory {
    data: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        Memory { data: [0; MEMORY_SIZE] }
    }

    #[inline(always)]
    pub fn getb(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    #[inline(always)]
    pub fn setb(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }

    #[inline(always)]
    pub fn getw(&self, address: u16) -> u16 {
        self.getb(address) as u16 | (self.getb(address.wrapping_add(1)) as u16) << 8
    }

    #[inline(always)]
    pub fn setw(&mut self, address: u16, value: u16) {
        self.setb(address, value as u8);
        self.setb(address.wrapping_add(1), (value >> 8) as u8);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Memory {
        fn test_byte_rw(&mut self, address: u16, value: u8) {
            self.setb(address, value);
            assert_eq!(value, self.getb(address));
        }
        fn test_word_rw(&mut self, address: u16, value: u16) {
            self.setw(address, value);
            assert_eq!(value, self.getw(address));
        }
    }

    #[test]
    fn byte_access() {
        let mut mem = Memory::new();
        mem.test_byte_rw(0, 123);
        mem.test_byte_rw(30000, 180);
        mem.test_byte_rw(65535, 250);
    }

    #[test]
    fn word_access() {
        let mut mem = Memory::new();
        mem.test_word_rw(2, 0x1002);
        mem.test_word_rw(30000, 0x8ABC);
        mem.test_word_rw(65535, 0xFA0C);
        assert_eq!(0x0c, mem.getb(0xffff));
        assert_eq!(0xfa, mem.getb(0x0000));
    }

    #[test]
    fn endianness() {
        let mut mem = Memory::new();
        mem.setb(0x1000, 0xA0);
        mem.setb(0x1001, 0x1D);
        assert_eq!(0x1DA0, mem.getw(0x1000));
    }
}
