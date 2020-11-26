use super::*;
use crate::mos6510::assembler::Assembler;

struct Ctx {
    cpu: Cpu,
    memory: Memory,
}

impl Ctx {
    const PC_INIT: u16 = 0x1000;
    const MEM_ADR: u16 = 0x2000;

    fn new() -> Self {
        let mut ctx = Self {
            memory: Memory::new(),
            cpu: Cpu::new(),
        };
        ctx.cpu.regs.sp = Cpu::SP_INIT;
        ctx.cpu.regs.pc = Self::PC_INIT;
        ctx
    }

    fn with_cam(c: u8, a: u8, m: u8) -> Self {
        let mut ctx = Self::new();
        ctx.cpu.flags.c = c != 0;
        ctx.cpu.regs.a = a;
        ctx.memory[Self::MEM_ADR] = m;
        ctx
    }

    fn assert_inst(&mut self, line: &str, cycles: u8) {
        let mut asm = Assembler::new();
        asm.reset_phase(true);
        assert!(asm.set_location_counter(self.cpu.regs.pc).is_ok());
        let r = asm.process_line(line);
        assert!(r.is_ok(), "line \"{}\" : {:?}", line, r);
        self.memory.set_block(asm.origin(), asm.code());
        let c = self.cpu.exec_inst(&mut self.memory);
        assert_eq!(c, cycles, "wrong number of cycles");
    }

    fn assert_nz(&self, n: u8, z: u8) {
        assert_eq!(self.cpu.flags.n, n != 0, "flag n");
        assert_eq!(self.cpu.flags.z, z != 0, "flag z");
    }

    fn assert_nzc(&self, n: u8, z: u8, c: u8) {
        self.assert_nz(n, z);
        assert_eq!(self.cpu.flags.c, c != 0, "flag c");
    }

    fn assert_v(&self, v: u8) {
        assert_eq!(self.cpu.flags.v, v != 0, "flag v");
    }

    fn assert_nzcv(&self, n: u8, z: u8, c: u8, v: u8) {
        self.assert_nzc(n, z, c);
        self.assert_v(v);
    }

    fn assert_anzc(&self, a: u8, n: u8, z: u8, c: u8) {
        assert_eq!(self.cpu.regs.a, a, "reg a");
        self.assert_nzc(n, z, c);
    }

    fn assert_anzcv(&self, a: u8, n: u8, z: u8, c: u8, v: u8) {
        self.assert_anzc(a, n, z, c);
        assert_eq!(self.cpu.flags.v, v != 0, "flag v");
    }

    fn assert_xnzc(&self, x: u8, n: u8, z: u8, c: u8) {
        assert_eq!(self.cpu.regs.x, x, "reg x");
        self.assert_nzc(n, z, c);
    }

    fn assert_ynzc(&self, y: u8, n: u8, z: u8, c: u8) {
        assert_eq!(self.cpu.regs.y, y, "reg y");
        self.assert_nzc(n, z, c);
    }

    fn assert_memnz(&self, addr: u16, m: u8, n: u8, z: u8) {
        assert_eq!(self.memory[addr], m);
        self.assert_nz(n, z);
    }

    fn assert_branch_taken(&self, offset: i8) {
        assert_eq!(self.cpu.regs.pc, (Ctx::PC_INIT as i32 + 2 + offset as i32) as u16);
    }

    fn assert_branch_not_taken(&self) {
        assert_eq!(self.cpu.regs.pc, (Ctx::PC_INIT as i32 + 2) as u16);
    }
}

#[test]
fn test_reset() {
    let mut ctx = Ctx::new();
    ctx.memory.set_word(super::Cpu::RESET_VECTOR, 0x234a);
    ctx.cpu.reset(&ctx.memory);
    assert_eq!(ctx.cpu.regs.pc, 0x234a);
    assert_eq!(ctx.cpu.regs.sp, Cpu::SP_INIT);
}

