use crate::mos6510::error::AsmError;

pub struct ObjectCode {
    pub origin: u16,
    pub data: Vec<u8>,
}

pub struct ObjectCodeBuilder {
    pub write_enabled: bool,
    pub location_counter: u16,
    pub object_code: ObjectCode,
}

impl ObjectCodeBuilder {
    pub fn new(origin: u16) -> Self {
        Self {
            object_code: ObjectCode { origin, data: Vec::new() },
            location_counter: origin,
            write_enabled: false,
        }
    }

    pub fn emit_byte(&mut self, byte: u8) {
        self.location_counter += 1;
        if self.write_enabled {
            self.object_code.data.push(byte);
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
                    self.object_code.data.push(0)
                }
            }
            AsmError::Ok
        } else {
            AsmError::ValueOutOfRange
        }
    }
}
