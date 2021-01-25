mod commands;
mod view;

use self::commands::Command;
use crate::{backend::Backend, error::AppError, frontend::Frontend, terminal};
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
    backend: Backend,
    frontend: Frontend,
    parser: CommandParser,
    view: View,
    handle: Option<JoinHandle<Result<u8, AppError>>>,
    running: Arc<AtomicBool>,
    clock: f64,
}

const STATUS_OK: &str = "Ok";
const STATUS_IS_RUNNING: &str = "Emulation is running, press F5 to stop...";

impl Drop for Console {
    fn drop(&mut self) {
        terminal::end_session()
    }
}

impl Console {
    pub fn start(title: &str, clock: f64) -> Result<(), AppError> {
        let mut console = Self {
            backend: Backend::new(),
            frontend: Frontend::new(),
            parser: CommandParser::new(),
            view: View::new(title),
            handle: None,
            running: Arc::new(AtomicBool::new(false)),
            clock,
        };
        console.init();
        console.processing_loop()
    }

    fn processing_loop(&mut self) -> Result<(), AppError> {
        while !self.frontend.quit() && self.process() {
            self.frontend.update(&self.backend.memory)?;
        }
        Ok(())
    }

    fn init(&mut self) {
        terminal::begin_session();
        let (cols, rows) = terminal::size();
        self.view.update_size(&self.backend, cols, rows, self.clock, true);
        terminal::restore_cursor();
        terminal::flush();
    }

    fn print_cpu_line(&self) {
        self.view
            .print_cpu_line(&self.backend.cpu, self.backend.trap(), self.backend.clock(), self.clock);
    }

    fn print_mem_line(&self) {
        self.view.print_mem_line(&self.backend.memory);
    }

    fn print_dump(&self) {
        self.view.print_dump(&self.backend.memory, self.backend.cpu.regs.pc);
    }

    fn process_command(&mut self) {
        let mut status = String::from(STATUS_OK);
        match self.parser.parse(&self.view.command) {
            Some(Command::SetPC(pc)) => {
                self.backend.cpu.regs.pc = pc;
                self.print_cpu_line();
                self.print_dump();
            }
            Some(Command::SetSP(sp)) => {
                self.backend.cpu.regs.sp = sp;
                self.print_cpu_line();
            }
            Some(Command::SetA(a)) => {
                self.backend.cpu.regs.a = a;
                self.print_cpu_line();
            }
            Some(Command::SetX(x)) => {
                self.backend.cpu.regs.x = x;
                self.print_cpu_line();
            }
            Some(Command::SetY(y)) => {
                self.backend.cpu.regs.y = y;
                self.print_cpu_line();
            }
            Some(Command::SetFlagN(f)) => {
                self.backend.cpu.flags.n = f;
                self.print_cpu_line();
            }
            Some(Command::SetFlagV(f)) => {
                self.backend.cpu.flags.v = f;
                self.print_cpu_line();
            }
            Some(Command::SetFlagD(f)) => {
                self.backend.cpu.flags.d = f;
                self.print_cpu_line();
            }
            Some(Command::SetFlagI(f)) => {
                self.backend.cpu.flags.i = f;
                self.print_cpu_line();
            }
            Some(Command::SetFlagZ(f)) => {
                self.backend.cpu.flags.z = f;
                self.print_cpu_line();
            }
            Some(Command::SetFlagC(f)) => {
                self.backend.cpu.flags.c = f;
                self.print_cpu_line();
            }
            Some(Command::SetByte(addr, value)) => {
                self.backend.memory.set_byte(addr, value);
                self.print_mem_line();
                self.print_dump();
            }
            Some(Command::SetWord(addr, value)) => {
                self.backend.memory.set_word(addr, value);
                self.print_mem_line();
                self.print_dump();
            }
            Some(Command::Load(addr, fpath)) => {
                match self.backend.upload(addr, PathBuf::from(fpath)) {
                    Ok(size) => {
                        self.backend.cpu.regs.pc = addr;
                        self.view.code_addr = addr;
                        self.print_cpu_line();
                        self.print_mem_line();
                        self.print_dump();
                        status = format!("uploaded {} bytes", size);
                    }
                    Err(err) => {
                        status = format!("error: {:?}", err);
                    }
                };
            }
            Some(Command::Disassemble(addr)) => {
                self.view.code_addr = addr;
                self.print_dump();
            }
            Some(Command::MemoryDump(addr)) => {
                self.view.dump_addr = addr;
                self.print_mem_line();
                self.print_dump();
            }
            Some(Command::Reset) => {
                self.backend.cpu.reset(&self.backend.memory);
                self.print_cpu_line();
                self.print_dump();
            }
            Some(Command::Nmi) => {
                self.backend.cpu.nmi(&mut self.backend.memory);
                self.print_cpu_line();
                self.print_dump();
            }
            Some(Command::Irq) => {
                self.backend.cpu.irq(&mut self.backend.memory);
                self.print_cpu_line();
                self.print_dump();
            }
            None => {
                status = format!("invalid command: {}", &self.view.command);
            }
        }
        self.view.update_status(status);
        self.view.command.clear();
        self.view.print_command();
    }

