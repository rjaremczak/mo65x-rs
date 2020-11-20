use super::error::AsmError;

pub struct CodeBuilder {
    pub origin: u16,
    pub code: Vec<u8>,
    pub write_enabled: bool,
    pub location_counter: u16,
}

impl CodeBuilder {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            origin: 0,
            location_counter: 0,
            write_enabled: false,
        }
    }

    pub fn emit_byte(&mut self, byte: u8) {
        self.location_counter += 1;
        if self.write_enabled {
            self.code.push(byte);
        }
    }

    pub fn emit_word(&mut self, word: u16) {
        self.emit_byte(word as u8);
        self.emit_byte((word >> 8) as u8);
    }

    pub fn set_location_counter(&mut self, addr: u16) -> AsmError {
        if addr >= self.location_counter {
            let lc = self.location_counter;
            self.location_counter = addr;
            if self.write_enabled {
                for _ in lc..self.location_counter {
                    self.code.push(0)
                }
            }
            AsmError::Ok
        } else {
            AsmError::ValueOutOfRange
        }
    }
}
