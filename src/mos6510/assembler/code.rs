use super::error::AsmError;

pub struct ObjectCode {
    pub write_enabled: bool,
    pub origin: u16,
    pub location_counter: u16,
    pub data: Vec<u8>,
}

impl ObjectCode {
    pub fn new(origin: u16) -> Self {
        Self {
            origin,
            location_counter: origin,
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

    pub fn set_location_counter(&mut self, addr: u16) -> AsmError {
        if addr >= self.location_counter {
            let lc = self.location_counter;
            self.location_counter = addr;
            if self.write_enabled {
                for _ in lc..self.location_counter {
                    self.data.push(0)
                }
            }
            AsmError::Ok
        } else {
            AsmError::ValueOutOfRange
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //fn
}
