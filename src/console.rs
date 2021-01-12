mod commands;
mod view;

use self::commands::Command;
use crate::{
    backend::Backend,
    error::{AppError, Result},
    terminal,
};
use commands::CommandParser;
use crossterm::event::{self, poll, Event, KeyCode, KeyEvent};
use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, AtomicPtr, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
    time::Duration,
};
use view::View;
use Event::{Key, Resize};
use KeyCode::{Backspace, Char, Enter, Esc, F};

pub struct Console {
    parser: CommandParser,
    view: View,
    handle: Option<JoinHandle<Result<u8>>>,
    running: Arc<AtomicBool>,
}

const STATUS_OK: &str = "Ok";
const STATUS_IS_RUNNING: &str = "Emulation is running, press F5 to stop...";

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
            handle: None,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn init(&mut self, backend: &Backend) {
        terminal::begin_session();
        let (cols, rows) = terminal::size();
        self.view.update_size(backend, cols, rows, true);
        terminal::restore_cursor();
        terminal::flush();
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
            Some(Command::SetByte(addr, value)) => {
                backend.memory.set_byte(addr, value);
                self.view.print_header(backend.info());
                self.view.print_dump(&backend);
            }
            Some(Command::SetWord(addr, value)) => {
                backend.memory.set_word(addr, value);
                self.view.print_header(backend.info());
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
                self.view.print_header(backend.info());
                self.view.print_dump(&backend);
            }
            Some(Command::Reset) => {
                backend.cpu.reset(&backend.memory);
                self.view.print_header(backend.info());
                self.view.print_dump(&backend);
            }
            Some(Command::Nmi) => {
                backend.cpu.nmi(&mut backend.memory);
                self.view.print_header(backend.info());
                self.view.print_dump(&backend);
            }
            Some(Command::Irq) => {
                backend.cpu.irq(&mut backend.memory);
                self.view.print_header(backend.info());
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

    unsafe fn start_execution(&mut self, backend: &mut Backend) {
        self.running.store(true, Ordering::Relaxed);
        backend.trap_off();
        let backend_ptr = AtomicPtr::new(backend);
        let running_clone = self.running.clone();
        self.handle = Some(thread::spawn(move || {
            let cycles = (*backend_ptr.into_inner()).execute(Duration::from_micros(1));
            running_clone.store(false, Ordering::Relaxed);
            cycles
        }));
    }

    fn stop_execution(&mut self, backend: &Backend) -> Result<u8> {
        backend.trap_on();
        match self.handle.take() {
            Some(h) => h.join().unwrap(),
            None => Err(AppError::EmulatorNotRunning),
        }
    }

    fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    pub fn process(&mut self, backend: &mut Backend) -> bool {
        let idle = !self.is_running();
        if !idle {
            self.view.print_header(backend.info());
        }
        if poll(Duration::from_millis(2)).unwrap() {
            match event::read() {
                Ok(Key(KeyEvent { code: Char(c), .. })) => {
                    if idle {
                        self.view.input_char(c);
                    }
                }
                Ok(Key(KeyEvent { code: Backspace, .. })) => {
                    if idle {
                        self.view.input_backspace();
                    }
                }
                Ok(Key(KeyEvent { code: Esc, .. })) => {
                    if idle {
                        return false;
                    }
                }
                Ok(Key(KeyEvent { code: Enter, .. })) => {
                    if idle {
                        self.process_command(backend);
                    }
                }
                Ok(Key(KeyEvent { code: F(2), .. })) => {
                    backend.reset_statistics();
                    self.view.print_header(backend.info());
                }
                Ok(Key(KeyEvent { code: F(5), .. })) => {
                    if idle {
                        unsafe { self.start_execution(backend) };
                        self.view.clear_dump();
                        self.view.update_status(String::from(STATUS_IS_RUNNING));
                    } else {
                        let result = self.stop_execution(backend);
                        self.view.print_dump(backend);
                        self.view.update_status(format!("{:?}", result));
                    }
                    self.view.print_header(backend.info());
                }
                Ok(Key(KeyEvent { code: F(10), .. })) => {
                    if idle {
                        backend.trap_on();
                        let status = backend.execute(Duration::from_micros(1));
                        self.view.print_header(backend.info());
                        self.view.print_dump(&backend);
                        self.view.update_status(format!("{:?}", status));
                    }
                }
                Ok(Resize(cols, rows)) => {
                    self.view.update_size(backend, cols, rows, idle);
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