#[test]
fn test_irq() {
    let mut ctx = Ctx::new();
    ctx.memory.set_word(super::Cpu::IRQ_VECTOR, 0xabcd);
    ctx.cpu.reset(&ctx.memory);
    ctx.cpu.flags = Flags::from_byte(0b11001111);
    let sp0 = ctx.cpu.regs.sp_address();
    let pc0 = ctx.cpu.regs.pc;
    ctx.cpu.irq(&mut ctx.memory);
    assert!(ctx.cpu.flags.i);
    assert_eq!(ctx.memory[ctx.cpu.regs.sp_address() + 1], 0b11001111);
    assert_eq!(ctx.memory[ctx.cpu.regs.sp_address() + 2], pc0 as u8);
    assert_eq!(ctx.memory[ctx.cpu.regs.sp_address() + 3], (pc0 >> 8) as u8);
    assert_eq!(ctx.cpu.regs.sp_address(), sp0 - 3);
    assert_eq!(ctx.cpu.regs.pc, 0xabcd);
}

#[test]
fn test_nmi() {
    let mut ctx = Ctx::new();
    ctx.memory.set_word(super::Cpu::NMI_VECTOR, 0xbcfa);
    ctx.cpu.reset(&ctx.memory);
    ctx.cpu.nmi(&mut ctx.memory);
    assert!(ctx.cpu.flags.i);
    assert_eq!(ctx.cpu.regs.pc, 0xbcfa);
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
    assert_eq!(ctx.memory[Ctx::MEM_ADR], 0x04);
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
    ctx.memory[Ctx::MEM_ADR] = -110i8 as u8;
    ctx.assert_inst("CMP $2000", 4);
    ctx.assert_anzcv(-100i8 as u8, 0, 0, 1, 0);

    ctx.cpu.regs.a = 150;
    ctx.memory[Ctx::MEM_ADR] = 120;
    ctx.assert_inst("CMP $2000", 4);
    ctx.assert_anzcv(150, 0, 0, 1, 0);
}

#[test]
fn test_cpx() {
    let mut ctx = Ctx::new();
    ctx.cpu.regs.x = -100i8 as u8;
    ctx.memory[Ctx::MEM_ADR] = -110i8 as u8;
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
    ctx.memory[Ctx::MEM_ADR] = -110i8 as u8;
    ctx.assert_inst("CPY $2000", 4);
    ctx.assert_nzcv(0, 0, 1, 0);
    assert_eq!(ctx.cpu.regs.y, -100i8 as u8);
}

#[test]
fn test_asl() {
    let mut ctx = Ctx::new();
    ctx.cpu.regs.a = 0b11000001;
    ctx.assert_inst("ASL", 2);
    ctx.assert_anzcv(0b10000010, 1, 0, 1, 0);

    ctx.memory[0xf002] = 0b01000001;
    ctx.cpu.regs.x = 0x12;
    ctx.assert_inst("ASL $eff0,X", 7);
    ctx.assert_anzcv(0b10000010, 1, 0, 0, 0);
}

#[test]
fn test_lsr() {
    let mut ctx = Ctx::new();
    ctx.cpu.regs.a = 0b11000001;
    ctx.assert_inst("LSR", 2);
    ctx.assert_anzcv(0b01100000, 0, 0, 1, 0);

    ctx.memory[0xf002] = 0b10000010;
    ctx.assert_inst("LSR $f002", 6);
    assert_eq!(ctx.memory[0xf002], 0b01000001);
    ctx.assert_nzcv(0, 0, 0, 0);

    ctx.memory[0xf0] = 1;
    ctx.assert_inst("LSR $f0", 5);
    assert_eq!(ctx.memory[0xf0], 0);
    ctx.assert_nzcv(0, 1, 1, 0);
}

#[test]
fn test_rol() {
    let mut ctx = Ctx::new();
    ctx.cpu.flags.c = true;
    ctx.cpu.regs.a = 0b11000001;
    ctx.assert_inst("ROL", 2);
    ctx.assert_anzcv(0b10000011, 1, 0, 1, 0);

    ctx.cpu.flags.c = false;
    ctx.cpu.regs.a = 0b01000001;
    ctx.assert_inst("ROL", 2);
    ctx.assert_anzcv(0b10000010, 1, 0, 0, 0);

    ctx.cpu.flags.c = false;
    ctx.cpu.regs.a = 0b10000000;
    ctx.assert_inst("ROL", 2);
    ctx.assert_anzcv(0, 0, 1, 1, 0);
}

