pub struct Registers {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: StackPointer,
    flags: Flags,
}

struct StackPointer {
    offset: u8,
}

impl StackPointer {
    const BASE: u16 = 0x0100;

    pub fn address(&self) -> u16 {
        self.offset as u16 | Self::BASE
    }
}

pub struct Flags {
    negative: bool,
    overflow: bool,
    decimal: bool,
    interrupt: bool,
    zero: bool,
    carry: bool,
}

impl Flags {
    const BM_NEGATIVE: u8 = 0x80;
    const BM_OVERFLOW: u8 = 0x40;
    const BM_BREAK: u8 = 0x10;
    const BM_DECIMAL: u8 = 0x08;
    const BM_INTERRUPT: u8 = 0x04;
    const BM_ZERO: u8 = 0x02;
    const BM_CARRY: u8 = 0x01;

    pub fn compute_n(&mut self, result: u16) {
        self.negative = (result & 0x80) != 0;
    }
    pub fn compute_z(&mut self, result: u16) {
        self.zero = (result & 0xff) == 0;
    }
    pub fn compute_c(&mut self, result: u16) {
        self.carry = (result & 0xff00) != 0;
    }
    pub fn compute_v(&mut self, op1: u16, op2: u16, result: u16) {
        self.overflow = ((op1 ^ result) & (op2 ^ result) & 0x80) != 0;
    }

    pub fn compute_nz(&mut self, result: u16) {
        self.compute_n(result);
        self.compute_z(result);
    }

    pub fn compute_nzc(&mut self, result: u16) {
        self.compute_nz(result);
        self.compute_c(result);
    }

    pub fn set(&mut self, v: u8) {
        self.negative = (v & Self::BM_NEGATIVE) != 0;
        self.overflow = (v & Self::BM_OVERFLOW) != 0;
        self.decimal = (v & Self::BM_DECIMAL) != 0;
        self.interrupt = (v & Self::BM_INTERRUPT) != 0;
        self.zero = (v & Self::BM_ZERO) != 0;
        self.carry = (v & Self::BM_CARRY) != 0;
    }

    pub fn get(&self) -> u8 {
        mask(self.negative, Self::BM_NEGATIVE)
            | mask(self.overflow, Self::BM_OVERFLOW)
            | mask(self.decimal, Self::BM_DECIMAL)
            | mask(self.interrupt, Self::BM_INTERRUPT)
            | mask(self.zero, Self::BM_ZERO)
            | mask(self.carry, Self::BM_CARRY)
    }
}

#[inline(always)]
fn mask(b: bool, m: u8) -> u8 {
    if b {
        m
    } else {
        0
    }
}
