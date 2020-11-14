mod decoder;
mod env;
mod flags;
mod registers;

use self::{env::Env, flags::Flags, registers::Registers};
use super::{memory::Memory, opcode::OPCODES};
use decoder::*;

pub struct Cpu {
    regs: Registers,
    flags: Flags,
    decode_table: OpCodeTable,
}

impl Cpu {
    pub fn new(pc: u16) -> Self {
        Self {
            regs: Registers::new(pc, 0xfd),
            flags: Flags::new(),
            decode_table: generate_opcode_table(),
        }
    }

    pub fn prep_implied(&mut self, env: &mut Env, b: bool) {}

    pub fn exec_test(&mut self, env: &mut Env) {}

    pub fn exec_instruction(&mut self, memory: &mut Memory) -> u8 {
        let opcode = &OPCODES[memory[self.regs.pc] as usize];
        let opcode_entry = self.decode_table[opcode.code as usize];
        // let mut env = ExecEnv::new(memory, self.regs.pc, opcode.cycles);
        // let outref = (opcode_entry.prep_handler)(&mut env, &mut self.regs);
        // let mut env = self.prep_implied(memory, true);
        // (opcode_entry.exec_handler)(self, &mut env, memory, outref);
        // let mut dummy: u8 = 0;
        let mut env = Env::new(self.regs.pc.wrapping_add(1), opcode.cycles);
        env.prep_implied(memory, &mut self.regs);
        self.exec_test(&mut env);
        self.regs.pc = self.regs.pc.wrapping_add(opcode_entry.size as u16);
        0
    }

    pub fn exec_adc(&mut self, env: &mut Env, _: &mut Memory) {
        let mut result = self.regs.a as u16 + env.arg + self.flags.c as u16;
        if self.flags.d {
            self.flags.c = decimal_correction(&mut result);
            self.flags.compute_nz(result);
        } else {
            self.flags.compute_nzc(result);
        }
        self.flags.compute_v(self.regs.a as u16, env.arg, result);
        self.regs.a = result as u8;
        env.add_cycle_when_page_crossed();
    }

    pub fn exec_sbc(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_and(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_ora(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_asl(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_lsr(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_eor(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_rol(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_ror(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_bit(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_cmp(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_cpx(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_cpy(&mut self, env: &mut Env, memory: &mut Memory) {}

    pub fn exec_inc(&mut self, env: &mut Env, memory: &mut Memory, outref: &mut u8) {
        let result = *outref as u16 + 1;
        *outref = result as u8;
        self.flags.compute_nz(result);
    }

    pub fn exec_inx(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_iny(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_dec(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_dex(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_dey(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_bcc(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_bcs(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_beq(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_bmi(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_bne(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_bpl(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_bvc(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_bvs(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_clc(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_cld(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_cli(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_clv(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_sec(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_sed(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_sei(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_jmp(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_jsr(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_brk(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_rti(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_rts(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_lda(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_ldx(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_ldy(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_sta(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_stx(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_sty(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_tax(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_tay(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_tsx(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_txa(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_tya(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_txs(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_pla(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_plp(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_pha(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_php(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_nop(&mut self, env: &mut Env, memory: &mut Memory, _: &mut u8) {}
    pub fn exec_kil(&mut self, env: &mut Env, memory: &mut Memory, _: &mut u8) {}
}

fn decimal_correction(result: &mut u16) -> bool {
    if (*result & 0x0f) > 0x09 {
        *result += 0x06;
    }
    if (*result & 0xf0) > 0x90 {
        *result += 0x60;
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use crate::mos6510::memory::RESET_VECTOR;

    use super::*;

    #[test]
    fn test_init() {
        let mut memory = Memory::new();
        let pc = memory.word(RESET_VECTOR);
        let mut cpu = Cpu::new(pc);
        assert_eq!(cpu.regs.pc, pc);
    }
}
