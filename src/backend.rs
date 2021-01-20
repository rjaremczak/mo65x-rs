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
    mos6510::{cpu::Cpu, memory::Memory},
};

pub struct Backend {
    pub memory: Memory,
    pub cpu: Cpu,
    trap: AtomicBool,
    cycles: AtomicU64,
    duration_ns: AtomicU64,
}

impl Backend {
    pub fn new() -> Self {
        let mut backend = Self {
            cpu: Cpu::new(),
            memory: Memory::new(),
            trap: AtomicBool::new(true),
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

    pub fn clock(&self) -> f64 {
        self.cycles.load(Relaxed) as f64 * 1e9 / self.duration_ns.load(Relaxed) as f64
    }

    pub fn upload(&mut self, addr: u16, fpath: PathBuf) -> Result<usize, AppError> {
        if !self.trap.load(Relaxed) {
            return Err(AppError::EmulatorAlreadyRunning);
        }
        let mut buf = Vec::new();
        let size = File::open(&fpath)?.read_to_end(&mut buf)?;
        self.memory.set_block(addr, &buf);
        Ok(size)
    }

    pub fn execute(&mut self, period: Duration) -> Result<u8, AppError> {
        let period_ns = period.as_nanos() as u64;
        loop {
            let t0 = Instant::now();
            let cycles = self.cpu.exec_inst(&mut self.memory);
            self.cycles.fetch_add(cycles as u64, Relaxed);
            if cycles == 0 {
                self.trap.store(true, Relaxed);
                return Err(AppError::InvalidOpCode(self.cpu.regs.pc, self.memory[self.cpu.regs.pc]));
            }
            if self.trap.load(Relaxed) {
                return Ok(cycles);
            }
            let t1 = t0 + Duration::from_nanos(period_ns * cycles as u64);
            while Instant::now() < t1 {}
            self.duration_ns.fetch_add((Instant::now() - t0).as_nanos() as u64, Relaxed);
        }
    }

    #[inline]
    pub fn trap_on(&self) {
        self.trap.store(true, Relaxed);
    }

    #[inline]
    pub fn trap_off(&self) {
        self.trap.store(false, Relaxed);
    }

    #[inline]
    pub fn trap(&self) -> bool {
        self.trap.load(Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let b = Backend::new();
        assert!(b.trap.load(Relaxed));
    }
}
