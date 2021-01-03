mod parser;
mod view;

use crate::{
    backend::Backend,
    frontend::Frontend,
    info::Info,
    mos6510::{disassembler::disassemble, memory::Memory},
    terminal,
};
use crossterm::event::{self, poll, Event, KeyCode, KeyEvent};
use parser::CommandParser;
use std::{path::PathBuf, time::Duration};
use Event::{Key, Resize};
use KeyCode::{Backspace, Char, Enter, Esc};

use self::parser::Command;

pub struct Console {
    size: (u16, u16),
    title: String,
    info: Info,
    command: String,
    status: String,
    parser: CommandParser,
    dis_view: view::Disassembler,
    mem_view: view::Memory,
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
        Self {
            size: terminal::size(),
            title: String::from(title),
            info: Info::default(),
            command: String::default(),
            status: String::from(STATUS_OK),
            parser: CommandParser::new(),
            dis_view: view::Disassembler::default(),
            mem_view: view::Memory::default(),
        }
    }

    pub fn init(&self) {
        terminal::begin_session()
    }

    fn print(&mut self, memory: &Memory) {
        terminal::hide_cursor();
        self.print_info();
        self.dis_view.print(memory);
        self.print_status();
        self.print_command();
        terminal::show_cursor();
    }

    fn print_info(&self) {
        terminal::move_cursor(0, 0);
        terminal::special();
        terminal::print(&self.title);
        terminal::normal();
        terminal::print(" ");
        self.info.print();
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

    fn process_command(&mut self, backend: &mut Backend) {
        self.update_status(STATUS_OK);
        match self.parser.parse(&self.command) {
            Some(Command::SetPC(w)) => backend.cpu_regs_mut().pc = w,
            Some(Command::SetSP(b)) => backend.cpu_regs_mut().sp = b,
            Some(Command::SetA(b)) => backend.cpu_regs_mut().a = b,
            Some(Command::SetX(b)) => backend.cpu_regs_mut().x = b,
            Some(Command::SetY(b)) => backend.cpu_regs_mut().y = b,
            Some(Command::SetMemoryByte(addr, b)) => backend.set_memory_byte(addr, b),
            Some(Command::Load(addr, f)) => match backend.upload(addr, PathBuf::from(f)) {
                Ok(size) => self.update_status(&format!("uploaded {} bytes", size)),
                Err(err) => self.update_status(&format!("error: {:?}", err)),
            },
            Some(Command::DisAddr(addr)) => {
                self.dis_view.pc_sync = false;
                self.dis_view.addr = addr;
                self.dis_view.print(backend.memory());
            }
            Some(Command::DisPcSync) => {
                self.dis_view.pc_sync = false;
            }
            None => {}
        }
        self.update_info(backend.info());
        self.command.clear();
        self.print_command();
    }

    fn resize(&mut self, cols: u16, rows: u16) -> bool {
        if cols != self.size.0 || rows != self.size.1 {
            self.size = (cols, rows);
            self.dis_view.rows = 1..self.size.1 - 1;
            return true;
        }
        false
    }

    fn update_status(&mut self, status: &str) {
        self.status = String::from(status);
        terminal::store_cursor();
        self.print_status();
        terminal::restore_cursor();
    }

    fn update_info(&mut self, info: Info) {
        self.info = info;
        if self.dis_view.pc_sync {
            self.dis_view.addr = self.info.regs.pc;
        }
        self.print_info();
    }

    pub fn process(&mut self, backend: &mut Backend, frontend: &mut Frontend) -> bool {
        if poll(Duration::from_millis(2)).unwrap() {
            match event::read() {
                Ok(Key(KeyEvent { code: Char(c), .. })) => self.process_char(c),
                Ok(Key(KeyEvent { code: Backspace, .. })) => terminal::backspace(),
                Ok(Key(KeyEvent { code: Esc, .. })) => return false,
                Ok(Key(KeyEvent { code: Enter, .. })) => self.process_command(backend),
                Ok(Resize(cols, rows)) => {
                    if self.resize(cols, rows) {
                        self.info = backend.info();
                        // terminal::clear();
                        self.print(backend.memory());
                    }
                }
                Ok(event) => self.update_status(&format!("unhandled event: {:?}", event)),
                Err(err) => self.update_status(&format!("event handling error: {:?}", err)),
            }
            terminal::flush();
        }
        true
    }
}
