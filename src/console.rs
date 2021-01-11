mod commands;
mod view;

use self::commands::Command;
use crate::{
    backend::{Backend, ExecMode},
    error::Result,
    frontend::Frontend,
    terminal,
};
use commands::CommandParser;
use crossterm::event::{self, poll, Event, KeyCode, KeyEvent};
use std::{path::PathBuf, sync::atomic::AtomicPtr, thread, time::Duration};
use view::View;
use Event::{Key, Resize};
use KeyCode::{Backspace, Char, Enter, Esc, F};

pub struct Console {
    parser: CommandParser,
    view: View,
}

const STATUS_OK: &str = "Ok";

impl Drop for Console {
    fn drop(&mut self) {
        terminal::end_session()
    }
}

impl Console {
    pub fn new(title: &str) -> Self {
        Self {
            parser: CommandParser::new(),
            view: View::new(title),
        }
    }

    pub fn init(&mut self, backend: &Backend) {
        terminal::begin_session();
        // let (cols, rows) = terminal::size();
        // self.view.update_size(backend, cols, rows);
    }

    fn process_command(&mut self, backend: &mut Backend) {
        let mut status = String::from(STATUS_OK);
        match self.parser.parse(&self.view.command) {
            Some(Command::SetPC(pc)) => {
                backend.cpu.regs.pc = pc;
                self.view.print_header(backend.info());
                self.view.print_dump(backend);
            }
            Some(Command::SetSP(sp)) => {
                backend.cpu.regs.sp = sp;
                self.view.print_header(backend.info());
            }
            Some(Command::SetA(a)) => {
                backend.cpu.regs.a = a;
                self.view.print_header(backend.info());
            }
            Some(Command::SetX(x)) => {
                backend.cpu.regs.x = x;
                self.view.print_header(backend.info());
            }
            Some(Command::SetY(y)) => {
                backend.cpu.regs.y = y;
                self.view.print_header(backend.info());
            }
            Some(Command::SetMemoryByte(addr, value)) => {
                backend.memory[addr] = value;
                self.view.print_dump(&backend);
            }
            Some(Command::Load(addr, fpath)) => {
                match backend.upload(addr, PathBuf::from(fpath)) {
                    Ok(size) => {
                        self.view.print_dump(&backend);
                        status = format!("uploaded {} bytes", size);
                    }
                    Err(err) => {
                        status = format!("error: {:?}", err);
                    }
                };
            }
            Some(Command::Disassemble(addr)) => {
                self.view.code_addr = addr;
                self.view.print_dump(&backend);
            }
            Some(Command::MemoryDump(addr)) => {
                self.view.dump_addr = addr;
                self.view.print_dump(&backend);
            }
            None => {
                status = format!("invalid command: {}", &self.view.command);
            }
        }
        self.view.update_status(status);
        self.view.command.clear();
        self.view.print_command();
    }

    unsafe fn execute(&mut self, backend: &mut Backend, frontend: &mut Frontend) -> Result<()> {
        backend.set_mode(ExecMode::Run);
        let backend_ptr = AtomicPtr::new(backend);
        let handle = thread::spawn(move || (*backend_ptr.into_inner()).execute(Duration::from_micros(1)));
        let mut fberr: Result<()> = Ok(());
        while !frontend.quit() && fberr.is_ok() {
            frontend.vsync();
            fberr = frontend.update(&backend.memory);
        }
        backend.set_mode(ExecMode::Step);
        handle.join().unwrap();
        fberr
    }

    pub fn process(&mut self, backend: &mut Backend, frontend: &mut Frontend) -> bool {
        if poll(Duration::from_millis(2)).unwrap() {
            match event::read() {
                Ok(Key(KeyEvent { code: Char(c), .. })) => {
                    self.view.input_char(c);
                }
                Ok(Key(KeyEvent { code: Backspace, .. })) => {
                    self.view.input_backspace();
                }
                Ok(Key(KeyEvent { code: Esc, .. })) => {
                    return false;
                }
                Ok(Key(KeyEvent { code: Enter, .. })) => {
                    self.process_command(backend);
                }
                Ok(Key(KeyEvent { code: F(10), .. })) => {
                    backend.set_mode(ExecMode::Step);
                    let status = match backend.execute(Duration::from_micros(1)) {
                        0 => format!(
                            "halted at {:04X}, invalid opcode: {:02X}",
                            backend.cpu.regs.pc, backend.memory[backend.cpu.regs.pc]
                        ),
                        cycles @ _ => format!("ok, {} cycles spent", cycles),
                    };
                    self.view.print_header(backend.info());
                    self.view.print_dump(&backend);
                    self.view.update_status(status);
                }
                Ok(Key(KeyEvent { code: F(5), .. })) => {
                    self.view.update_status(String::from("running..."));
                    terminal::flush();
                    let result = unsafe { self.execute(backend, frontend) };
                    self.view.update_status(format!("{:?}", result));
                }
                Ok(Resize(cols, rows)) => {
                    self.view.update_size(backend, cols, rows);
                }
                Ok(event) => {
                    self.view.update_status(format!("unhandled event: {:?}", event));
                }
                Err(err) => {
                    self.view.update_status(format!("event handling error: {:?}", err));
                }
            }
            terminal::restore_cursor();
            terminal::flush();
        }
        true
    }
}
