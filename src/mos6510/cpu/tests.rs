use super::*;
use crate::mos6510::{assembler::Assembler, error::AsmError};

struct Ctx {
    cpu: Cpu,
    memory: Memory,
}

impl Ctx {
    fn new() -> Self {
        Self {
            memory: Memory::new(),
            cpu: Cpu::new(),
        }
    }

    fn with_cam(c: u8, a: u8, m: u8) -> Self {
        let mut ctx = Self::new();
        ctx.cpu.flags.c = c != 0;
        ctx.cpu.regs.a = a;
        ctx.memory[0x2000] = m;
        ctx
    }

    fn assert_inst(&mut self, line: &str, cycles: u8) {
        let mut asm = Assembler::new(self.cpu.regs.pc);
        asm.generate_code(true);
        let r = asm.process_line(line);
        assert!(matches!(r, AsmError::Ok), "line \"{}\" : {:?}", line, r);
        self.memory.set_bytes(asm.object_code().origin, &asm.object_code().data);
        let c = self.cpu.exec_inst(&mut self.memory);
        assert_eq!(c, cycles, "wrong number of cycles");
    }

    fn assert_nzcv(&self, n: u8, z: u8, c: u8, v: u8) {
        assert_eq!(self.cpu.flags.n, n != 0, "flag n");
        assert_eq!(self.cpu.flags.z, z != 0, "flag z");
        assert_eq!(self.cpu.flags.c, c != 0, "flag c");
        assert_eq!(self.cpu.flags.v, v != 0, "flag v");
    }

    fn assert_anzcv(&self, a: u8, n: u8, z: u8, c: u8, v: u8) {
        assert_eq!(self.cpu.regs.a, a, "reg a");
        self.assert_nzcv(n, z, c, v);
    }
}

#[test]
fn test_reset() {
    let mut memory = Memory::new();
    memory.set_word(super::Cpu::RESET_VECTOR, 0x234a);
    let mut cpu = Cpu::new();
    cpu.reset(&memory);
    assert_eq!(cpu.regs.pc, 0x234a);
    assert_eq!(cpu.regs.sp, Cpu::SP_INIT);
}

#[test]
fn test_irq() {
    let mut memory = Memory::new();
    memory.set_word(super::Cpu::IRQ_VECTOR, 0xabcd);
    let mut cpu = Cpu::new();
    cpu.reset(&memory);
    cpu.flags = Flags::from_byte(0b11001111);
    let sp0 = cpu.regs.sp_address();
    let pc0 = cpu.regs.pc;
    cpu.irq(&mut memory);
    assert!(cpu.flags.i);
    assert_eq!(memory[cpu.regs.sp_address() + 1], 0b11001111);
    assert_eq!(memory[cpu.regs.sp_address() + 2], pc0 as u8);
    assert_eq!(memory[cpu.regs.sp_address() + 3], (pc0 >> 8) as u8);
    assert_eq!(cpu.regs.sp_address(), sp0 - 3);
    assert_eq!(cpu.regs.pc, 0xabcd);
}

#[test]
fn test_nmi() {
    let mut memory = Memory::new();
    memory.set_word(super::Cpu::NMI_VECTOR, 0xbcfa);
    let mut cpu = Cpu::new();
    cpu.reset(&memory);
    cpu.nmi(&mut memory);
    assert!(cpu.flags.i);
    assert_eq!(cpu.regs.pc, 0xbcfa);
}

#[test]
fn test_adc() {
    let mut ctx = Ctx::with_cam(1, 0x2e, 0x01);
    ctx.assert_inst("ADC $2000", 4);
    ctx.assert_anzcv(0x30, 0, 0, 0, 0);
}

#[test]
fn test_sbc() {
    let mut ctx = Ctx::with_cam(1, 0x80, 0);
    ctx.assert_inst("SBC #$82", 2);
    ctx.assert_anzcv(-2i8 as u8, 1, 0, 0, 0);

    let mut ctx = Ctx::with_cam(1, 0x04, 0x04);
    assert_eq!(ctx.memory[0x2000], 0x04);
    ctx.assert_inst("SBC $2000", 4);
    ctx.assert_anzcv(0, 0, 1, 1, 0);
}

#[test]
fn test_and() {
    let mut ctx = Ctx::with_cam(0, 0x84, 0);
    ctx.assert_inst("AND #$fb", 2);
    ctx.assert_anzcv(0x80, 1, 0, 0, 0);

    ctx.cpu.regs.a = 0x84;
    ctx.cpu.regs.y = 0x12;
    ctx.memory.set_word(0x70, 0x20f0);
    ctx.memory[0x2102] = 0xfb;
    ctx.assert_inst("AND ($70),Y", 6);
    ctx.assert_anzcv(0x80, 1, 0, 0, 0);
}