#[test]
fn test_ror() {
    let mut ctx = Ctx::new();
    ctx.cpu.flags.c = true;
    ctx.cpu.regs.a = 0b11000001;
    ctx.assert_inst("ROR", 2);
    ctx.assert_anzcv(0b11100000, 1, 0, 1, 0);

    ctx.cpu.flags.c = false;
    ctx.cpu.regs.a = 0b01000000;
    ctx.assert_inst("ROR", 2);
    ctx.assert_anzcv(0b00100000, 0, 0, 0, 0);

    ctx.cpu.flags.c = false;
    ctx.cpu.regs.a = 1;
    ctx.assert_inst("ROR", 2);
    ctx.assert_anzcv(0, 0, 1, 1, 0);
}

#[test]
fn test_bit() {
    let mut ctx = Ctx::new();
    ctx.cpu.regs.a = 0x81;
    ctx.memory[0x2001] = 0x41;
    ctx.assert_inst("BIT $2001", 4);
    ctx.assert_anzcv(0x81, 0, 0, 0, 1);

    ctx.cpu.regs.a = 0x41;
    ctx.memory[0x20] = 0x02;
    ctx.assert_inst("BIT $20", 3);
    ctx.assert_anzcv(0x41, 0, 1, 0, 0);
}

#[test]
fn test_inc() {
    let mut ctx = Ctx::new();
    ctx.memory[0x3102] = 0xff;
    ctx.cpu.regs.x = 0x82;
    ctx.assert_inst("INC $3080,X", 7);
    ctx.assert_memnz(0x3102, 0x0, 0, 1);

    ctx.memory[0x31] = 0x7f;
    ctx.assert_inst("INC $31", 5);
    ctx.assert_memnz(0x31, 0x80, 1, 0);
}

#[test]
fn test_inx() {
    let mut ctx = Ctx::new();
    ctx.cpu.regs.x = 0xff;
    ctx.assert_inst("INX", 2);
    ctx.assert_xnzc(0, 0, 1, 0);

    ctx.cpu.regs.x = 0x7f;
    ctx.assert_inst("INX", 2);
    ctx.assert_xnzc(0x80, 1, 0, 0);
}

#[test]
fn test_iny() {
    let mut ctx = Ctx::new();
    ctx.cpu.regs.y = 0xff;
    ctx.assert_inst("INY", 2);
    ctx.assert_ynzc(0, 0, 1, 0);

    ctx.cpu.regs.y = 0x9f;
    ctx.assert_inst("INY", 2);
    ctx.assert_ynzc(0xa0, 1, 0, 0);
}

#[test]
fn test_dec() {
    let mut ctx = Ctx::new();
    ctx.memory[0x3102] = 0x00;
    ctx.cpu.regs.x = 0x82;
    ctx.assert_inst("DEC $3080,X", 7);
    ctx.assert_memnz(0x3102, 0xff, 1, 0);

    ctx.memory[0x31] = 0x01;
    ctx.assert_inst("DEC $31", 5);
    ctx.assert_memnz(0x31, 0x00, 0, 1);
}

#[test]
fn test_dex() {
    let mut ctx = Ctx::new();
    ctx.cpu.regs.x = 0;
    ctx.assert_inst("DEX", 2);
    ctx.assert_xnzc(0xff, 1, 0, 0);

    ctx.cpu.regs.x = 1;
    ctx.assert_inst("DEX", 2);
    ctx.assert_xnzc(0, 0, 1, 0);
}

#[test]
fn test_dey() {
    let mut ctx = Ctx::new();
    ctx.cpu.regs.y = 0;
    ctx.assert_inst("DEY", 2);
    ctx.assert_ynzc(0xff, 1, 0, 0);

    ctx.cpu.regs.y = 1;
    ctx.assert_inst("DEY", 2);
    ctx.assert_ynzc(0, 0, 1, 0);
}

#[test]
fn test_bcc_taken() {
    let mut ctx = Ctx::new();
    ctx.cpu.flags.c = false;
    ctx.assert_inst("BCC +3", 3);
    ctx.assert_branch_taken(3);
}

#[test]
fn test_bcc_not_taken() {
    let mut ctx = Ctx::new();
    ctx.cpu.flags.c = true;
    ctx.assert_inst("BCC +13", 2);
    ctx.assert_branch_not_taken();
}

#[test]
fn test_bcs_taken() {
    let mut ctx = Ctx::new();
    ctx.cpu.flags.c = true;
    ctx.assert_inst("BCS +33", 3);
    ctx.assert_branch_taken(33);
}

#[test]
fn test_bcs_not_taken() {
    let mut ctx = Ctx::new();
    ctx.cpu.flags.c = false;
    ctx.assert_inst("BCS +13", 2);
    ctx.assert_branch_not_taken();
}

