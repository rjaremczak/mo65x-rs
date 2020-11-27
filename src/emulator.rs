use crate::{
    mos6510::{cpu::Cpu, memory::Memory},
    video::Video,
};

pub struct Emulator {
    state: State,
    cpu: Cpu,
    memory: Memory,
    video: Video,
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
            video: Video::new(),
        }
    }

    pub fn init(&mut self) {
        self.cpu.reset(&self.memory);
        self.cpu.exec_inst(&mut self.memory);
        self.video.init();
    }

    pub fn run(&mut self) {
        self.state = State::Running
    }

    pub fn stop(&mut self) {
        self.state = State::Stopped
    }

    pub fn is_running(&self) -> bool {
        self.state == State::Running
    }
}
