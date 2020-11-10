use crate::mos6510::memory::Memory;

pub struct ExecEnv<'a> {
    pub memory: &'a mut Memory,
    pub arg: u16,
    pub addr: u16,
    pub page_crossed: bool,
    pub cycles: u8,
}

impl<'a> ExecEnv<'a> {
    pub fn new(memory: &'a mut Memory, arg: u16, addr: u16, cycles: u8) -> Self {
        Self {
            memory,
            arg,
            addr,
            page_crossed: false,
            cycles,
        }
    }

    pub fn add_cycle_when_page_crossed(&mut self) {
        if self.page_crossed {
            self.cycles += 1;
        }
    }
}
