use crate::{backend::Backend, mos6510::disassembler::disassemble};
use crate::{mos6510::cpu::Cpu, terminal};

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
const HEADER_ROWS: u16 = 2;
const DUMP_COL: u16 = 30;

impl View {
    pub fn new(title: &str) -> Self {
        Self {
            title: String::from(title),
            status: String::from(STATUS_OK),
            ..Self::default()
        }
    }

    pub fn print_header(&self, backend: &Backend, clock: f64) {
        terminal::set_cursor_pos(0, 0);
        terminal::highlight();
        terminal::print("CPU ");
        terminal::normal();
        print_property("PC", &format!("{:04X} ", backend.cpu.regs.pc));
        print_property("SP", &format!("{:04X} ", backend.cpu.regs.sp as u16 | 0x100));
        print_property("A", &format!("{:02X} ", backend.cpu.regs.a));
        print_property("X", &format!("{:02X} ", backend.cpu.regs.x));
        print_property("Y", &format!("{:02X} ", backend.cpu.regs.y));
        print_property("P", &format!("{:08b}", backend.cpu.flags.to_byte()));
        print_property(
            " trap",
            match backend.trap() {
                true => "on",
                false => "off",
            },
        );
        print_speed(backend.clock(), clock);
        terminal::newline();
        terminal::highlight();
        terminal::print("MEM ");
        terminal::normal();
        print_property("RST", &format!("{:04X} ", backend.memory.word(Cpu::RESET_VECTOR)));
        print_property("NMI", &format!("{:04X} ", backend.memory.word(Cpu::NMI_VECTOR)));
        print_property("IRQ", &format!("{:04X} ", backend.memory.word(Cpu::IRQ_VECTOR)));
        print_property("IOC", &format!("{:08b} ", backend.memory.byte(Cpu::IO_PORT_CONFIG)));
        print_property("IOD", &format!("{:08b} ", backend.memory.byte(Cpu::IO_PORT_DATA)));
    }

    pub fn update_size(&mut self, backend: &Backend, cols: u16, rows: u16, clock: f64, idle: bool) {
        #[cfg(target_os = "windows")]
        let rows = rows + 1;

        let cols = cols + 1;
        if cols != self.cols || rows != self.rows {
            self.cols = cols;
            self.rows = rows;
            self.dump_row = HEADER_ROWS;
            self.command_row = self.rows - 3;
            self.status_row = self.rows - 2;
            self.shortcuts_row = self.rows - 1;
            self.bytes_per_row = (cols - DUMP_COL - 8) / 3;
            terminal::clear();
            self.print_header(backend, clock);
            if idle {
                self.print_dump(&backend);
            }
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

    pub fn clear_dump(&self) {
        terminal::hide_cursor();
        for row in self.dump_row..self.command_row {
            terminal::set_cursor_pos(0, row);
            terminal::clear_line();
        }
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
        terminal::clear_line();
        terminal::highlight();
        terminal::print(&self.title);
        print_shortcut(" F1", "Help");
        print_shortcut(" F2", "Clear Stats.");
        print_shortcut(" F5", "Help");
        print_shortcut(" F10", "Help");
        print_shortcut(" Esc", "Quit");
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

fn print_property(name: &str, value: &str) {
    terminal::dim();
    terminal::print(name);
    terminal::print(":");
    terminal::bold();
    terminal::print(value);
}

fn print_shortcut(key: &str, desc: &str) {
    terminal::normal();
    terminal::print(key);
    terminal::dim();
    terminal::print(":");
    terminal::print(desc);
}

fn print_speed(actual_clock: f64, requested_clock: f64) {
    terminal::dim();
    terminal::print(" f:");
    terminal::bold();
    terminal::print(&format!("{:.2}", actual_clock / 1e6));
    terminal::dim();
    terminal::print(&format!("/{:.2} MHz  ", requested_clock / 1e6));
}
