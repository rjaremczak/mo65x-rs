use std::ops::Range;

use crate::{backend::Backend, info::Info, mos6510::disassembler::disassemble};
use crate::{mos6510::memory::Memory, terminal};

pub struct Header {
    pub title: String,
}

impl Header {
    pub fn new(title: &str) -> Self {
        Self {
            title: String::from(title),
        }
    }

    fn label(&self, label: &str, text: &str) {
        terminal::dim();
        terminal::print(label);
        terminal::print(":");
        terminal::bold();
        terminal::print(text);
    }

    pub fn print(&self, info: Info) {
        terminal::move_cursor(0, 0);
        terminal::special();
        terminal::print(&self.title);
        terminal::normal();
        terminal::print(" ");
        self.label("PC", &format!("{:04X}", info.regs.pc));
        self.label(" SP", &format!("{:04X}", info.regs.sp as u16 | 0x100));
        self.label(" A", &format!("{:02X}", info.regs.a));
        self.label(" X", &format!("{:02X}", info.regs.x));
        self.label(" Y", &format!("{:02X}", info.regs.y));
        self.label(" P", &format!("{:08b}", info.flags.to_byte()));
        if info.cycles > 0 {
            self.label(" F", &format!("{}", info.frequency()));
        }
    }
}

#[derive(Default)]
pub struct CodeView {
    pub rows: Range<u16>,
    pub addr: u16,
}

impl CodeView {
    pub fn print(&self, backend: &Backend) {
        let mut lc = self.addr;
        for row in self.rows.clone() {
            let columns = disassemble(&backend.memory, &mut lc);
            let highlight = lc == backend.cpu.regs.pc;
            terminal::move_cursor(0, row);
            if highlight {
                terminal::normal()
            } else {
                terminal::dim();
            }
            terminal::print(&format!("{} {} ", columns.0, columns.1));
            if highlight {
                terminal::special()
            } else {
                terminal::normal();
            }
            terminal::print(&(columns.2));
        }
    }
}
