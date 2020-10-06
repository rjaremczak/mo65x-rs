const MEMORY_SIZE: usize = u16::MAX as usize + 1;

pub struct Memory {
    data: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        Memory { data: [0; MEMORY_SIZE] }
    }

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
        self.byte(address) as u16 | (self.byte(address.wrapping_add(1)) as u16) << 8
    }

    #[inline]
    pub fn set_word(&mut self, address: u16, value: u16) {
        self.set_byte(address, value as u8);
        self.set_byte(address.wrapping_add(1), (value >> 8) as u8);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Memory {
        fn test_byte_rw(&mut self, address: u16, value: u8) {
            self.set_byte(address, value);
            assert_eq!(value, self.byte(address));
        }
        fn test_word_rw(&mut self, address: u16, value: u16) {
            self.set_word(address, value);
            assert_eq!(value, self.word(address));
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
        assert_eq!(0x0c, mem.byte(0xffff));
        assert_eq!(0xfa, mem.byte(0x0000));
    }

    #[test]
    fn endianness() {
        let mut mem = Memory::new();
        mem.set_byte(0x1000, 0xA0);
        mem.set_byte(0x1001, 0x1D);
        assert_eq!(0x1DA0, mem.word(0x1000));
    }
}