    unsafe fn start_execution(&mut self) {
        self.running.store(true, Ordering::Relaxed);
        self.backend.trap_off();
        let backend_ptr = AtomicPtr::new(&mut self.backend);
        let running_clone = self.running.clone();
        let period = Duration::from_secs_f64(1.0 / self.clock);
        self.handle = Some(thread::spawn(move || {
            let cycles = (*backend_ptr.into_inner()).execute(period);
            running_clone.store(false, Ordering::Relaxed);
            cycles
        }));
    }

    fn stop_execution(&mut self) -> Result<u8, AppError> {
        self.backend.trap_on();
        match self.handle.take() {
            Some(h) => h.join().unwrap(),
            None => Err(AppError::EmulatorNotRunning),
        }
    }

    fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    fn wait_for_key(&self) {
        loop {
            if poll(Duration::from_millis(20)).unwrap() {
                match event::read() {
                    Ok(Key(KeyEvent { .. })) => return,
                    _ => {}
                }
            };
            if self.frontend.quit() {
                return;
            }
        }
    }

    pub fn process(&mut self) -> bool {
        let idle = !self.is_running();
        if !idle {
            self.print_cpu_line();
            self.print_mem_line();
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
                #[allow(unused_must_use)]
                Ok(Key(KeyEvent { code: Esc, .. })) => {
                    self.stop_execution();
                    return false;
                }
                Ok(Key(KeyEvent { code: Enter, .. })) => {
                    if idle {
                        self.process_command();
                    }
                }
                Ok(Key(KeyEvent { code: F(1), .. })) => {
                    if idle {
                        view::print_help();
                        self.wait_for_key();
                        self.view.print_all(&self.backend, self.clock, idle);
                    }
                }
                Ok(Key(KeyEvent { code: F(2), .. })) => {
                    self.backend.reset_statistics();
                    self.print_cpu_line();
                }
                Ok(Key(KeyEvent { code: F(5), .. })) => {
                    if idle {
                        unsafe { self.start_execution() };
                        self.view.clear_dump();
                        self.view.update_status(String::from(STATUS_IS_RUNNING));
                    } else {
                        let result = self.stop_execution();
                        self.print_dump();
                        self.view.update_status(format!("{:?}", result));
                    }
                    self.print_cpu_line();
                }
                Ok(Key(KeyEvent { code: F(10), .. })) => {
                    if idle {
                        self.backend.trap_on();
                        let status = self.backend.execute(Duration::from_micros(1));
                        self.print_cpu_line();
                        self.print_mem_line();
                        self.print_dump();
                        self.view.update_status(format!("{:?}", status));
                    }
                }
                Ok(Resize(cols, rows)) => {
                    self.view.update_size(&self.backend, cols, rows, self.clock, idle);
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
