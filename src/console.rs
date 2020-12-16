use std::io::{stdout, Write};

use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyEvent},
    style::{Colorize, PrintStyledContent},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};

use crate::mos6510::{cpuinfo::CpuInfo, error::AppError, memory::Memory, statistics::Statistics};

pub struct Console {}

impl Console {
    pub fn new() -> Result<Self, AppError> {
        let mut c = Self {};
        c.init()?;
        Ok(c)
    }

    fn init(&self) -> Result<(), AppError> {
        enable_raw_mode()?;
        Ok(())
    }

    pub fn update(&mut self, memory: &Memory, statistics: Statistics, cpuinfo: CpuInfo) -> Result<(), AppError> {
        Ok(())
    }

    pub fn process(&mut self) -> Result<(), AppError> {
        let mut stdout = stdout();
        stdout.queue(MoveTo(5, 5))?.queue(Clear(ClearType::All))?;
        stdout.flush()?;
        let c = self.readc();
        stdout
            .execute(MoveTo(6, 6))?
            .execute(PrintStyledContent(format!("received {}", c).magenta()))?;
        Ok(())
    }

    fn readc(&self) -> char {
        loop {
            if let Ok(Event::Key(KeyEvent {
                code: KeyCode::Char(c), ..
            })) = event::read()
            {
                return c;
            }
        }
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
    }
}