#[test]
fn test_beq_taken() {
    let mut ctx = Ctx::new();
    ctx.cpu.flags.z = true;
    ctx.assert_inst("BEQ +103", 3);
    ctx.assert_branch_taken(103);
}

#[test]
fn test_beq_not_taken() {
    let mut ctx = Ctx::new();
    ctx.cpu.flags.z = false;
    ctx.assert_inst("BEQ -2", 2);
    ctx.assert_branch_not_taken();
}

#[test]
fn test_bne_taken() {
    let mut ctx = Ctx::new();
    ctx.cpu.flags.z = false;
    ctx.assert_inst("BNE +43", 3);
    ctx.assert_branch_taken(43);
}

#[test]
fn test_bne_not_taken() {
    let mut ctx = Ctx::new();
    ctx.cpu.flags.z = true;
    ctx.assert_inst("BNE -27", 2);
    ctx.assert_branch_not_taken();
}

#[test]
fn test_bmi_taken() {
    let mut ctx = Ctx::new();
    ctx.cpu.flags.n = true;
    ctx.assert_inst("BMI -3", 4);
    ctx.assert_branch_taken(-3);
}

#[test]
fn test_bmi_not_taken() {
    let mut ctx = Ctx::new();
    ctx.cpu.flags.n = false;
    ctx.assert_inst("BMI -42", 2);
    ctx.assert_branch_not_taken();
}

#[test]
fn test_bpl_taken() {
    let mut ctx = Ctx::new();
    ctx.cpu.flags.n = false;
    ctx.assert_inst("BPL +2", 3);
    ctx.assert_branch_taken(2);
}

#[test]
fn test_bpl_not_taken() {
    let mut ctx = Ctx::new();
    ctx.cpu.flags.n = true;
    ctx.assert_inst("BPL -82", 2);
    ctx.assert_branch_not_taken();
}

#[test]
fn test_bvc_taken() {
    let mut ctx = Ctx::new();
    ctx.cpu.flags.v = false;
    ctx.assert_inst("BVC +29", 3);
    ctx.assert_branch_taken(29);
}

#[test]
fn test_bvc_not_taken() {
    let mut ctx = Ctx::new();
    ctx.cpu.flags.v = true;
    ctx.assert_inst("BVC +29", 2);
    ctx.assert_branch_not_taken();
}

#[test]
fn test_bvs_taken() {
    let mut ctx = Ctx::new();
    ctx.cpu.flags.v = true;
    ctx.assert_inst("BVS +29", 3);
    ctx.assert_branch_taken(29);
}

#[test]
fn test_bvs_not_taken() {
    let mut ctx = Ctx::new();
    ctx.cpu.flags.v = false;
    ctx.assert_inst("BVS +29", 2);
    ctx.assert_branch_not_taken();
}

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
fn test_jmp() {
    let mut ctx = Ctx::new();
    ctx.cpu.reset(&mut ctx.memory);
    ctx.assert_inst("jmp $8000", 3);
    assert_eq!(ctx.cpu.regs.pc, 0x8000);

    ctx.memory.set_word(0xa000, 0x1f80);
    ctx.assert_inst("JMP ($a000)", 5);
    assert_eq!(ctx.cpu.regs.pc, 0x1f80);
}

#[test]
fn test_jsr() {
    let mut ctx = Ctx::new();
    let sp0 = ctx.cpu.regs.sp_address();
    let pc0 = ctx.cpu.regs.pc + 2;
    ctx.assert_inst("JSR $f000", 6);
    assert_eq!(ctx.memory[ctx.cpu.regs.sp_address() + 1], pc0 as u8);
    assert_eq!(ctx.memory[ctx.cpu.regs.sp_address() + 2], (pc0 >> 8) as u8);
    assert_eq!(ctx.cpu.regs.sp_address(), sp0 - 2);
}

