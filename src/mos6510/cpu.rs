mod decoder;
mod env;
mod flags;
mod registers;

use self::{env::Env, flags::Flags, registers::Registers};
use super::memory::Memory;
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

    pub fn exec_inst(&mut self, memory: &mut Memory) -> u8 {
        let opcode = memory[self.regs.pc];
        let entry = self.decode_table[opcode as usize];
        let mut env = Env::new(self.regs.pc.wrapping_add(1), entry.cycles);
        (entry.prep_handler)(&mut env, memory, &mut self.regs);
        (entry.exec_handler)(self, &mut env, memory);
        self.regs.pc = self.regs.pc.wrapping_add(entry.size as u16);
        env.cycles
    }

    pub fn exec_adc(&mut self, env: &mut Env, _: &mut Memory) {
        let mut result = self.regs.a as u16 + env.arg() as u16 + self.flags.c as u16;
        if self.flags.d {
            self.flags.c = decimal_correction(&mut result);
            self.flags.compute_nz(result);
        } else {
            self.flags.compute_nzc(result);
        }
        self.flags.compute_v(self.regs.a as u16, env.addr, result);
        self.regs.a = result as u8;
        env.add_cycle_when_page_crossed();
    }

    pub fn exec_sbc(&mut self, env: &mut Env, _: &mut Memory) {
        let op = env.arg() as u16 ^ 0x00ff;
        let mut result = self.regs.a as u16 + op + self.flags.c as u16;
        if self.flags.d {
            result -= 0x66;
            self.flags.c = decimal_correction(&mut result);
            self.flags.compute_nz(result);
        } else {
            self.flags.compute_nzc(result);
        }
        self.flags.compute_v(self.regs.a as u16, op, result);
        self.regs.a = result as u8;
        env.add_cycle_when_page_crossed();
    }

    pub fn exec_and(&mut self, env: &mut Env, _: &mut Memory) {
        self.regs.a &= env.arg();
        self.flags.compute_nz(self.regs.a as u16);
        env.add_cycle_when_page_crossed();
    }

    pub fn exec_ora(&mut self, env: &mut Env, _: &mut Memory) {
        self.regs.a |= env.arg();
        self.flags.compute_nz(self.regs.a as u16);
        env.add_cycle_when_page_crossed();
    }

    pub fn exec_eor(&mut self, env: &mut Env, _: &mut Memory) {
        self.regs.a ^= env.arg();
        self.flags.compute_nz(self.regs.a as u16);
        env.add_cycle_when_page_crossed();
    }

    pub fn exec_cmp(&mut self, env: &mut Env, _: &mut Memory) {
        self.flags.compute_nzc(self.regs.a as u16 + (env.arg() as u16 ^ 0xff) + 1);
        env.add_cycle_when_page_crossed();
    }

    pub fn exec_asl(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_lsr(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_rol(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_ror(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_bit(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_cpx(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_cpy(&mut self, env: &mut Env, memory: &mut Memory) {}

    pub fn exec_inc(&mut self, env: &mut Env, _: &mut Memory) {
        let result = env.arg().wrapping_add(1);
        env.set_arg(result);
        self.flags.compute_nz(result as u16);
    }

    pub fn exec_inx(&mut self, _: &mut Env, _: &mut Memory) {
        self.regs.x = self.regs.x.wrapping_add(1);
        self.flags.compute_nz(self.regs.x as u16);
    }

    pub fn exec_iny(&mut self, _: &mut Env, _: &mut Memory) {
        self.regs.y = self.regs.y.wrapping_add(1);
        self.flags.compute_nz(self.regs.y as u16);
    }

    pub fn exec_dec(&mut self, env: &mut Env, _: &mut Memory) {
        let result = env.arg().wrapping_sub(1);
        env.set_arg(result);
        self.flags.compute_nz(result as u16);
    }

    pub fn exec_dex(&mut self, _: &mut Env, _: &mut Memory) {
        self.regs.x = self.regs.x.wrapping_sub(1);
        self.flags.compute_nz(self.regs.x as u16);
    }

    pub fn exec_dey(&mut self, _: &mut Env, _: &mut Memory) {
        self.regs.y = self.regs.y.wrapping_sub(1);
        self.flags.compute_nz(self.regs.y as u16);
    }

    pub fn exec_bcc(&mut self, env: &mut Env, _: &mut Memory) {
        if !self.flags.c {
            self.exec_branch(env)
        }
    }

    pub fn exec_bcs(&mut self, env: &mut Env, _: &mut Memory) {
        if self.flags.c {
            self.exec_branch(env)
        }
    }

    pub fn exec_beq(&mut self, env: &mut Env, _: &mut Memory) {
        if self.flags.z {
            self.exec_branch(env)
        }
    }

    pub fn exec_bmi(&mut self, env: &mut Env, _: &mut Memory) {
        if self.flags.n {
            self.exec_branch(env)
        }
    }

    pub fn exec_bne(&mut self, env: &mut Env, _: &mut Memory) {
        if !self.flags.z {
            self.exec_branch(env)
        }
    }

    pub fn exec_bpl(&mut self, env: &mut Env, _: &mut Memory) {
        if !self.flags.n {
            self.exec_branch(env)
        }
    }

    pub fn exec_bvc(&mut self, env: &mut Env, _: &mut Memory) {
        if !self.flags.v {
            self.exec_branch(env)
        }
    }

    pub fn exec_bvs(&mut self, env: &mut Env, _: &mut Memory) {
        if self.flags.v {
            self.exec_branch(env)
        }
    }

    pub fn exec_clc(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_cld(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_cli(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_clv(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_sec(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_sed(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_sei(&mut self, env: &mut Env, memory: &mut Memory) {}

    pub fn exec_jmp(&mut self, env: &mut Env, _: &mut Memory) {
        self.regs.pc = env.addr;
    }

    pub fn exec_jsr(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_brk(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_rti(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_rts(&mut self, env: &mut Env, memory: &mut Memory) {}

    pub fn exec_lda(&mut self, env: &mut Env, _: &mut Memory) {
        self.regs.a = env.arg();
        self.flags.compute_nz(self.regs.a as u16);
        env.add_cycle_when_page_crossed();
    }

    pub fn exec_ldx(&mut self, env: &mut Env, _: &mut Memory) {
        self.regs.x = env.arg();
        self.flags.compute_nz(self.regs.x as u16);
        env.add_cycle_when_page_crossed();
    }

    pub fn exec_ldy(&mut self, env: &mut Env, _: &mut Memory) {
        self.regs.y = env.arg();
        self.flags.compute_nz(self.regs.y as u16);
        env.add_cycle_when_page_crossed();
    }

    pub fn exec_sta(&mut self, env: &mut Env, _: &mut Memory) {
        env.set_arg(self.regs.a);
    }

    pub fn exec_stx(&mut self, env: &mut Env, _: &mut Memory) {
        env.set_arg(self.regs.x);
    }

    pub fn exec_sty(&mut self, env: &mut Env, _: &mut Memory) {
        env.set_arg(self.regs.y);
    }

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
    pub fn exec_nop(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_kil(&mut self, env: &mut Env, memory: &mut Memory) {}

    #[inline]
    fn exec_branch(&mut self, env: &mut Env) {
        env.cycles += 1;
        let base = self.regs.pc;
        self.regs.pc = (base as i32 + (env.arg() as i8) as i32) as u16;
        env.update_page_crossed(base, self.regs.pc);
        env.add_cycle_when_page_crossed();
    }

    fn push(&mut self, env: &mut Env, memory: &mut Memory, b: u8) {
        memory[self.regs.sp_address()] = b;
        self.regs.sp = self.regs.sp.wrapping_sub(1);
    }

    fn pull(&mut self, env: &mut Env, memory: &mut Memory) -> u8 {
        self.regs.sp = self.regs.sp.wrapping_sub(1);
        memory[self.regs.sp_address()]
    }
}

#[inline]
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
mod tests;
