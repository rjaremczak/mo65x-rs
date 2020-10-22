pub struct ObjectCode {
    pub origin: u16,
    pub location_counter: u16,
    pub data: Vec<u8>,
    pub write_enabled: bool,
}

impl ObjectCode {
    pub fn new() -> Self {
        Self {
            origin: 0,
            location_counter: 0,
            data: Vec::new(),
            write_enabled: false,
        }
    }

    pub fn emit_byte(&mut self, byte: u8) {
        self.location_counter += 1;
        if self.write_enabled {
            self.data.push(byte);
        }
    }

    pub fn emit_word(&mut self, word: u16) {
        self.emit_byte(word as u8);
        self.emit_byte((word >> 8) as u8);
    }

    pub fn fill_gap(&mut self, len: usize) {
        for _ in 0..len {
            self.data.push(0)
        }
    }
}