#[test]
fn test_brk() {
    let mut ctx = Ctx::new();
    ctx.memory.set_word(Cpu::IRQ_VECTOR, 0xabcd);
    ctx.cpu.flags = Flags::from_byte(0b11001111);
    let sp0 = ctx.cpu.regs.sp_address();
    let pc0 = ctx.cpu.regs.pc + 2;
    ctx.assert_inst("BRK", 7);
    assert_eq!(ctx.cpu.flags.i, true);
    assert_eq!(ctx.memory[ctx.cpu.regs.sp_address() + 1], 0b11011111);
    assert_eq!(ctx.memory[ctx.cpu.regs.sp_address() + 2], pc0 as u8);
    assert_eq!(ctx.memory[ctx.cpu.regs.sp_address() + 3], (pc0 >> 8) as u8);
    assert_eq!(ctx.cpu.regs.sp_address(), sp0 - 3);
    assert_eq!(ctx.cpu.regs.pc, 0xabcd);
}

#[test]
fn test_rti() {
    let mut ctx = Ctx::new();
    ctx.cpu.flags.i = true;
    ctx.cpu.push_word(&mut ctx.memory, 0x8003);
    ctx.cpu.push(&mut ctx.memory, 0b11010011);
    ctx.assert_inst("RTI", 6);
    assert_eq!(ctx.cpu.flags.i, false);
    assert_eq!(ctx.cpu.regs.pc, 0x8003);
    assert_eq!(ctx.cpu.flags.to_byte(), 0b11000011);
}

#[test]
fn test_rts() {
    let mut ctx = Ctx::new();
    ctx.cpu.push_word(&mut ctx.memory, 0x8002);
    ctx.assert_inst("RTS", 6);
    assert_eq!(ctx.cpu.regs.pc, 0x8003);
}

#[test]
fn test_lda() {
    let mut ctx = Ctx::new();
    ctx.assert_inst("LDA #$23", 2);
    ctx.assert_anzc(0x23, 0, 0, 0);

    ctx.memory.set_word(0xf0, 0x2080);
    ctx.cpu.regs.y = 0x92;
    ctx.memory[0x2112] = 0xf4;
    ctx.assert_inst("LDA ($f0),Y", 6);
    ctx.assert_anzc(0xf4, 1, 0, 0);
}

#[test]
fn test_ldx() {
    let mut ctx = Ctx::new();
    ctx.assert_inst("LDX #$23", 2);
    ctx.assert_xnzc(0x23, 0, 0, 0);

    ctx.memory[0x2112] = 0xf4;
    ctx.cpu.regs.y = 0x13;
    ctx.assert_inst("LDX $20ff,Y", 5);
    ctx.assert_xnzc(0xf4, 1, 0, 0);
}

#[test]
fn test_ldy() {
    let mut ctx = Ctx::new();
    ctx.assert_inst("LDY #$23", 2);
    ctx.assert_ynzc(0x23, 0, 0, 0);

    ctx.memory[0xeaf0] = 0xf4;
    ctx.cpu.regs.x = 0xf0;
    ctx.assert_inst("LDY $EA00,X", 4);
    ctx.assert_ynzc(0xf4, 1, 0, 0);
}

#[test]
fn test_sta() {
    let mut ctx = Ctx::new();
    ctx.cpu.regs.a = 0x8a;
    ctx.assert_inst("STA $2300", 4);
    ctx.assert_memnz(0x2300, 0x8a, 0, 0);

    ctx.cpu.regs.a = 0x00;
    ctx.assert_inst("STA $2300", 4);
    ctx.assert_memnz(0x2300, 0x00, 0, 0);
}

#[test]
fn test_stx() {
    let mut ctx = Ctx::new();
    ctx.cpu.regs.x = 0x8a;
    ctx.assert_inst("STX $3300", 4);
    ctx.assert_memnz(0x3300, 0x8a, 0, 0);

    ctx.cpu.regs.x = 0x00;
    ctx.assert_inst("STX $3300", 4);
    ctx.assert_memnz(0x3300, 0x00, 0, 0);
}

#[test]
fn test_sty() {
    let mut ctx = Ctx::new();
    ctx.cpu.regs.y = 0x8a;
    ctx.assert_inst("STY $3300", 4);
    ctx.assert_memnz(0x3300, 0x8a, 0, 0);

    ctx.cpu.regs.y = 0x00;
    ctx.assert_inst("STY $3300", 4);
    ctx.assert_memnz(0x3300, 0x00, 0, 0);
}

#[test]
fn test_tax() {
    let mut ctx = Ctx::new();
    ctx.cpu.regs.a = 0xcf;
    ctx.assert_inst("TAX", 2);
    ctx.assert_xnzc(0xcf, 1, 0, 0);

    ctx.cpu.regs.a = 0;
    ctx.assert_inst("TAX", 2);
    ctx.assert_xnzc(0, 0, 1, 0);
}

