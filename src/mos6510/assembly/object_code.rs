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
}
