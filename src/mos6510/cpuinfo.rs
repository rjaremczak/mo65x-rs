use super::cpu::{flags::Flags, registers::Registers, Cpu};

#[derive(Debug)]
pub struct CpuInfo {
    pub regs: Registers,
    pub flags: Flags,
}

impl From<&Cpu> for CpuInfo {
    fn from(cpu: &Cpu) -> Self {
        Self {
            regs: cpu.regs,
            flags: cpu.flags,
        }
    }
}