#[test]
fn test_tay() {
    let mut ctx = Ctx::new();
    ctx.cpu.regs.a = 0xcf;
    ctx.assert_inst("TAY", 2);
    ctx.assert_ynzc(0xcf, 1, 0, 0);

    ctx.cpu.regs.a = 0;
    ctx.assert_inst("TAY", 2);
    ctx.assert_ynzc(0, 0, 1, 0);
}

#[test]
fn test_tsx() {
    let mut ctx = Ctx::new();
    ctx.cpu.regs.sp = 0x8f;
    ctx.assert_inst("TSX", 2);
    ctx.assert_xnzc(0x8f, 1, 0, 0);

    ctx.cpu.regs.sp = 0x00;
    ctx.assert_inst("TSX", 2);
    ctx.assert_xnzc(0, 0, 1, 0);
}

#[test]
fn test_txa() {
    let mut ctx = Ctx::new();
    ctx.cpu.regs.x = 0xcf;
    ctx.assert_inst("TXA", 2);
    ctx.assert_anzc(0xcf, 1, 0, 0);

    ctx.cpu.regs.x = 0;
    ctx.assert_inst("TXA", 2);
    ctx.assert_anzc(0, 0, 1, 0);
}

#[test]
fn test_tya() {
    let mut ctx = Ctx::new();
    ctx.cpu.regs.y = 0xcf;
    ctx.assert_inst("TYA", 2);
    ctx.assert_anzc(0xcf, 1, 0, 0);

    ctx.cpu.regs.y = 0;
    ctx.assert_inst("TYA", 2);
    ctx.assert_anzc(0, 0, 1, 0);
}

#[test]
fn test_txs() {
    let mut ctx = Ctx::new();
    ctx.cpu.regs.x = 0x81;
    ctx.assert_inst("TXS", 2);
    assert_eq!(ctx.cpu.regs.sp, 0x81);
    assert_eq!(ctx.cpu.flags.n, false);
    assert_eq!(ctx.cpu.flags.z, false);

    ctx.cpu.regs.x = 0;
    ctx.assert_inst("TXS", 2);
    assert_eq!(ctx.cpu.regs.sp, 0);
    assert_eq!(ctx.cpu.flags.n, false);
    assert_eq!(ctx.cpu.flags.z, false);
}

#[test]
fn test_pla() {
    let mut ctx = Ctx::new();
    ctx.cpu.regs.sp = 0x80;
    ctx.memory[0x181] = 0xab;
    ctx.assert_inst("PLA", 4);
    ctx.assert_anzcv(0xab, 1, 0, 0, 0);
    assert_eq!(ctx.cpu.regs.sp_address(), 0x181);
}

#[test]
fn test_pha() {
    let mut ctx = Ctx::new();
    let sp = ctx.cpu.regs.sp_address();
    ctx.cpu.regs.a = 0x1f;
    ctx.assert_inst("PHA", 3);
    assert_eq!(ctx.cpu.regs.sp_address(), sp - 1);
    assert_eq!(ctx.memory[sp], 0x1f);
}

#[test]
fn test_plp() {
    let mut ctx = Ctx::new();
    ctx.cpu.regs.sp = 0x80;
    ctx.memory[0x181] = 0b00001100;
    ctx.assert_inst("PLP", 4);
    assert_eq!(ctx.cpu.flags.to_byte(), 0b00001100);
    assert_eq!(ctx.cpu.regs.sp_address(), 0x181);
}

#[test]
fn test_php() {
    let mut ctx = Ctx::new();
    let sp = ctx.cpu.regs.sp_address();
    ctx.memory[sp] = 0;
    ctx.cpu.flags = Flags::from_byte(0b11001100);
    ctx.assert_inst("PHP", 3);
    assert_eq!(ctx.cpu.regs.sp_address(), sp - 1);
    assert_eq!(ctx.memory[sp], 0b11001100);
}

#[test]
fn test_nop() {
    let mut ctx = Ctx::new();
    ctx.assert_inst("NOP", 2);
}

#[test]
fn test_kil() {
    let mut ctx = Ctx::new();
    ctx.assert_inst("KIL", 0);
    assert_eq!(ctx.cpu.regs.pc, Ctx::PC_INIT);
}
