use std::io::{stdout, Write};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    style::{Colorize, PrintStyledContent},
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use crate::{error::Result, mos6510::memory::Memory, state::State};

pub struct Console {
    title: String,
    header: String,
    cols: u16,
    rows: u16,
}

impl Console {
    pub fn new(title: String) -> Result<Self> {
        let size = size()?;
        let mut c = Self {
            title,
            header: String::new(),
            cols: size.0,
            rows: size.1,
        };
        c.init()?;
        Ok(c)
    }

    fn set_size(&mut self, size: (u16, u16)) {
        self.cols = size.0;
        self.rows = size.1;
    }

    fn init(&self) -> Result<()> {
        stdout().execute(EnterAlternateScreen)?.execute(Clear(ClearType::All))?;
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
        stdout().execute(LeaveAlternateScreen).unwrap();
    }
}
