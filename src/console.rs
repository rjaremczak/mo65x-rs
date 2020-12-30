mod print_info;

use crate::{
    backend::Backend,
    frontend::Frontend,
    info::Info,
    mos6510::{disassembler::disassemble, memory::Memory},
    terminal,
};
use crossterm::event::{self, poll, Event, KeyCode, KeyEvent};
use std::time::Duration;
use Event::{Key, Resize};
use KeyCode::{Backspace, Char, Enter, Esc};

pub struct Console {
    size: (u16, u16),
    title: String,
    info: Info,
    command: String,
    status: String,
}

impl Drop for Console {
    fn drop(&mut self) {
        terminal::end_session()
    }
}

const PROMPT: &str = "> ";
const STATUS_OK: &str = "Ok";

impl Console {
    pub fn new(title: &str) -> Self {
        let mut console = Self {
            size: terminal::size(),
            title: String::from(title),
            info: Info::default(),
            command: String::default(),
            status: String::from(STATUS_OK),
        };
        terminal::begin_session();
        console
    }

    fn print(&mut self, memory: &Memory) {
        terminal::hide_cursor();
        self.print_header();
        self.info.print();
        self.print_disassembly(self.info.regs.pc, memory);
        self.print_status();
        self.print_command();
        terminal::show_cursor();
    }

    fn print_header(&self) {
        terminal::move_cursor(0, 0);
        terminal::special();
        terminal::print(&self.title);
        terminal::normal();
        terminal::print(" ");
    }

    fn print_disassembly(&self, pc: u16, memory: &Memory) {
        let mut pc = pc;
        for row in 1..self.size.1 - 1 {
            let columns = disassemble(memory, &mut pc);
            terminal::move_cursor(0, row);
            terminal::normal();
            terminal::print(&(columns.0 + " "));
            terminal::dim();
            terminal::print(&(columns.1 + " "));
            terminal::bold();
            terminal::print(&(columns.2));
        }
    }

    fn print_status(&self) {
        terminal::move_cursor(0, self.size.1);
        terminal::dim();
        terminal::print(&format!("{:1$}", self.status, self.size.0 as usize));
    }

    fn print_command(&self) {
        let len = (PROMPT.len() + self.command.len()) as u16;
        terminal::move_cursor(0, self.size.1 - 1);
        terminal::bold();
        terminal::print(PROMPT);
        terminal::special();
        terminal::print(&format!("{:1$}", self.command, self.size.0 as usize - len as usize));
        terminal::move_cursor(len, self.size.1 - 1);
    }

    fn process_char(&mut self, c: char) {
        self.command.push(c);
        terminal::print(&self.command[self.command.len() - 1..]);
    }

    fn process_command(&mut self) {
        self.update_status(format!("process command: {}", self.command));
        self.command.clear();
        self.print_command();
    }

    fn resize(&mut self, cols: u16, rows: u16) -> bool {
        if cols != self.size.0 || rows != self.size.1 {
            self.size = (cols, rows);
            return true;
        }
        false
    }

    fn update_status(&mut self, status: String) {
        self.status = status;
        terminal::store_cursor();
        self.print_status();
        terminal::restore_cursor();
    }

    pub fn process(&mut self, backend: &mut Backend, frontend: &mut Frontend) -> bool {
        if poll(Duration::from_millis(2)).unwrap() {
            match event::read() {
                Ok(Key(KeyEvent { code: Char(c), .. })) => self.process_char(c),
                Ok(Key(KeyEvent { code: Backspace, .. })) => terminal::backspace(),
                Ok(Key(KeyEvent { code: Esc, .. })) => return false,
                Ok(Key(KeyEvent { code: Enter, .. })) => self.process_command(),
                Ok(Resize(cols, rows)) => {
                    if self.resize(cols, rows) {
                        self.info = backend.state();
                        // terminal::clear();
                        self.print(backend.memory());
                    }
                }
                Ok(event) => self.update_status(format!("unhandled event: {:?}", event)),
                Err(err) => self.update_status(format!("event handling error: {:?}", err)),
            }
            terminal::flush();
        }
        true
    }
}
