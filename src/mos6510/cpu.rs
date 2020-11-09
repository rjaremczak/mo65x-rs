mod exec_env;
mod flags;
mod registers;
mod state;

use self::{exec_env::ExecEnv, flags::Flags, registers::Registers, state::CpuState};
use super::{memory::Memory, opcode::OPCODES};

pub struct Cpu {
    state: CpuState,
    regs: Registers,
    flags: Flags,
}

pub type AddrModeHandler = fn(&mut Cpu, &mut ExecEnv);
pub type InstructionHandler = fn(&mut Cpu, &mut ExecEnv);

impl Cpu {
    pub fn new(pc: u16) -> Self {
        Self {
            state: CpuState::new(),
            regs: Registers::new(pc, 0xfd),
            flags: Flags::new(),
        }
    }

    pub fn exec_begin(&mut self) {
        self.state.start_running();
    }

    pub fn exec_instruction(&mut self, memory: &mut Memory) -> u8 {
        let opcode = &OPCODES[memory.getb(self.regs.pc) as usize];
        let env = ExecEnv::new(opcode.cycles);
        0
    }

    pub fn exec_end(&mut self) {
        self.state.try_stop_running();
    }

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

    pub fn inst_adc(&mut self, env: &mut ExecEnv) {}
    pub fn inst_sbc(&mut self, env: &mut ExecEnv) {}
    pub fn inst_and(&mut self, env: &mut ExecEnv) {}
    pub fn inst_ora(&mut self, env: &mut ExecEnv) {}
    pub fn inst_asl(&mut self, env: &mut ExecEnv) {}
    pub fn inst_lsr(&mut self, env: &mut ExecEnv) {}
    pub fn inst_eor(&mut self, env: &mut ExecEnv) {}
    pub fn inst_rol(&mut self, env: &mut ExecEnv) {}
    pub fn inst_ror(&mut self, env: &mut ExecEnv) {}
    pub fn inst_bit(&mut self, env: &mut ExecEnv) {}
    pub fn inst_cmp(&mut self, env: &mut ExecEnv) {}
    pub fn inst_cpx(&mut self, env: &mut ExecEnv) {}
    pub fn inst_cpy(&mut self, env: &mut ExecEnv) {}
    pub fn inst_inc(&mut self, env: &mut ExecEnv) {}
    pub fn inst_inx(&mut self, env: &mut ExecEnv) {}
    pub fn inst_iny(&mut self, env: &mut ExecEnv) {}
    pub fn inst_dec(&mut self, env: &mut ExecEnv) {}
    pub fn inst_dex(&mut self, env: &mut ExecEnv) {}
    pub fn inst_dey(&mut self, env: &mut ExecEnv) {}
    pub fn inst_bcc(&mut self, env: &mut ExecEnv) {}
    pub fn inst_bcs(&mut self, env: &mut ExecEnv) {}
    pub fn inst_beq(&mut self, env: &mut ExecEnv) {}
    pub fn inst_bmi(&mut self, env: &mut ExecEnv) {}
    pub fn inst_bne(&mut self, env: &mut ExecEnv) {}
    pub fn inst_bpl(&mut self, env: &mut ExecEnv) {}
    pub fn inst_bvc(&mut self, env: &mut ExecEnv) {}
    pub fn inst_bvs(&mut self, env: &mut ExecEnv) {}
    pub fn inst_clc(&mut self, env: &mut ExecEnv) {}
    pub fn inst_cld(&mut self, env: &mut ExecEnv) {}
    pub fn inst_cli(&mut self, env: &mut ExecEnv) {}
    pub fn inst_clv(&mut self, env: &mut ExecEnv) {}
    pub fn inst_sec(&mut self, env: &mut ExecEnv) {}
    pub fn inst_sed(&mut self, env: &mut ExecEnv) {}
    pub fn inst_sei(&mut self, env: &mut ExecEnv) {}
    pub fn inst_jmp(&mut self, env: &mut ExecEnv) {}
    pub fn inst_jsr(&mut self, env: &mut ExecEnv) {}
    pub fn inst_brk(&mut self, env: &mut ExecEnv) {}
    pub fn inst_rti(&mut self, env: &mut ExecEnv) {}
    pub fn inst_rts(&mut self, env: &mut ExecEnv) {}
    pub fn inst_lda(&mut self, env: &mut ExecEnv) {}
    pub fn inst_ldx(&mut self, env: &mut ExecEnv) {}
    pub fn inst_ldy(&mut self, env: &mut ExecEnv) {}
    pub fn inst_sta(&mut self, env: &mut ExecEnv) {}
    pub fn inst_stx(&mut self, env: &mut ExecEnv) {}
    pub fn inst_sty(&mut self, env: &mut ExecEnv) {}
    pub fn inst_tax(&mut self, env: &mut ExecEnv) {}
    pub fn inst_tay(&mut self, env: &mut ExecEnv) {}
    pub fn inst_tsx(&mut self, env: &mut ExecEnv) {}
    pub fn inst_txa(&mut self, env: &mut ExecEnv) {}
    pub fn inst_tya(&mut self, env: &mut ExecEnv) {}
    pub fn inst_txs(&mut self, env: &mut ExecEnv) {}
    pub fn inst_pla(&mut self, env: &mut ExecEnv) {}
    pub fn inst_plp(&mut self, env: &mut ExecEnv) {}
    pub fn inst_pha(&mut self, env: &mut ExecEnv) {}
    pub fn inst_php(&mut self, env: &mut ExecEnv) {}
    pub fn inst_nop(&mut self, env: &mut ExecEnv) {}
    pub fn inst_kil(&mut self, env: &mut ExecEnv) {}
}

#[cfg(test)]
mod tests {
    use crate::mos6510::memory::RESET_VECTOR;

    use super::*;

    #[test]
    fn test_init() {
        let mut memory = Memory::new();
        let pc = memory.getw(RESET_VECTOR);
        let mut cpu = Cpu::new(pc);
        assert_eq!(cpu.regs.pc, pc);
        assert_eq!(cpu.state, CpuState::new());
    }
}
