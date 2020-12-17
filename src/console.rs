use std::{
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor::MoveTo,
    event::{self, poll, Event, KeyCode, KeyEvent},
    style::{self, style, Attribute, Color, Colorize, PrintStyledContent, StyledContent, Styler},
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, SetTitle},
    ExecutableCommand, QueueableCommand,
};
use style::{Attributes, ContentStyle};

use crate::{backend::Backend, error::Result, frontend::Frontend, mos6510::memory::Memory, state::State};

pub struct Console {
    cols: u16,
    rows: u16,
    title: String,
    header: String,
    command: String,
    status: String,
}

impl Console {
    pub fn new(title: &str) -> Result<Self> {
        let mut c = Self {
            cols: 0,
            rows: 0,
            title: String::from(title),
            header: String::from(title),
            command: String::new(),
            status: String::new(),
        };
        c.init()?;
        c.resize(size()?)?;
        stdout().flush()?;
        Ok(c)
    }

    fn resize(&mut self, size: (u16, u16)) -> Result<()> {
        self.cols = size.0;
        self.rows = size.1;
        stdout().queue(Clear(ClearType::All))?;
        self.print_header()?;
        self.update_status("resize".to_string())?;
        Ok(())
    }

    fn init(&self) -> Result<()> {
        enable_raw_mode()?;
        stdout()
            .queue(SetTitle(&self.title))?
            .queue(EnterAlternateScreen)?
            .queue(Clear(ClearType::All))?
            .flush()?;
        Ok(())
    }

    fn print_header(&self) -> Result<()> {
        stdout()
            .queue(MoveTo(0, 0))?
            .queue(PrintStyledContent(self.header.as_str().reverse()))?;
        Ok(())
    }

    fn print_status(&self) -> Result<()> {
        stdout()
            .queue(MoveTo(0, self.rows - 1))?
            .queue(PrintStyledContent(self.status.as_str().red()))?;
        Ok(())
    }

    fn update_status(&mut self, status: String) -> Result<()> {
        self.status = status;
        self.print_status()?;
        Ok(())
    }

    pub fn update(&mut self, memory: &Memory, state: State) -> Result<()> {
        Ok(())
    }

    fn process_char(&self, c: char) -> Result<()> {
        stdout()
            .queue(MoveTo(0, self.rows - 1))?
            .queue(PrintStyledContent(style(format!("received: {}", c))))?;
        Ok(())
    }

    pub fn process(&mut self, backend: &mut Backend, frontend: &mut Frontend) -> Result<bool> {
        if poll(Duration::from_millis(5))? {
            match event::read() {
                Ok(Event::Key(KeyEvent {
                    code: KeyCode::Char(c), ..
                })) => self.process_char(c)?,
                Ok(Event::Key(KeyEvent { code: KeyCode::Esc, .. })) => return Ok(false),
                Ok(Event::Resize(cols, rows)) => self.resize((cols, rows))?,
                Ok(event) => {
                    self.update_status(format!("unhandled event: {:?}", event))?;
                }
                Err(err) => {
                    self.update_status(format!("event handling error: {:?}", err))?;
                }
            }
            stdout().flush()?;
        }
        Ok(true)
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        stdout().execute(LeaveAlternateScreen).unwrap();
    }
}
