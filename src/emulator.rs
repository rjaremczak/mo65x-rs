use minifb::Key;

use crate::{
    gui::Gui,
    mos6510::{cpu::Cpu, memory::Memory},
};

pub struct Emulator {
    state: State,
    cpu: Cpu,
    memory: Memory,
    gui: Gui,
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
        }
    }

    pub fn init(&mut self) {
        self.cpu.reset(&self.memory);
        self.cpu.exec_inst(&mut self.memory);
        self.gui.init();
    }

    pub fn run(&mut self) {
        self.state = State::Running;
        while self.gui.is_window_open() && !self.gui.is_key_down(Key::Escape) {
            self.gui.update_fb(&self.memory.view(0x100, 0x400));
        }
    }

    pub fn stop(&mut self) {
        self.state = State::Stopped
    }

    pub fn is_running(&self) -> bool {
        self.state == State::Running
    }
}
