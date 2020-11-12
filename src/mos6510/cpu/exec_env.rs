use super::registers::Registers;
use crate::mos6510::memory::Memory;

pub struct ExecEnv {
    pub pc: u16,
    pub arg: u16,
    pub page_crossed: bool,
    pub cycles: u8,
}

impl ExecEnv {
    pub fn new(pc: u16, cycles: u8) -> Self {
        Self {
            pc,
            arg: 0,
            page_crossed: false,
            cycles,
        }
    }

    pub fn add_cycle_when_page_crossed(&mut self) {
        if self.page_crossed {
            self.cycles += 1;
        }
    }

    pub fn prep_implied<'a>(&mut self, memory: &'a mut Memory, regs: &'a mut Registers) -> &'a mut u8 {
        &mut regs.a
    }

    pub fn prep_branch<'a>(&mut self, memory: &mut Memory, regs: &'a mut Registers) -> &'a mut u8 {
        &mut regs.a
    }

    pub fn prep_immediate<'a>(&mut self, memory: &mut Memory, regs: &'a mut Registers) -> &'a mut u8 {
        &mut regs.a
    }

    pub fn prep_zero_page<'a>(&mut self, memory: &mut Memory, regs: &'a mut Registers) -> &'a mut u8 {
        &mut regs.a
    }

    pub fn prep_zero_page_x<'a>(&mut self, memory: &mut Memory, regs: &'a mut Registers) -> &'a mut u8 {
        &mut regs.a
    }

    pub fn prep_zero_page_y<'a>(&mut self, memory: &mut Memory, regs: &'a mut Registers) -> &'a mut u8 {
        &mut regs.a
    }

    pub fn prep_indexed_indirect_x<'a>(&mut self, memory: &mut Memory, regs: &'a mut Registers) -> &'a mut u8 {
        &mut regs.a
    }

    pub fn prep_indirect_indexed_y<'a>(&mut self, memory: &mut Memory, regs: &'a mut Registers) -> &'a mut u8 {
        &mut regs.a
    }

    pub fn prep_indirect<'a>(&mut self, memory: &mut Memory, regs: &'a mut Registers) -> &'a mut u8 {
        &mut regs.a
    }

    pub fn prep_absolute<'a>(&mut self, memory: &mut Memory, regs: &'a mut Registers) -> &'a mut u8 {
        &mut regs.a
    }

    pub fn prep_absolute_x<'a>(&mut self, memory: &mut Memory, regs: &'a mut Registers) -> &'a mut u8 {
        &mut regs.a
    }

    pub fn prep_absolute_y<'a>(&mut self, memory: &mut Memory, regs: &'a mut Registers) -> &'a mut u8 {
        &mut regs.a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_implied() {
        let mut memory = Memory::new();
        let mut regs = Registers::new(1000, 0xfd);
        let mut env = ExecEnv::new(1000, 2);
        env.prep_implied(&mut memory, &mut regs);
    }
}
