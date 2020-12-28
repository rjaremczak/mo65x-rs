use crate::mos6510::cpu::{flags::Flags, registers::Registers};

#[derive(Debug, Clone, Copy, Default)]
pub struct State {
    pub regs: Registers,
    pub flags: Flags,
    pub cycles: u64,
    pub duration: std::time::Duration,
    pub trap: bool,
}

impl State {
    pub fn frequency(&self) -> f64 {
        self.cycles as f64 / self.duration.as_secs_f64()
    }
}
