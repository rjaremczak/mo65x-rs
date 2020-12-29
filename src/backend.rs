use std::{
    fs::File,
    io::Read,
    path::PathBuf,
    sync::atomic::AtomicBool,
    sync::atomic::{AtomicU64, Ordering::Relaxed},
    time::{Duration, Instant},
};

use crate::{
    error::AppError,
    info::Info,
    mos6510::{cpu::Cpu, memory::Memory},
};

pub struct Backend {
    cpu: Cpu,
    memory: Memory,
    trap: AtomicBool,
    cycles: AtomicU64,
    duration_ns: AtomicU64,
}

impl Backend {
    pub fn new() -> Self {
        let mut backend = Self {
            cpu: Cpu::new(),
            memory: Memory::new(),
            trap: AtomicBool::new(false),
            cycles: AtomicU64::new(0),
            duration_ns: AtomicU64::new(0),
        };
        backend.init();
        backend
    }

    pub fn init(&mut self) {
        self.cpu.reset(&self.memory);
    }

    pub fn reset_statistics(&self) {
        self.cycles.store(0, Relaxed);
        self.duration_ns.store(0, Relaxed);
    }

    pub fn state(&self) -> Info {
        Info {
            regs: self.cpu.regs,
            flags: self.cpu.flags,
            cycles: self.cycles.load(Relaxed),
            duration: Duration::from_nanos(self.duration_ns.load(Relaxed)),
            trap: self.trap.load(Relaxed),
        }
    }

    pub fn upload(&mut self, addr: u16, fpath: PathBuf) -> Result<usize, AppError> {
        if self.trap.load(Relaxed) {
            return Err(AppError::EmulatorAlreadyRunning);
        }
        let mut buf = Vec::new();
        let size = File::open(&fpath)?.read_to_end(&mut buf)?;
        self.memory.set_block(addr, &buf);
        Ok(size)
    }

    #[inline]
    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    pub fn set_reg_pc(&mut self, pc: u16) {
        self.cpu.regs.pc = pc;
    }

    pub fn run(&mut self, period: Duration) -> bool {
        let period_ns = period.as_nanos() as u64;
        loop {
            let t0 = Instant::now();
            let cycles = self.cpu.exec_inst(&mut self.memory) as u64;
            let t1 = t0 + Duration::from_nanos(period_ns * cycles);
            while Instant::now() < t1 {}
            self.cycles.fetch_add(cycles, Relaxed);
            self.duration_ns.fetch_add((Instant::now() - t0).as_nanos() as u64, Relaxed);
            if cycles == 0 || self.trap.load(Relaxed) {
                println!("run ends: {}", cycles != 0);
                return cycles != 0;
            }
        }
    }

    #[inline]
    pub fn set_trap(&self, on: bool) {
        self.trap.store(on, Relaxed)
    }

    #[inline]
    pub fn trap(&self) -> bool {
        self.trap.load(Relaxed)
    }
}
