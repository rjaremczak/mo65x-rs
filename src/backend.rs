use std::{
    fs::File,
    io::Read,
    path::PathBuf,
    sync::atomic::AtomicBool,
    sync::atomic::{AtomicU64, Ordering::Relaxed},
    thread::{self, sleep, JoinHandle},
    time::{Duration, Instant},
};

use crate::mos6510::{cpu::Cpu, error::AppError, memory::Memory};

pub struct Backend {
    cpu: Cpu,
    memory: Memory,
    step_mode: AtomicBool,
    cycles: AtomicU64,
    duration_ns: AtomicU64,
}

pub struct Statistics {
    pub cycles: u64,
    pub duration: Duration,
}

impl Backend {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            memory: Memory::new(),
            step_mode: AtomicBool::new(false),
            cycles: AtomicU64::new(0),
            duration_ns: AtomicU64::new(0),
        }
    }

    pub fn init(&mut self) {
        self.cpu.reset(&self.memory);
        self.cpu.exec_inst(&mut self.memory);
    }

    pub fn reset_statistics(&self) {
        self.cycles.store(0, Relaxed);
        self.duration_ns.store(0, Relaxed);
    }

    pub fn statistics(&self) -> Statistics {
        Statistics {
            cycles: self.cycles.load(Relaxed),
            duration: Duration::from_nanos(self.duration_ns.load(Relaxed)),
        }
    }

    pub fn upload(&mut self, addr: u16, fpath: PathBuf) -> Result<usize, AppError> {
        if self.step_mode() {
            return Err(AppError::EmulatorAlreadyRunning);
        }
        let mut buf = Vec::new();
        let size = File::open(&fpath)?.read_to_end(&mut buf)?;
        self.memory.set_block(addr, &buf);
        Ok(size)
    }

    pub fn execute(&mut self, addr: u16, period: Duration) -> Result<JoinHandle<()>, AppError> {
        if self.step_mode.compare_and_swap(false, true, Relaxed) {
            Ok(thread::spawn(|| {}))
        } else {
            return Err(AppError::EmulatorAlreadyRunning);
        }
    }

    pub fn refresh_fb(&self) {
        // let t0 = Instant::now();
        // if t0 > ref_time {
        // self.gui.update_fb(self.memory.view(self.fb_addr, Gui::FB_LEN));
        // ref_time = t0 + ref_period;
    }

    #[inline]
    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    pub fn set_reg_pc(&mut self, pc: u16) {
        self.cpu.regs.pc = pc;
    }

    pub fn run(&mut self, period: Duration) -> bool {
        // let ref_period = Duration::from_millis(20);
        // let mut ref_time = Instant::now() + ref_period;
        // while self.gui.is_window_open() && !self.gui.is_key_down(Key::Escape) {
        let period_ns = period.as_nanos() as u64;
        loop {
            let t0 = Instant::now();
            let cycles = self.cpu.exec_inst(&mut self.memory) as u64;
            let dt_ns = Instant::now().duration_since(t0).as_nanos() as u64;
            sleep(Duration::from_nanos((period_ns * cycles).saturating_sub(dt_ns)));
            self.cycles.fetch_add(cycles, Relaxed);
            self.duration_ns.fetch_add((Instant::now() - t0).as_nanos() as u64, Relaxed);
            if cycles == 0 || self.step_mode.load(Relaxed) {
                return cycles != 0;
            }
        }
    }

    pub fn set_step_mode(&self, on: bool) {
        self.step_mode.store(on, Relaxed)
    }

    pub fn step_mode(&self) -> bool {
        self.step_mode.load(Relaxed)
    }
}
