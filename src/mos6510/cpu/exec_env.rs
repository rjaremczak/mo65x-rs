use super::registers::Registers;
use crate::mos6510::memory::Memory;

pub struct ExecEnv {
    pub pc: u16,
    pub val: u16,
    pub ptr: *mut u8,
    pub page_crossed: bool,
    pub cycles: u8,
}

impl ExecEnv {
    pub fn new(pc: u16, cycles: u8) -> Self {
        Self {
            pc,
            val: 0,
            ptr: std::ptr::null_mut(),
            page_crossed: false,
            cycles,
        }
    }

    pub fn add_cycle_when_page_crossed(&mut self) {
        if self.page_crossed {
            self.cycles += 1;
        }
    }

    pub fn prep_implied(&mut self, memory: &mut Memory, regs: &mut Registers) {
        self.ptr = &mut regs.a as *mut u8;
    }

    pub fn prep_branch(&mut self, memory: &mut Memory, regs: &mut Registers) {
        // all handled by branch instructions
    }

    pub fn prep_immediate(&mut self, memory: &mut Memory, regs: &mut Registers) {
        self.val = memory[self.pc] as u16;
    }

    pub fn prep_zero_page(&mut self, memory: &mut Memory, regs: &mut Registers) {
        self.val = memory[self.pc] as u16;
        self.ptr = memory[self.val] as *mut u8;
    }

    pub fn prep_zero_page_x(&mut self, memory: &mut Memory, regs: &mut Registers) {
        self.val = memory[self.pc].wrapping_add(regs.x) as u16;
        self.ptr = memory[self.val] as *mut u8;
    }

    pub fn prep_zero_page_y(&mut self, memory: &mut Memory, regs: &mut Registers) {
        self.val = memory[self.pc].wrapping_add(regs.y) as u16;
        self.ptr = memory[self.val] as *mut u8;
    }

    pub fn prep_indexed_indirect_x(&mut self, memory: &mut Memory, regs: &mut Registers) {
        let addr = memory[self.pc].wrapping_add(regs.x) as u16;
        self.val = memory.word(addr);
        self.ptr = memory[self.val] as *mut u8;
    }

    pub fn prep_indirect_indexed_y(&mut self, memory: &mut Memory, regs: &mut Registers) {
        let addr = memory.word(memory[self.pc] as u16);
        self.val = addr.wrapping_add(regs.y as u16);
        self.ptr = memory[self.val] as *mut u8;
        self.update_page_crossed(addr, self.val);
    }

    pub fn prep_indirect(&mut self, memory: &mut Memory, regs: &mut Registers) {
        let addr = memory.word(self.pc);
        self.val = memory.word(addr);
        self.ptr = memory[self.val] as *mut u8;
    }

    pub fn prep_absolute(&mut self, memory: &mut Memory, regs: &mut Registers) {
        self.val = memory.word(self.pc);
        self.ptr = memory[self.val] as *mut u8;
    }

    pub fn prep_absolute_x(&mut self, memory: &mut Memory, regs: &mut Registers) {
        let addr = memory.word(self.pc);
        self.val = addr.wrapping_add(regs.x as u16);
        self.ptr = memory[self.val] as *mut u8;
        self.update_page_crossed(addr, self.val);
    }

    pub fn prep_absolute_y(&mut self, memory: &mut Memory, regs: &mut Registers) {
        let addr = memory.word(self.pc);
        self.val = addr.wrapping_add(regs.y as u16);
        self.ptr = memory[self.val] as *mut u8;
        self.update_page_crossed(addr, self.val);
    }

    #[inline]
    fn update_page_crossed(&mut self, addr: u16, ea: u16) {
        self.page_crossed = ((addr ^ ea) & 0xff00) != 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> (ExecEnv, Memory, Registers) {
        (ExecEnv::new(1000, 2), Memory::new(), Registers::new(1000, 0xfd))
    }

    #[test]
    fn test_implied() {
        let (mut env, mut memory, mut regs) = setup();
        env.prep_implied(&mut memory, &mut regs);
        assert_eq!(env.ptr, &mut regs.a as *mut u8);
    }
}
