use crate::mos6510::cpu::{flags::Flags, registers::Registers};

#[derive(Debug, Clone, Copy, Default)]
pub struct Info {
    pub regs: Registers,
    pub flags: Flags,
    pub cycles: u64,
    pub duration: std::time::Duration,
    pub trap: bool,
    pub rst: u16,
    pub nmi: u16,
    pub irq: u16,
    pub io_data: u8,
    pub io_config: u8,
}

impl Info {
    pub fn frequency(&self) -> f64 {
        self.cycles as f64 / self.duration.as_secs_f64()
    }
}
