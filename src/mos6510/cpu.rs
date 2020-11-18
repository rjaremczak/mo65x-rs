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
    pub const IO_PORT_CONFIG: u16 = 0x0000;
    pub const IO_PORT_DATA: u16 = 0x0001;
    pub const NMI_VECTOR: u16 = 0xfffa;
    pub const RESET_VECTOR: u16 = 0xfffc;
    pub const IRQ_VECTOR: u16 = 0xfffe;
    pub const SP_BASE: u16 = 0x0100;
    pub const SP_INIT: u8 = 0xfd;

    pub fn new() -> Self {
        Self {
            regs: Registers::new(),
            flags: Flags::new(),
            decode_table: generate_opcode_table(),
        }
    }

    pub fn reset(&mut self, memory: &Memory) {
        self.regs = Registers::new();
        self.regs.pc = memory.word(Cpu::RESET_VECTOR);
        self.regs.sp = Cpu::SP_INIT;
        self.flags = Flags::new();
    }

    #[inline]
    fn general_irq(&mut self, memory: &mut Memory, pc: u16, flags: u8, vector: u16) {
        self.push_word(memory, pc);
        self.push(memory, flags);
        self.flags.i = true;
        self.regs.pc = memory.word(vector);
    }

    pub fn irq(&mut self, memory: &mut Memory) {
        self.general_irq(memory, self.regs.pc, self.flags.to_byte(), Cpu::IRQ_VECTOR);
    }

    pub fn nmi(&mut self, memory: &mut Memory) {
        self.general_irq(memory, self.regs.pc, self.flags.to_byte(), Cpu::NMI_VECTOR);
    }

    pub fn exec_brk(&mut self, _: &mut Env, memory: &mut Memory) {
        self.general_irq(memory, self.regs.pc + 1, self.flags.to_byte() | Flags::BM_BREAK, Cpu::IRQ_VECTOR);
    }

    pub fn exec_inst(&mut self, memory: &mut Memory) -> u8 {
        let opcode = memory[self.regs.pc];
        let entry = self.decode_table[opcode as usize];
        let mut env = Env::with(self.regs.pc + 1, entry.cycles);
        (entry.prep_handler)(&mut env, memory, &mut self.regs);
        (entry.exec_handler)(self, &mut env, memory);
        self.regs.pc = self.regs.pc + entry.size as u16;
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

    pub fn exec_asl(&mut self, env: &mut Env, _: &mut Memory) {
        let tmp = (env.arg() as u16) << 1;
        self.flags.compute_nzc(tmp);
        env.set_arg(tmp as u8);
    }

    pub fn exec_lsr(&mut self, env: &mut Env, _: &mut Memory) {
        let mut tmp = env.arg();
        self.flags.c = tmp & 0x01 != 0;
        tmp >>= 1;
        self.flags.compute_nz(tmp as u16);
        env.set_arg(tmp);
    }

    pub fn exec_rol(&mut self, env: &mut Env, _: &mut Memory) {
        let tmp = (env.arg() as u16) << 1 | self.flags.c as u16;
        self.flags.compute_nzc(tmp);
        env.set_arg(tmp as u8);
    }

    pub fn exec_ror(&mut self, env: &mut Env, _: &mut Memory) {
        let mut tmp = env.arg() as u16 | if self.flags.c { 0x100 } else { 0 };
        self.flags.c = tmp & 0x01 != 0;
        tmp >>= 1;
        self.flags.compute_nz(tmp);
        env.set_arg(tmp as u8);
    }

    pub fn exec_bit(&mut self, env: &mut Env, _: &mut Memory) {
        let tmp = env.arg();
        self.flags.z = (self.regs.a & tmp) == 0;
        self.flags.n = tmp & 0x80 != 0;
        self.flags.v = tmp & 0x40 != 0;
    }

    pub fn exec_cmp(&mut self, env: &mut Env, _: &mut Memory) {
        self.flags.compute_nzc(self.regs.a as u16 + (env.arg() as u16 ^ 0xff) + 1);
        env.add_cycle_when_page_crossed();
    }

    pub fn exec_cpx(&mut self, env: &mut Env, memory: &mut Memory) {
        self.flags.compute_nzc(self.regs.x as u16 + (env.arg() as u16 ^ 0xff) + 1);
    }

    pub fn exec_cpy(&mut self, env: &mut Env, memory: &mut Memory) {
        self.flags.compute_nzc(self.regs.y as u16 + (env.arg() as u16 ^ 0xff) + 1);
    }

    pub fn exec_inc(&mut self, env: &mut Env, _: &mut Memory) {
        let result = env.arg() + 1;
        env.set_arg(result);
        self.flags.compute_nz(result as u16);
    }

    pub fn exec_inx(&mut self, _: &mut Env, _: &mut Memory) {
        self.regs.x = self.regs.x + 1;
        self.flags.compute_nz(self.regs.x as u16);
    }

    pub fn exec_iny(&mut self, _: &mut Env, _: &mut Memory) {
        self.regs.y = self.regs.y + 1;
        self.flags.compute_nz(self.regs.y as u16);
    }

    pub fn exec_dec(&mut self, env: &mut Env, _: &mut Memory) {
        let result = env.arg() - 1;
        env.set_arg(result);
        self.flags.compute_nz(result as u16);
    }

    pub fn exec_dex(&mut self, _: &mut Env, _: &mut Memory) {
        self.regs.x = self.regs.x - 1;
        self.flags.compute_nz(self.regs.x as u16);
    }

    pub fn exec_dey(&mut self, _: &mut Env, _: &mut Memory) {
        self.regs.y = self.regs.y - 1;
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

    pub fn exec_clc(&mut self, _: &mut Env, _: &mut Memory) {
        self.flags.c = false;
    }

    pub fn exec_cld(&mut self, _: &mut Env, _: &mut Memory) {
        self.flags.d = false;
    }

    pub fn exec_cli(&mut self, _: &mut Env, _: &mut Memory) {
        self.flags.i = false;
    }

    pub fn exec_clv(&mut self, _: &mut Env, _: &mut Memory) {
        self.flags.v = false;
    }

    pub fn exec_sec(&mut self, _: &mut Env, _: &mut Memory) {
        self.flags.c = true;
    }

    pub fn exec_sed(&mut self, _: &mut Env, _: &mut Memory) {
        self.flags.d = true;
    }

    pub fn exec_sei(&mut self, _: &mut Env, _: &mut Memory) {
        self.flags.i = true;
    }

    pub fn exec_jmp(&mut self, env: &mut Env, _: &mut Memory) {
        self.regs.pc = env.addr;
    }

    pub fn exec_jsr(&mut self, env: &mut Env, memory: &mut Memory) {
        self.push_word(memory, self.regs.pc + 1);
        self.regs.pc = env.addr;
    }

    pub fn exec_rti(&mut self, env: &mut Env, memory: &mut Memory) {
        self.flags = Flags::from_byte(self.pull(memory));
        self.regs.pc = self.pull_word(memory);
        self.flags.i = false;
    }

    pub fn exec_rts(&mut self, env: &mut Env, memory: &mut Memory) {
        self.regs.pc = self.pull_word(memory) + 1;
    }

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

    pub fn exec_tax(&mut self, _: &mut Env, _: &mut Memory) {
        self.regs.x = self.regs.a;
        self.flags.compute_nz(self.regs.x as u16);
    }

    pub fn exec_txa(&mut self, _: &mut Env, _: &mut Memory) {
        self.regs.a = self.regs.x;
        self.flags.compute_nz(self.regs.a as u16);
    }

    pub fn exec_tay(&mut self, _: &mut Env, _: &mut Memory) {
        self.regs.y = self.regs.a;
        self.flags.compute_nz(self.regs.y as u16);
    }
    pub fn exec_tya(&mut self, _: &mut Env, _: &mut Memory) {
        self.regs.a = self.regs.y;
        self.flags.compute_nz(self.regs.a as u16);
    }

    pub fn exec_tsx(&mut self, _: &mut Env, _: &mut Memory) {
        self.regs.x = self.regs.sp;
        self.flags.compute_nz(self.regs.x as u16);
    }

    pub fn exec_txs(&mut self, _: &mut Env, _: &mut Memory) {
        self.regs.sp = self.regs.x;
    }

    pub fn exec_pla(&mut self, env: &mut Env, memory: &mut Memory) {
        self.regs.a = self.pull(memory);
        self.flags.compute_nz(self.regs.a as u16);
    }

    pub fn exec_plp(&mut self, env: &mut Env, memory: &mut Memory) {
        self.flags = Flags::from_byte(self.pull(memory));
    }

    pub fn exec_pha(&mut self, env: &mut Env, memory: &mut Memory) {
        self.push(memory, self.regs.a);
    }

    pub fn exec_php(&mut self, env: &mut Env, memory: &mut Memory) {
        self.push(memory, self.flags.to_byte());
    }

    pub fn exec_nop(&mut self, _: &mut Env, _: &mut Memory) {}
    pub fn exec_kil(&mut self, _: &mut Env, _: &mut Memory) {}

    #[inline]
    fn exec_branch(&mut self, env: &mut Env) {
        env.cycles += 1;
        let base = self.regs.pc;
        self.regs.pc = (base as i32 + (env.arg() as i8) as i32) as u16;
        env.update_page_crossed(base, self.regs.pc);
        env.add_cycle_when_page_crossed();
    }

    #[inline]
    fn push(&mut self, memory: &mut Memory, b: u8) {
        memory[self.regs.sp_address()] = b;
        self.regs.sp = self.regs.sp - 1;
    }

    #[inline]
    fn push_word(&mut self, memory: &mut Memory, word: u16) {
        self.push(memory, (word >> 8) as u8);
        self.push(memory, word as u8);
    }

    #[inline]
    fn pull(&mut self, memory: &Memory) -> u8 {
        self.regs.sp = self.regs.sp - 1;
        memory[self.regs.sp_address()]
    }

    #[inline]
    fn pull_word(&mut self, memory: &Memory) -> u16 {
        self.pull(memory) as u16 | (self.pull(memory) as u16) << 8
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
