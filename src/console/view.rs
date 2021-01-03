use std::ops::Range;

use crate::terminal;
use crate::{info::Info, mos6510::disassembler::disassemble};

impl Info {
    fn label(&self, label: &str, text: &str) {
        terminal::dim();
        terminal::print(label);
        terminal::print(":");
        terminal::bold();
        terminal::print(text);
    }

    pub fn print(&self) {
        self.label("PC", &format!("{:04X}", self.regs.pc));
        self.label(" SP", &format!("{:04X}", self.regs.sp as u16 | 0x100));
        self.label(" A", &format!("{:02X}", self.regs.a));
        self.label(" X", &format!("{:02X}", self.regs.x));
        self.label(" Y", &format!("{:02X}", self.regs.y));
        self.label(" P", &format!("{:08b}", self.flags.to_byte()));
        self.label(
            " T",
            match self.trap {
                true => "on",
                false => "off",
            },
        );
        if self.cycles > 0 {
            self.label(" F", &format!("{}", self.frequency()));
        }
    }
}

#[derive(Default)]
pub struct Disassembler {
    pub rows: Range<u16>,
    pub addr: u16,
    pub pc_sync: bool,
}

impl Disassembler {
    pub fn print(&self, memory: &crate::mos6510::memory::Memory) {
        let mut lc = self.addr;
        for row in self.rows.clone() {
            let columns = disassemble(memory, &mut lc);
            terminal::move_cursor(0, row);
            terminal::normal();
            terminal::print(&(columns.0 + " "));
            terminal::dim();
            terminal::print(&(columns.1 + " "));
            terminal::bold();
            terminal::print(&(columns.2));
        }
    }
}

#[derive(Default)]
pub struct Memory {
    pub addr: u16,
}
