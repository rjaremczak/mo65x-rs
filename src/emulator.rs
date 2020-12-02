use std::path::PathBuf;

use minifb::Key;

use crate::{
    gui::Gui,
    mos6510::{cpu::Cpu, error::AppError, memory::Memory},
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

    pub fn upload(&mut self, origin: u16, bin: PathBuf) -> Result<(), AppError> {
        if self.is_running() {
            return Err(AppError::EmulatorIsRunning);
        }

        Ok(())
    }

    pub fn run(&mut self) {
        self.state = State::Running;
        while self.gui.is_window_open() && !self.gui.is_key_down(Key::Escape) {
            self.gui.update_fb(&self.memory.view(self.fb_addr, Gui::FB_LEN));
        }
    }

    pub fn stop(&mut self) {
        self.state = State::Stopped
    }

    #[inline]
    pub fn is_running(&self) -> bool {
        self.state != State::Stopped
    }
}
