use crate::mos6510::memory::Memory;

pub type ExecEnvHandler = fn(&mut ExecEnv, &mut Memory);

pub struct ExecEnv {
    pub pc: u16,
    pub arg: u16,
    pub addr: u16,
    pub page_crossed: bool,
    pub cycles: u8,
}

impl ExecEnv {
    pub fn new(pc: u16, cycles: u8) -> Self {
        Self {
            pc,
            arg: 0,
            addr: 0,
            page_crossed: false,
            cycles,
        }
    }

    pub fn add_cycle_when_page_crossed(&mut self) {
        if self.page_crossed {
            self.cycles += 1;
        }
    }

    pub fn prep_implied(&mut self, memory: &mut Memory) {}
    pub fn prep_branch(&mut self, memory: &mut Memory) {}
    pub fn prep_immediate(&mut self, memory: &mut Memory) {}
    pub fn prep_zero_page(&mut self, memory: &mut Memory) {}
    pub fn prep_zero_page_x(&mut self, memory: &mut Memory) {}
    pub fn prep_zero_page_y(&mut self, memory: &mut Memory) {}
    pub fn prep_indexed_indirect_x(&mut self, memory: &mut Memory) {}
    pub fn prep_indirect_indexed_y(&mut self, memory: &mut Memory) {}
    pub fn prep_indirect(&mut self, memory: &mut Memory) {}
    pub fn prep_absolute(&mut self, memory: &mut Memory) {}
    pub fn prep_absolute_x(&mut self, memory: &mut Memory) {}
    pub fn prep_absolute_y(&mut self, memory: &mut Memory) {}
}
