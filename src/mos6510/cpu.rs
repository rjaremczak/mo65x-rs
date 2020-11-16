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

    pub fn exec_inc(&mut self, env: &mut Env, memory: &mut Memory) {
        let result = env.arg().wrapping_add(1);
        env.set_arg(result);
        self.flags.compute_nz(result as u16);
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
    pub fn exec_nop(&mut self, env: &mut Env, memory: &mut Memory) {}
    pub fn exec_kil(&mut self, env: &mut Env, memory: &mut Memory) {}
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
    use crate::mos6510::{assembler::Assembler, error::AsmError, memory::RESET_VECTOR};

    use super::*;

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

        fn assert_anzcv(&self, a: u8, n: bool, z: bool, c: bool, v: bool) {
            assert_eq!(self.cpu.regs.a, a, "reg a");
            assert_eq!(self.cpu.flags.n, n, "flag n");
            assert_eq!(self.cpu.flags.z, z, "flag z");
            assert_eq!(self.cpu.flags.c, c, "flag c");
            assert_eq!(self.cpu.flags.v, v, "flag v");
        }

        fn set_ca(&mut self, c: bool, a: u8) {
            self.cpu.flags.c = c;
            self.cpu.regs.a = a;
        }

        fn set_cam(&mut self, c: bool, a: u8, m: u8) {
            self.set_ca(c, a);
            self.memory[0x2000] = m;
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
        let mut ctx = Ctx::new();
        ctx.set_cam(true, 0x2e, 0x01);
        ctx.assert_inst("ADC $2000", 3, 4);
        ctx.assert_anzcv(0x30, false, false, false, false);
    }

    #[test]
    fn test_sbc() {
        let mut ctx = Ctx::new();
        ctx.set_ca(true, 0x80);
        ctx.assert_inst("SBC #$82", 2, 2);
        ctx.assert_anzcv(-2i8 as u8, true, false, false, false);

        ctx.set_cam(true, 0x04, 0x04);
        assert_eq!(ctx.memory[0x2000], 0x04);
        ctx.assert_inst("SBC $2000", 3, 4);
        ctx.assert_anzcv(0, false, true, true, false);
    }

    #[test]
    fn test_and() {}

    #[test]
    fn test_ora() {}

    #[test]
    fn test_asl() {}

    #[test]
    fn test_lsr() {}

    #[test]
    fn test_eor() {}

    #[test]
    fn test_rol() {}

    #[test]
    fn test_ror() {}

    #[test]
    fn test_bit() {}

    #[test]
    fn test_cmp() {}

    #[test]
    fn test_cpx() {}

    #[test]
    fn test_cpy() {}

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