#[test]
fn test_ora() {
    let mut ctx = Ctx::new();
    ctx.cpu.regs.a = 0b11000001;
    ctx.assert_inst("ORA #$02", 2);
    ctx.assert_anzcv(0xc3, 1, 0, 0, 0);

    ctx.cpu.regs.a = 0b01000000;
    ctx.assert_inst("ORA #$23", 2);
    ctx.assert_anzcv(0x63, 0, 0, 0, 0);

    ctx.cpu.regs.a = 0;
    ctx.assert_inst("ORA #$0", 2);
    ctx.assert_anzcv(0, 0, 1, 0, 0);
}

#[test]
fn test_eor() {
    let mut ctx = Ctx::with_cam(0, 0b11011110, 0b01001101);
    ctx.assert_inst("EOR $2000", 4);
    ctx.assert_anzcv(0b10010011, 1, 0, 0, 0);

    ctx.cpu.regs.a = 0b01001101;
    ctx.memory[0x21] = 0b01001101;
    ctx.assert_inst("EOR $21", 3);
    ctx.assert_anzcv(0, 0, 1, 0, 0);
}

#[test]
fn test_cmp() {
    let mut ctx = Ctx::new();
    ctx.cpu.regs.a = 0x81;
    ctx.assert_inst("CMP #$80", 2);
    ctx.assert_anzcv(0x81, 0, 0, 1, 0);

    ctx.cpu.regs.a = 0x71;
    ctx.assert_inst("CMP #$90", 2);
    ctx.assert_anzcv(0x71, 1, 0, 0, 0);

    ctx.cpu.regs.a = 0x01;
    ctx.assert_inst("CMP #$01", 2);
    ctx.assert_anzcv(0x01, 0, 1, 1, 0);

    ctx.cpu.regs.a = -100i8 as u8;
    ctx.memory[0x2000] = -110i8 as u8;
    ctx.assert_inst("CMP $2000", 4);
    ctx.assert_anzcv(-100i8 as u8, 0, 0, 1, 0);

    ctx.cpu.regs.a = 150;
    ctx.memory[0x2000] = 120;
    ctx.assert_inst("CMP $2000", 4);
    ctx.assert_anzcv(150, 0, 0, 1, 0);
}

#[test]
fn test_cpx() {
    let mut ctx = Ctx::new();
    ctx.cpu.regs.x = -100i8 as u8;
    ctx.memory[0x2000] = -110i8 as u8;
    ctx.assert_inst("CPX $2000", 4);
    ctx.assert_nzcv(0, 0, 1, 0);
    assert_eq!(ctx.cpu.regs.x, -100i8 as u8);

    ctx.cpu.regs.x = 150;
    ctx.memory[0x20] = 120;
    ctx.assert_inst("CPX $20", 3);
    ctx.assert_nzcv(0, 0, 1, 0);
    assert_eq!(ctx.cpu.regs.x, 150);
}

#[test]
fn test_cpy() {
    let mut ctx = Ctx::new();
    ctx.cpu.regs.y = 0x71;
    ctx.assert_inst("CPY #$90", 2);
    ctx.assert_nzcv(1, 0, 0, 0);
    assert_eq!(ctx.cpu.regs.y, 0x71);

    ctx.cpu.regs.y = -100i8 as u8;
    ctx.memory[0x2000] = -110i8 as u8;
    ctx.assert_inst("CPY $2000", 4);
    ctx.assert_nzcv(0, 0, 1, 0);
    assert_eq!(ctx.cpu.regs.y, -100i8 as u8);
}

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
fn test_clc() {
    let mut ctx = Ctx::new();
    ctx.cpu.flags.c = true;
    ctx.assert_inst("CLC", 2);
    assert_eq!(ctx.cpu.flags.c, false);
}

#[test]
fn test_cld() {
    let mut ctx = Ctx::new();
    ctx.cpu.flags.d = true;
    ctx.assert_inst("CLD", 2);
    assert_eq!(ctx.cpu.flags.d, false);
}

#[test]
fn test_cli() {
    let mut ctx = Ctx::new();
    ctx.cpu.flags.i = true;
    ctx.assert_inst("CLI", 2);
    assert_eq!(ctx.cpu.flags.i, false);
}

#[test]
fn test_clv() {
    let mut ctx = Ctx::new();
    ctx.cpu.flags.v = true;
    ctx.assert_inst("CLV", 2);
    assert_eq!(ctx.cpu.flags.v, false);
}

#[test]
fn test_sec() {
    let mut ctx = Ctx::new();
    ctx.cpu.flags.c = false;
    ctx.assert_inst("SEC", 2);
    assert_eq!(ctx.cpu.flags.c, true);
}

#[test]
fn test_sed() {
    let mut ctx = Ctx::new();
    ctx.cpu.flags.d = false;
    ctx.assert_inst("SED", 2);
    assert_eq!(ctx.cpu.flags.d, true);
}

#[test]
fn test_sei() {
    let mut ctx = Ctx::new();
    ctx.cpu.flags.i = false;
    ctx.assert_inst("SEI", 2);
    assert_eq!(ctx.cpu.flags.i, true);
}

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
