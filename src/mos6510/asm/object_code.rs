pub struct ObjectCode {
    pub origin: u16,
    pub data: Vec<u8>,
}

impl ObjectCode {
    pub fn new() -> Self {
        Self {
            origin: 0,
            data: Vec::new(),
        }
    }

    pub fn emit_byte(&mut self, byte: u8) {
        self.data.push(byte);
    }

    pub fn emit_word(&mut self, word: u16) {
        self.data.push(word as u8);
        self.data.push((word >> 8) as u8);
    }
}
