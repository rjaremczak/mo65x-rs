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
