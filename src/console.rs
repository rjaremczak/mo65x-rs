use std::io::{stdout, Write};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    style::{Colorize, PrintStyledContent},
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};

use crate::{error::Result, mos6510::memory::Memory, state::State};

pub struct Console {
    header: String,
}

impl Console {
    pub fn new(header: String) -> Result<Self> {
        let mut c = Self { header };
        c.init()?;
        Ok(c)
    }

    fn init(&self) -> Result<()> {
        enable_raw_mode()?;
        Ok(())
    }

    pub fn update(&mut self, memory: &Memory, state: State) -> Result<()> {
        Ok(())
    }

    pub fn process(&mut self) -> Result<()> {
        let mut stdout = stdout();
        //stdout.queue(MoveTo(5, 5))?.queue(Clear(ClearType::All))?;
        stdout.flush()?;
        let c = self.getchar();
        stdout.execute(PrintStyledContent(format!("received: {}", c).magenta()))?;
        Ok(())
    }

    fn getchar(&self) -> char {
        loop {
            match event::read() {
                Ok(Event::Key(KeyEvent {
                    code: KeyCode::Char(c), ..
                })) => {
                    return c;
                }
                Ok(event) => println!("event: {:?}", event),
                _ => {}
            }
        }
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
    }
}
