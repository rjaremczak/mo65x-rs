use std::{
    fs::File,
    io::Read,
    path::PathBuf,
    sync::atomic::AtomicBool,
    sync::atomic::{AtomicU64, Ordering::Relaxed},
    thread::sleep,
    time::{Duration, Instant},
};

use crate::mos6510::{
    cpu::{flags::Flags, registers::Registers, Cpu},
    error::AppError,
    memory::Memory,
};

pub struct Backend {
    cpu: Cpu,
    memory: Memory,
    trap: AtomicBool,
    cycles: AtomicU64,
    duration_ns: AtomicU64,
}

#[derive(Debug)]
pub struct CpuInfo {
    pub regs: Registers,
    pub flags: Flags,
}

#[derive(Debug)]
pub struct Statistics {
    pub cycles: u64,
    pub duration: Duration,
}

impl Backend {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            memory: Memory::new(),
            trap: AtomicBool::new(false),
            cycles: AtomicU64::new(0),
            duration_ns: AtomicU64::new(0),
        }
    }

    pub fn init(&mut self) {
        self.cpu.reset(&self.memory);
        self.memory[0x200] = 5;
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

    pub fn cpuinfo(&self) -> CpuInfo {
        CpuInfo {
            flags: self.cpu.flags,
            regs: self.cpu.regs,
        }
    }

    pub fn upload(&mut self, addr: u16, fpath: PathBuf) -> Result<usize, AppError> {
        if self.trap() {
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
            println!("pc: {:04X}", self.cpu.regs.pc);
            let cycles = self.cpu.exec_inst(&mut self.memory) as u64;
            let dt_ns = Instant::now().duration_since(t0).as_nanos() as u64;
            // sleep(Duration::from_nanos((period_ns * cycles).saturating_sub(dt_ns)));
            sleep(Duration::from_millis(1000));
            self.cycles.fetch_add(cycles, Relaxed);
            self.duration_ns.fetch_add((Instant::now() - t0).as_nanos() as u64, Relaxed);
            if cycles == 0 || self.trap() {
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
