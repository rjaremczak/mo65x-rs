mod commands;
mod view;

use crate::{backend::Backend, frontend::Frontend, info::Info, mos6510::memory::Memory, terminal};
use commands::CommandParser;
use crossterm::event::{self, poll, Event, KeyCode, KeyEvent};
use std::{path::PathBuf, time::Duration};
use view::CodeView;
use Event::{Key, Resize};
use KeyCode::{Backspace, Char, Enter, Esc, F};

use self::{commands::Command, view::Header};

pub struct Console {
    cols: u16,
    rows: u16,
    header: Header,
    command: String,
    status: String,
    parser: CommandParser,
    code: CodeView,
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
            cols: 0,
            rows: 0,
            header: Header::new(title),
            command: String::default(),
            status: String::from(STATUS_OK),
            parser: CommandParser::new(),
            code: CodeView::default(),
        }
    }

    pub fn init(&mut self, backend: &Backend) {
        terminal::begin_session();
        let (cols, rows) = terminal::size();
        self.update_size(backend, cols, rows);
    }

    fn print_status(&self) {
        terminal::move_cursor(0, self.rows);
        terminal::dim();
        terminal::print(&format!("{:1$}", self.status, self.cols as usize));
    }

    fn print_command(&self) {
        let len = (PROMPT.len() + self.command.len()) as u16;
        terminal::move_cursor(0, self.rows - 1);
        terminal::bold();
        terminal::print(PROMPT);
        terminal::special();
        terminal::print(&format!("{:1$}", self.command, self.cols as usize - len as usize));
        terminal::move_cursor(len, self.rows - 1);
    }

    fn process_char(&mut self, c: char) {
        self.command.push(c);
        terminal::print(&self.command[self.command.len() - 1..]);
    }

    fn backspace(&mut self) {
        if !self.command.is_empty() {
            terminal::backspace();
            self.command.pop();
        }
    }

    fn process_command(&mut self, backend: &mut Backend) {
        self.status = String::from(STATUS_OK);
        match self.parser.parse(&self.command) {
            Some(Command::SetPC(pc)) => {
                backend.cpu.regs.pc = pc;
                self.header.print(backend.info());
                self.code.print(backend);
            }
            Some(Command::SetSP(sp)) => {
                backend.cpu.regs.sp = sp;
                self.header.print(backend.info());
            }
            Some(Command::SetA(a)) => {
                backend.cpu.regs.a = a;
                self.header.print(backend.info());
            }
            Some(Command::SetX(x)) => {
                backend.cpu.regs.x = x;
                self.header.print(backend.info());
            }
            Some(Command::SetY(y)) => {
                backend.cpu.regs.y = y;
                self.header.print(backend.info());
            }
            Some(Command::SetMemoryByte(addr, value)) => {
                backend.memory[addr] = value;
                self.code.print(&backend);
            }
            Some(Command::Load(addr, fpath)) => {
                match backend.upload(addr, PathBuf::from(fpath)) {
                    Ok(size) => {
                        self.update_status(format!("uploaded {} bytes", size));
                        self.code.print(&backend);
                    }
                    Err(err) => {
                        self.update_status(format!("error: {:?}", err));
                    }
                };
            }
            Some(Command::Disassemble(addr)) => {
                self.code.addr = addr;
                self.code.print(&backend);
            }
            None => {
                self.update_status(format!("invalid command: {}", &self.command));
            }
        }
        self.command.clear();
        self.print_command();
    }

    pub fn process(&mut self, backend: &mut Backend, frontend: &mut Frontend) -> bool {
        if poll(Duration::from_millis(2)).unwrap() {
            match event::read() {
                Ok(Key(KeyEvent { code: Char(c), .. })) => self.process_char(c),
                Ok(Key(KeyEvent { code: Backspace, .. })) => self.backspace(),
                Ok(Key(KeyEvent { code: Esc, .. })) => return false,
                Ok(Key(KeyEvent { code: Enter, .. })) => self.process_command(backend),
                Ok(Key(KeyEvent { code: F(10), .. })) => {
                    backend.set_trap(true);
                    match backend.run(Duration::from_micros(1)) {
                        0 => self.update_status(format!(
                            "halted at {:04X}, invalid opcode: {:02X}",
                            backend.cpu.regs.pc, backend.memory[backend.cpu.regs.pc]
                        )),
                        cycles @ _ => self.update_status(format!("ok, {} cycles spent", cycles)),
                    }
                    self.header.print(backend.info());
                    self.code.print(&backend);
                }
                Ok(Key(KeyEvent { code: F(5), .. })) => {
                    self.update_status(String::from("run not yet implemented"));
                }
                Ok(Resize(cols, rows)) => self.update_size(backend, cols, rows),
                Ok(event) => self.update_status(format!("unhandled event: {:?}", event)),
                Err(err) => self.update_status(format!("event handling error: {:?}", err)),
            }
            terminal::flush();
        }
        true
    }

    fn update_size(&mut self, backend: &Backend, cols: u16, rows: u16) {
        if cols != self.cols || rows != self.rows {
            self.code.rows = rows - 2;
            self.code.width = cols;
            self.header.print(backend.info());
            self.code.print(&backend);
            self.print_status();
            self.print_command();
        }
    }

    fn update_status(&mut self, status: String) {
        self.status = status;
        self.print_status();
    }
}
