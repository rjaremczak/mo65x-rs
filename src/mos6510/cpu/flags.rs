pub struct Flags {
    pub n: bool,
    pub v: bool,
    pub d: bool,
    pub i: bool,
    pub z: bool,
    pub c: bool,
}

impl Flags {
    const BM_NEGATIVE: u8 = 0x80;
    const BM_OVERFLOW: u8 = 0x40;
    const BM_BREAK: u8 = 0x10;
    const BM_DECIMAL: u8 = 0x08;
    const BM_INTERRUPT: u8 = 0x04;
    const BM_ZERO: u8 = 0x02;
    const BM_CARRY: u8 = 0x01;

    pub fn new() -> Self {
        Self {
            n: false,
            v: false,
            d: false,
            i: false,
            z: false,
            c: false,
        }
    }

    pub fn compute_n(&mut self, result: u16) {
        self.n = (result & 0x80) != 0;
    }

    pub fn compute_z(&mut self, result: u16) {
        self.z = (result & 0xff) == 0;
    }

    pub fn compute_c(&mut self, result: u16) {
        self.c = (result & 0xff00) != 0;
    }

    pub fn compute_v(&mut self, op1: u16, op2: u16, result: u16) {
        self.v = ((op1 ^ result) & (op2 ^ result) & 0x80) != 0;
    }

    pub fn compute_nz(&mut self, result: u16) {
        self.compute_n(result);
        self.compute_z(result);
    }

    pub fn compute_nzc(&mut self, result: u16) {
        self.compute_nz(result);
        self.compute_c(result);
    }

    pub fn set(&mut self, val: u8) {
        self.n = (val & Self::BM_NEGATIVE) != 0;
        self.v = (val & Self::BM_OVERFLOW) != 0;
        self.d = (val & Self::BM_DECIMAL) != 0;
        self.i = (val & Self::BM_INTERRUPT) != 0;
        self.z = (val & Self::BM_ZERO) != 0;
        self.c = (val & Self::BM_CARRY) != 0;
    }

    pub fn get(&self) -> u8 {
        mask(self.n, Self::BM_NEGATIVE)
            | mask(self.v, Self::BM_OVERFLOW)
            | mask(self.d, Self::BM_DECIMAL)
            | mask(self.i, Self::BM_INTERRUPT)
            | mask(self.z, Self::BM_ZERO)
            | mask(self.c, Self::BM_CARRY)
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
