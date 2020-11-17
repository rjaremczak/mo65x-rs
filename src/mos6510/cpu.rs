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
mod tests {
    use super::*;
    use crate::mos6510::{assembler::Assembler, error::AsmError, memory::RESET_VECTOR};

    struct Ctx {
        cpu: Cpu,
        memory: Memory,
    }

    impl Ctx {
        fn new() -> Self {
            Self {
                memory: Memory::new(),
                cpu: Cpu::new(1000),
            }
        }

        fn from_cam(c: u8, a: u8, m: u8) -> Self {
            let mut ctx = Self::new();
            ctx.cpu.flags.c = c != 0;
            ctx.cpu.regs.a = a;
            ctx.memory[0x2000] = m;
            ctx
        }

        fn assert_inst(&mut self, line: &str, size: usize, cycles: u8) {
            let mut asm = Assembler::new(self.cpu.regs.pc);
            asm.generate_code(true);
            let r = asm.process_line(line);
            assert!(matches!(r, AsmError::Ok), "line \"{}\" : {:?}", line, r);
            assert_eq!(asm.object_code().data.len(), size);
            self.memory.set_bytes(asm.object_code().origin, &asm.object_code().data);
            let c = self.cpu.exec_inst(&mut self.memory);
            assert_eq!(c, cycles, "wrong number of cycles");
        }

        fn assert_anzcv(&self, a: u8, n: u8, z: u8, c: u8, v: u8) {
            assert_eq!(self.cpu.regs.a, a, "reg a");
            assert_eq!(self.cpu.flags.n, n != 0, "flag n");
            assert_eq!(self.cpu.flags.z, z != 0, "flag z");
            assert_eq!(self.cpu.flags.c, c != 0, "flag c");
            assert_eq!(self.cpu.flags.v, v != 0, "flag v");
        }
    }

    #[test]
    fn test_init() {
        let memory = Memory::new();
        let pc = memory.word(RESET_VECTOR);
        let cpu = Cpu::new(pc);
        assert_eq!(cpu.regs.pc, pc);
    }

    #[test]
    fn test_adc() {
        let mut ctx = Ctx::from_cam(1, 0x2e, 0x01);
        ctx.assert_inst("ADC $2000", 3, 4);
        ctx.assert_anzcv(0x30, 0, 0, 0, 0);
    }

    #[test]
    fn test_sbc() {
        let mut ctx = Ctx::from_cam(1, 0x80, 0);
        ctx.assert_inst("SBC #$82", 2, 2);
        ctx.assert_anzcv(-2i8 as u8, 1, 0, 0, 0);

        let mut ctx = Ctx::from_cam(1, 0x04, 0x04);
        assert_eq!(ctx.memory[0x2000], 0x04);
        ctx.assert_inst("SBC $2000", 3, 4);
        ctx.assert_anzcv(0, 0, 1, 1, 0);
    }

    #[test]
    fn test_and() {
        let mut ctx = Ctx::from_cam(0, 0x84, 0);
        ctx.assert_inst("AND #$fb", 2, 2);
        ctx.assert_anzcv(0x80, 1, 0, 0, 0);

        ctx.cpu.regs.a = 0x84;
        ctx.cpu.regs.y = 0x12;
        ctx.memory.set_word(0x70, 0x20f0);
        ctx.memory[0x2102] = 0xfb;
        ctx.assert_inst("AND ($70),Y", 2, 6);
        ctx.assert_anzcv(0x80, 1, 0, 0, 0);
    }

    #[test]
    fn test_ora() {
        let mut ctx = Ctx::new();
        ctx.cpu.regs.a = 0b11000001;
        ctx.assert_inst("ORA #$02", 2, 2);
        ctx.assert_anzcv(0xc3, 1, 0, 0, 0);

        ctx.cpu.regs.a = 0b01000000;
        ctx.assert_inst("ORA #$23", 2, 2);
        ctx.assert_anzcv(0x63, 0, 0, 0, 0);

        ctx.cpu.regs.a = 0;
        ctx.assert_inst("ORA #$0", 2, 2);
        ctx.assert_anzcv(0, 0, 1, 0, 0);
    }

    #[test]
    fn test_eor() {
        let mut ctx = Ctx::from_cam(0, 0b11011110, 0b01001101);
        ctx.assert_inst("EOR $2000", 3, 4);
        ctx.assert_anzcv(0b10010011, 1, 0, 0, 0);

        ctx.cpu.regs.a = 0b01001101;
        ctx.memory[0x21] = 0b01001101;
        ctx.assert_inst("EOR $21", 2, 3);
        ctx.assert_anzcv(0, 0, 1, 0, 0);
    }

    #[test]
    fn test_cmp() {}

    #[test]
    fn test_cpx() {}

    #[test]
    fn test_cpy() {}

    #[test]
    fn test_asl() {}

    #[test]
    fn test_lsr() {}

    #[test]
    fn test_rol() {}

    #[test]
    fn test_ror() {}

    #[test]
    fn test_bit() {}

    #[test]
    fn test_inc() {}

    #[test]
    fn test_inx() {}

    #[test]
    fn test_iny() {}

    #[test]
    fn test_dec() {}

    #[test]
    fn test_dex() {}

    #[test]
    fn test_dey() {}

    #[test]
    fn test_bcc() {}

    #[test]
    fn test_bcs() {}

    #[test]
    fn test_beq() {}

    #[test]
    fn test_bmi() {}

    #[test]
    fn test_bne() {}

    #[test]
    fn test_bpl() {}

    #[test]
    fn test_bvc() {}

    #[test]
    fn test_bvs() {}

    #[test]
    fn test_clc() {}

    #[test]
    fn test_cld() {}

    #[test]
    fn test_cli() {}

    #[test]
    fn test_clv() {}

    #[test]
    fn test_sec() {}

    #[test]
    fn test_sed() {}

    #[test]
    fn test_sei() {}

    #[test]
    fn test_jmp() {}

    #[test]
    fn test_jsr() {}

    #[test]
    fn test_brk() {}

    #[test]
    fn test_rti() {}

    #[test]
    fn test_rts() {}

    #[test]
    fn test_lda() {}

    #[test]
    fn test_ldx() {}

    #[test]
    fn test_ldy() {}

    #[test]
    fn test_sta() {}

    #[test]
    fn test_stx() {}

    #[test]
    fn test_sty() {}

    #[test]
    fn test_tax() {}

    #[test]
    fn test_tay() {}

    #[test]
    fn test_tsx() {}

    #[test]
    fn test_txa() {}

    #[test]
    fn test_tya() {}

    #[test]
    fn test_txs() {}

    #[test]
    fn test_pla() {}

    #[test]
    fn test_plp() {}

    #[test]
    fn test_pha() {}

    #[test]
    fn test_php() {}

    #[test]
    fn test_nop() {}

    #[test]
    fn test_kil() {}
}
