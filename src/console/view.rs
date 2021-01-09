use crate::terminal;
use crate::{backend::Backend, info::Info, mos6510::disassembler::disassemble};

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
        terminal::clear_line();
        terminal::highlight();
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
            self.label(" f", &format!("{:.2} MHz", info.frequency() / 1e6));
        }
        terminal::newline();
    }
}

#[derive(Default)]
pub struct View {
    pub code_addr: u16,
    pub dump_addr: u16,

    cols: u16,
    rows: u16,
    bytes_per_row: u16,
}

const START_ROW: u16 = 1;
const DUMP_COL: u16 = 30;

impl View {
    pub fn resize(&mut self, cols: u16, rows: u16) {
        self.cols = cols;
        self.rows = rows;
        self.bytes_per_row = (cols - DUMP_COL - 8) / 3;
    }

    pub fn print(&self, backend: &Backend) {
        terminal::move_cursor(0, START_ROW);
        let mut code = self.code_addr;
        let mut dump = self.dump_addr;
        for _ in 0..self.rows {
            terminal::clear_line();
            let highlight = code == backend.cpu.regs.pc;
            let columns = disassemble(&backend.memory, &mut code);
            if highlight {
                terminal::normal()
            } else {
                terminal::dim();
            }
            let left = &format!("{} {} ", columns.0, columns.1);
            terminal::print(left);
            if highlight {
                terminal::highlight()
            } else {
                terminal::normal();
            }
            terminal::print(&columns.2);
            terminal::move_to_col(DUMP_COL);
            terminal::dim();
            terminal::print(" │ ");
            terminal::print(&format!("{:04X}", dump));
            terminal::normal();
            for _ in 0..self.bytes_per_row {
                terminal::print(&format!(" {:02X}", backend.memory[dump]));
                dump = dump.wrapping_add(1);
            }
            terminal::newline();
        }
    }
}
