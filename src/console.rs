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
        terminal::move_cursor(0, self.size.1 - 1);
        terminal::bold();
        terminal::print(PROMPT);
        terminal::special();
        terminal::print(&self.command);
    }

    fn process_char(&mut self, c: char) {
        self.command.push(c);
        terminal::print(&self.command[self.command.len() - 1..]);
    }

    fn resize(&mut self, cols: u16, rows: u16) -> bool {
        if cols != self.size.0 || rows != self.size.1 {
            self.size = (cols, rows);
            return true;
        }
        false
    }

    pub fn process(&mut self, backend: &mut Backend, frontend: &mut Frontend) -> bool {
        if poll(Duration::from_millis(2)).unwrap() {
            match event::read() {
                Ok(Event::Key(KeyEvent {
                    code: KeyCode::Char(c), ..
                })) => self.process_char(c),
                Ok(Event::Key(KeyEvent {
                    code: KeyCode::Backspace, ..
                })) => terminal::backspace(),
                Ok(Event::Key(KeyEvent { code: KeyCode::Esc, .. })) => return false,
                Ok(Event::Resize(cols, rows)) => {
                    if self.resize(cols, rows) {
                        self.info = backend.state();
                        terminal::clear();
                        self.print(backend.memory());
                    }
                }
                Ok(event) => {
                    self.status = format!("unhandled event: {:?}", event);
                    terminal::store_cursor();
                    self.print_status();
                    terminal::restore_cursor();
                }
                Err(err) => {
                    self.status = format!("event handling error: {:?}", err);
                    self.print_status();
                }
            }
            terminal::flush();
        }
        true
    }
}
