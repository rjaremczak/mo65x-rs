use crate::{
    backend::Backend,
    mos6510::{cpu::flags::Flags, disassembler::disassemble, memory::Memory},
};
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

    pub fn print_cpu_line(&self, cpu: &Cpu, trap: bool, clock: f64, req_clock: f64) {
        terminal::set_cursor_pos(0, 0);
        terminal::highlight();
        terminal::print("CPU ");
        terminal::normal();
        print_property("PC", &format!("{:04X} ", cpu.regs.pc));
        print_property("SP", &format!("{:04X} ", cpu.regs.sp as u16 | 0x100));
        print_property("A", &format!("{:02X} ", cpu.regs.a));
        print_property("X", &format!("{:02X} ", cpu.regs.x));
        print_property("Y", &format!("{:02X} ", cpu.regs.y));
        // print_property("P", &format!("{:08b}", cpu.flags.to_byte()));
        print_flags(cpu.flags);
        print_property(
            " trap",
            match trap {
                true => "on",
                false => "off",
            },
        );
        print_speed(clock, req_clock);
    }

    pub fn print_mem_line(&self, memory: &Memory) {
        terminal::newline();
        terminal::highlight();
        terminal::print("MEM ");
        terminal::normal();
        print_property("RST", &format!("{:04X} ", memory.word(Cpu::RESET_VECTOR)));
        print_property("NMI", &format!("{:04X} ", memory.word(Cpu::NMI_VECTOR)));
        print_property("IRQ", &format!("{:04X} ", memory.word(Cpu::IRQ_VECTOR)));
        print_property("IOC", &format!("{:08b} ", memory.byte(Cpu::IO_PORT_CONFIG)));
        print_property("IOD", &format!("{:08b} ", memory.byte(Cpu::IO_PORT_DATA)));
    }

    pub fn update_size(&mut self, backend: &Backend, cols: u16, rows: u16, req_clock: f64, idle: bool) {
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
            self.print_all(backend, req_clock, idle);
        }
    }

    pub fn print_all(&self, backend: &Backend, req_clock: f64, idle: bool) {
        terminal::clear();
        self.print_cpu_line(&backend.cpu, backend.trap(), backend.clock(), req_clock);
        self.print_mem_line(&backend.memory);
        if idle {
            self.print_dump(&backend.memory, backend.cpu.regs.pc);
        }
        self.print_command();
        self.print_status();
        self.print_shortcuts();
    }

    pub fn print_dump(&self, memory: &Memory, pc: u16) {
        terminal::hide_cursor();
        terminal::set_cursor_pos(0, self.dump_row);
        let mut code = self.code_addr;
        let mut dump = self.dump_addr;
        for _ in self.dump_row..self.command_row {
            terminal::clear_line();
            let highlight = code == pc;
            let columns = disassemble(&memory, &mut code);
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
                terminal::print(&format!(" {:02X}", memory[dump]));
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
        print_shortcut(" F5", "Run/Stop");
        print_shortcut(" F10", "Step");
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

fn print_help_line(cmd: &str, desc: &str) {
    terminal::bold();
    terminal::print(&format!("{:30}", cmd));
    terminal::normal();
    terminal::print(&format!(" - {}", desc));
    terminal::newline();
}

pub fn print_help() {
    terminal::clear();
    terminal::set_cursor_pos(0, 0);
    terminal::dim();
    terminal::print("console commands:");
    terminal::newlines(2);
    print_help_line("pc|sp|a|x|y = hex-value", "assign value to a CPU register");
    print_help_line("n|v|v|i|z|c = bin-value", "assign 0 or 1 value to a CPU flag");
    print_help_line("sb hex-addr hex-byte", "assign byte value to a memory location");
    print_help_line("sw hex-addr hex-word", "assign word value to a memory location");
    print_help_line("l hex-addr file-path", "load binary file to memory at given location");
    print_help_line("d hex-addr", "set start address of disassembly view");
    print_help_line("m hex-addr", "set start address of hex dump view");
    print_help_line("reset", "simulate CPU reset");
    print_help_line("nmi", "simulate NMI request");
    print_help_line("irq", "simulate IRQ request");
    terminal::newline();
    terminal::dim();
    terminal::print("key shortcuts:");
    terminal::newlines(2);
    print_help_line("F1", "this help information");
    print_help_line("F2", "clear runtime statistics");
    print_help_line("F5", "start/stop continuous execution at requested speed");
    print_help_line("F10", "execute single instruction");
    print_help_line("Esc", "quit application");
    terminal::newline();
    terminal::dim();
    terminal::print("press a key to quit this help screen");
    terminal::flush();
}

fn print_flag(bit: bool, text: &str) {
    if bit {
        terminal::bold();
    } else {
        terminal::dim();
    }
    terminal::print(text);
}

fn print_flags(flags: Flags) {
    terminal::dim();
    terminal::print("P:");
    print_flag(flags.n, "N");
    print_flag(flags.v, "V");
    print_flag(false, "_");
    print_flag(false, "_");
    print_flag(flags.d, "D");
    print_flag(flags.i, "I");
    print_flag(flags.z, "Z");
    print_flag(flags.c, "C");
}
