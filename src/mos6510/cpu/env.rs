use super::registers::Registers;
use crate::mos6510::memory::Memory;

pub struct Env {
    pub pc: u16,
    pub arg: u16,
    pub page_crossed: bool,
    pub cycles: u8,
    ptr: *mut u8,
}

impl Env {
    pub fn new(pc: u16, cycles: u8) -> Self {
        Self {
            pc,
            arg: 0,
            ptr: std::ptr::null_mut(),
            page_crossed: false,
            cycles,
        }
    }

    #[inline]
    pub fn set_result(&mut self, val: u8) {
        unsafe {
            *self.ptr = val;
        }
    }

    #[inline]
    pub fn result(&self) -> u8 {
        unsafe { *self.ptr }
    }

    pub fn add_cycle_when_page_crossed(&mut self) {
        if self.page_crossed {
            self.cycles += 1;
        }
    }

    pub fn prep_implied(&mut self, _: &mut Memory, regs: &mut Registers) {
        self.ptr = &mut regs.a as *mut u8;
    }

    pub fn prep_branch(&mut self, _: &mut Memory, _: &mut Registers) {
        // all handled by branch instructions
    }

    pub fn prep_immediate(&mut self, memory: &mut Memory, _: &mut Registers) {
        self.arg = memory[self.pc] as u16;
    }

    pub fn prep_zero_page(&mut self, memory: &mut Memory, _: &mut Registers) {
        self.arg = memory[self.pc] as u16;
        self.ptr = &mut memory[self.arg] as *mut u8;
    }

    pub fn prep_zero_page_x(&mut self, memory: &mut Memory, regs: &mut Registers) {
        self.arg = memory[self.pc].wrapping_add(regs.x) as u16;
        self.ptr = &mut memory[self.arg] as *mut u8;
    }

    pub fn prep_zero_page_y(&mut self, memory: &mut Memory, regs: &mut Registers) {
        self.arg = memory[self.pc].wrapping_add(regs.y) as u16;
        self.ptr = &mut memory[self.arg] as *mut u8;
    }

    pub fn prep_indexed_indirect_x(&mut self, memory: &mut Memory, regs: &mut Registers) {
        let addr = memory[self.pc].wrapping_add(regs.x) as u16;
        self.arg = memory.word(addr);
        self.ptr = &mut memory[self.arg] as *mut u8;
    }

    pub fn prep_indirect_indexed_y(&mut self, memory: &mut Memory, regs: &mut Registers) {
        let addr = memory.word(memory[self.pc] as u16);
        self.arg = addr.wrapping_add(regs.y as u16);
        self.ptr = &mut memory[self.arg] as *mut u8;
        self.update_page_crossed(addr, self.arg);
    }

    pub fn prep_indirect(&mut self, memory: &mut Memory, _: &mut Registers) {
        let addr = memory.word(self.pc);
        self.arg = memory.word(addr);
        self.ptr = &mut memory[self.arg] as *mut u8;
    }

    pub fn prep_absolute(&mut self, memory: &mut Memory, _: &mut Registers) {
        self.arg = memory.word(self.pc);
        self.ptr = &mut memory[self.arg] as *mut u8;
    }

    pub fn prep_absolute_x(&mut self, memory: &mut Memory, regs: &mut Registers) {
        let addr = memory.word(self.pc);
        self.arg = addr.wrapping_add(regs.x as u16);
        self.ptr = &mut memory[self.arg] as *mut u8;
        self.update_page_crossed(addr, self.arg);
    }

    pub fn prep_absolute_y(&mut self, memory: &mut Memory, regs: &mut Registers) {
        let addr = memory.word(self.pc);
        self.arg = addr.wrapping_add(regs.y as u16);
        self.ptr = &mut memory[self.arg] as *mut u8;
        self.update_page_crossed(addr, self.arg);
    }

    #[inline]
    fn update_page_crossed(&mut self, addr: u16, ea: u16) {
        self.page_crossed = ((addr ^ ea) & 0xff00) != 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> (Env, Memory, Registers) {
        (Env::new(1000, 2), Memory::new(), Registers::new(1000, 0xfd))
    }

    #[test]
    fn test_implied() {
        let (mut env, mut memory, mut regs) = setup();
        env.prep_implied(&mut memory, &mut regs);
        assert_eq!(env.ptr, &mut regs.a as *mut u8);
        regs.a = 0x01;
        env.set_result(0x12);
        assert_eq!(regs.a, 0x12);
        assert_eq!(env.result(), 0x12);
    }

    #[test]
    fn test_immediate() {
        let (mut env, mut memory, mut regs) = setup();
        memory[regs.pc] = 0x23;
        env.prep_immediate(&mut memory, &mut regs);
        assert_eq!(env.arg, 0x23);
    }

    #[test]
    fn test_zero_page() {
        let (mut env, mut memory, mut regs) = setup();
        memory[regs.pc] = 0xf0;
        memory[0xf0] = 0x32;
        env.prep_zero_page(&mut memory, &mut regs);
        assert_eq!(env.arg, 0xf0);
        assert_eq!(env.result(), 0x32);
        env.set_result(0x0a);
        assert_eq!(env.result(), 0x0a);
    }

    #[test]
    fn test_zero_page_x() {
        let (mut env, mut memory, mut regs) = setup();
        regs.x = 5;
        memory[regs.pc] = 0xf0;
        memory[0xf0] = 0x2f;
        memory[0xf5] = 0x3a;
        env.prep_zero_page_x(&mut memory, &mut regs);
        assert_eq!(env.arg, 0xf5);
        assert_eq!(env.result(), 0x3a);
        env.set_result(0x2a);
        assert_eq!(env.result(), 0x2a);
    }

    #[test]
    fn test_zero_page_y() {
        let (mut env, mut memory, mut regs) = setup();
        regs.y = 7;
        memory[regs.pc] = 0xf0;
        memory[0xf0] = 0x2f;
        memory[0xf7] = 0x3a;
        env.prep_zero_page_y(&mut memory, &mut regs);
        assert_eq!(env.arg, 0xf7);
        assert_eq!(env.result(), 0x3a);
    }

    #[test]
    fn test_indexed_indirect_x() {
        let (mut env, mut memory, mut regs) = setup();
        regs.x = 3;
        memory[regs.pc] = 0xa0;
        memory.set_word(0x00a3, 0x2f00);
        memory[0x2f00] = 0xc1;
        env.prep_indexed_indirect_x(&mut memory, &mut regs);
        assert_eq!(env.arg, 0x2f00);
        assert_eq!(env.result(), 0xc1);
        env.set_result(0x0c);
        assert_eq!(memory[0x2f00], 0x0c);
    }
}
