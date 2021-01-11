use crate::terminal;
use crate::{backend::Backend, info::Info, mos6510::disassembler::disassemble};

use super::STATUS_OK;

#[derive(Default)]
pub struct View {
    pub command: String,
    pub status: String,
    pub code_addr: u16,
    pub dump_addr: u16,

    title: String,
    cols: u16,
    rows: u16,
    dump_row: u16,
    command_row: u16,
    status_row: u16,
    shortcuts_row: u16,
    bytes_per_row: u16,
}

const PROMPT: &str = "> ";
const DUMP_COL: u16 = 30;

impl View {
    pub fn new(title: &str) -> Self {
        Self {
            title: String::from(title),
            status: String::from(STATUS_OK),
            ..Self::default()
        }
    }

    fn label(&self, label: &str, text: &str) {
        terminal::dim();
        terminal::print(label);
        terminal::print(":");
        terminal::bold();
        terminal::print(text);
    }

    pub fn print_header(&self, info: Info) {
        terminal::set_cursor_pos(0, 0);
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

    pub fn update_size(&mut self, backend: &Backend, cols: u16, rows: u16) {
        // correction needed on windows
        let cols = cols + 1;
        let rows = rows + 1;

        if cols != self.cols || rows != self.rows {
            self.cols = cols;
            self.rows = rows;
            self.dump_row = 1;
            self.shortcuts_row = self.rows - 1;
            self.status_row = self.shortcuts_row - 1;
            self.command_row = self.status_row - 1;
            self.bytes_per_row = (cols - DUMP_COL - 8) / 3;
            self.print_header(backend.info());
            self.print_dump(&backend);
            self.print_command();
            self.print_status();
            self.print_shortcuts();
        }
    }

    pub fn print_dump(&self, backend: &Backend) {
        terminal::hide_cursor();
        terminal::set_cursor_pos(0, self.dump_row);
        let mut code = self.code_addr;
        let mut dump = self.dump_addr;
        for _ in self.dump_row..self.command_row {
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
            terminal::set_cursor_col(DUMP_COL);
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
        terminal::show_cursor();
    }

    pub fn print_command(&self) {
        terminal::set_cursor_pos(0, self.command_row);
        terminal::bold();
        terminal::print(PROMPT);
        terminal::highlight();

        let len = (PROMPT.len() + self.command.len()) as u16;
        terminal::print(&format!("{:1$}", self.command, self.cols as usize - len as usize));
        terminal::set_cursor_col(len);
        terminal::store_cursor();
    }

    fn print_status(&self) {
        terminal::set_cursor_pos(0, self.status_row);
        terminal::dim();
        terminal::print(&format!("{:1$}", self.status, self.cols as usize));
    }

    pub fn update_status(&mut self, status: String) {
        self.status = status;
        self.print_status();
    }

    fn print_shortcuts(&self) {
        terminal::set_cursor_pos(0, self.shortcuts_row);
        terminal::normal();
        terminal::clear_line();
        terminal::print("[F1]-Help [F5]-Run [F10]-Step Over [ESC]-Quit");
    }

    pub fn input_char(&mut self, c: char) {
        self.command.push(c);
        terminal::print(&self.command[self.command.len() - 1..]);
        terminal::store_cursor();
    }

    pub fn input_backspace(&mut self) {
        if !self.command.is_empty() {
            self.command.pop();
            terminal::backspace();
            terminal::store_cursor();
        }
    }
}
