use std::{
    fs::File,
    io::Read,
    path::PathBuf,
    thread::sleep,
    time::{Duration, Instant},
};

use minifb::Key;

use crate::{
    gui::Gui,
    mos6510::{cpu::Cpu, disassembler::disassemble_instr, error::AppError, memory::Memory, operation::Operation},
};

pub struct Emulator {
    state: State,
    cpu: Cpu,
    memory: Memory,
    gui: Gui,
    fb_addr: u16,
}

#[derive(PartialEq)]
enum State {
    Running,
    Stopping,
    Stopped,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            state: State::Stopped,
            cpu: Cpu::new(),
            memory: Memory::new(),
            gui: Gui::new(),
            fb_addr: 0x200,
        }
    }

    pub fn init(&mut self) {
        self.cpu.reset(&self.memory);
        self.cpu.exec_inst(&mut self.memory);
        self.gui.init();
        for i in 0..Gui::FB_LEN {
            self.memory[self.fb_addr + i as u16] = i as u8;
        }
    }

    pub fn upload(&mut self, addr: u16, fpath: PathBuf) -> Result<usize, AppError> {
        if self.is_running() {
            return Err(AppError::EmulatorIsRunning);
        }
        let mut buf = Vec::new();
        let size = File::open(&fpath)?.read_to_end(&mut buf)?;
        self.memory.set_block(addr, &buf);
        Ok(size)
    }

    pub fn run(&mut self, addr: u16, period: Duration) {
        self.cpu.regs.pc = addr;
        self.state = State::Running;
        while self.gui.is_window_open() && !self.gui.is_key_down(Key::Escape) {
            let t0 = Instant::now();
            let mut pc = self.cpu.regs.pc;
            disassemble_instr(&self.memory, &mut pc);
            let cycles = self.cpu.exec_inst(&mut self.memory);
            if cycles == 0 {
                let operation = Operation::get(self.memory[self.cpu.regs.pc]);
                println!("stopped at {:04X} opcode: {:#?}", self.cpu.regs.pc, operation);
                break;
            }
            let dt = period * cycles as u32;
            // sleep(dt - t0.elapsed());
            sleep(Duration::from_millis(500));
            self.gui.update_fb(self.memory.view(self.fb_addr, Gui::FB_LEN));
        }
        self.state = State::Stopped;
    }

    pub fn stop(&mut self) {
        self.state = State::Stopped
    }

    #[inline]
    pub fn is_running(&self) -> bool {
        self.state != State::Stopped
    }
}
