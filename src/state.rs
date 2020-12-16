use crate::mos6510::cpu::{flags::Flags, registers::Registers};

#[derive(Debug)]
pub struct State {
    pub regs: Registers,
    pub flags: Flags,
    pub cycles: u64,
    pub duration: std::time::Duration,
    pub trap: bool,
}
