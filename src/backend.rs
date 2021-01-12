use std::{
    fs::File,
    io::Read,
    path::PathBuf,
    ptr::read_volatile,
    sync::atomic::AtomicBool,
    sync::atomic::{AtomicU64, Ordering::Relaxed},
    time::{Duration, Instant},
};

use crate::{
    error::AppError,
    info::Info,
    mos6510::{
        cpu::{flags::Flags, registers::Registers, Cpu},
        memory::Memory,
    },
    Result,
};

#[derive(PartialEq, Debug)]
pub enum ExecMode {
    Run,
    Step,
}

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

    pub fn info(&self) -> Info {
        unsafe {
            Info {
                regs: self.cpu.regs,
                flags: self.cpu.flags,
                cycles: self.cycles.load(Relaxed),
                duration: Duration::from_nanos(self.duration_ns.load(Relaxed)),
                trap: self.trap.load(Relaxed),
                rst: self.memory.word(Cpu::RESET_VECTOR),
                nmi: self.memory.word(Cpu::NMI_VECTOR),
                irq: self.memory.word(Cpu::IRQ_VECTOR),
                io_config: self.memory[Cpu::IO_PORT_CONFIG],
                io_data: self.memory[Cpu::IO_PORT_DATA],
            }
        }
    }

    pub fn upload(&mut self, addr: u16, fpath: PathBuf) -> Result<usize> {
        if !self.trap.load(Relaxed) {
            return Err(AppError::EmulatorAlreadyRunning);
        }
        let mut buf = Vec::new();
        let size = File::open(&fpath)?.read_to_end(&mut buf)?;
        self.memory.set_block(addr, &buf);
        Ok(size)
    }

    pub fn execute(&mut self, period: Duration) -> u8 {
        let period_ns = period.as_nanos() as u64;
        loop {
            let t0 = Instant::now();
            let cycles = self.cpu.exec_inst(&mut self.memory);
            let t1 = t0 + Duration::from_nanos(period_ns * cycles as u64);
            while Instant::now() < t1 {}
            self.cycles.fetch_add(cycles as u64, Relaxed);
            self.duration_ns.fetch_add((Instant::now() - t0).as_nanos() as u64, Relaxed);
            if cycles == 0 || self.trap.load(Relaxed) {
                return cycles;
            }
        }
    }

    #[inline]
    pub fn set_mode(&self, mode: ExecMode) {
        self.trap.store(
            match mode {
                ExecMode::Run => false,
                ExecMode::Step => true,
            },
            Relaxed,
        );
    }

    #[inline]
    pub fn mode(&self) -> ExecMode {
        match self.trap.load(Relaxed) {
            false => ExecMode::Run,
            true => ExecMode::Step,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let b = Backend::new();
        assert_eq!(b.mode(), ExecMode::Step);
    }
}
