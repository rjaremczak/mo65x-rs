mod execenv;
mod flags;
mod registers;

use self::{execenv::ExecEnv, flags::Flags, registers::Registers};

enum CpuExecMode {
    Normal,
    PendingIrq,
    PendingNmi,
    PendingReset,
}
enum CpuState {
    Idle,
    Stopped,
    Halted,
    Running,
    Stopping,
}

pub struct Cpu {
    state: CpuState,
    exec_mode: CpuExecMode,
    regs: Registers,
    flags: Flags,
}

pub type AddrModeHandler = fn(&mut Cpu, &mut ExecEnv);
pub type InstructionHandler = fn(&mut Cpu, &mut ExecEnv);

impl Cpu {
    pub fn exec_prepare(&mut self) {}

    pub fn exec_instruction(&mut self) -> u8 {
        0
    }

    pub fn exec_conclude(&mut self) {}

    pub fn mode_implied(&mut self, env: &mut ExecEnv) {}
    pub fn mode_branch(&mut self, env: &mut ExecEnv) {}
    pub fn mode_immediate(&mut self, env: &mut ExecEnv) {}
    pub fn mode_zero_page(&mut self, env: &mut ExecEnv) {}
    pub fn mode_zero_page_x(&mut self, env: &mut ExecEnv) {}
    pub fn mode_zero_page_y(&mut self, env: &mut ExecEnv) {}
    pub fn mode_indexed_indirect_x(&mut self, env: &mut ExecEnv) {}
    pub fn mode_indirect_indexed_y(&mut self, env: &mut ExecEnv) {}
    pub fn mode_indirect(&mut self, env: &mut ExecEnv) {}
    pub fn mode_absolute(&mut self, env: &mut ExecEnv) {}
    pub fn mode_absolute_x(&mut self, env: &mut ExecEnv) {}
    pub fn mode_absolute_y(&mut self, env: &mut ExecEnv) {}
}
