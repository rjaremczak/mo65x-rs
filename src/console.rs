mod textline;

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
use textline::TextLine;

use crate::{backend::Backend, error::Result, frontend::Frontend, mos6510::memory::Memory, state::State};

#[derive(Default)]
pub struct Console {
    cols: u16,
    rows: u16,
    header: TextLine,
    command: TextLine,
    status: TextLine,
}

impl Console {
    pub fn new(title: &str) -> Result<Self> {
        let mut c = Self::default();
        enable_raw_mode()?;
        stdout().queue(EnterAlternateScreen)?.queue(Clear(ClearType::All))?.flush()?;
        c.update_size()?;
        Ok(c)
    }

    fn update_size(&mut self) -> Result<()> {
        let (cols, rows) = size()?;
        self.cols = cols;
        self.rows = rows;
        self.header.update_width(self.cols);
        self.status.update_width(self.cols);
        Ok(())
    }

    fn set_size(&mut self, size: (u16, u16)) -> Result<()> {
        self.cols = size.0;
        self.rows = size.1;
        let tit = self.title.as_str().clone();
        self.set_header(tit);
        stdout().queue(Clear(ClearType::All))?;
        self.print_header()?;
        self.set_status("resize");
        Ok(())
    }

    fn print_all(&self) -> Result<()> {
        self.print_header()?;
        self.print_status()?;
        Ok(())
    }

    fn init(&self) -> Result<()> {
        enable_raw_mode()?;
        stdout().queue(EnterAlternateScreen)?.queue(Clear(ClearType::All))?.flush()?;
        Ok(())
    }

    fn set_header(&mut self, txt: &str) {
        self.header = format!("{:width$}", txt, width = self.cols as usize).reverse();
    }

    fn print_header(&self) -> Result<()> {
        stdout().queue(MoveTo(0, 0))?.queue(PrintStyledContent(self.header.clone()))?;
        Ok(())
    }

    fn set_status(&mut self, txt: &str) {
        self.status = format!("{:width$}", txt, width = self.cols as usize).red();
    }

    fn print_status(&self) -> Result<()> {
        stdout()
            .queue(MoveTo(0, self.rows - 1))?
            .queue(PrintStyledContent(self.status.clone()))?;
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
        if poll(Duration::from_millis(2))? {
            match event::read() {
                Ok(Event::Key(KeyEvent {
                    code: KeyCode::Char(c), ..
                })) => self.process_char(c)?,
                Ok(Event::Key(KeyEvent { code: KeyCode::Esc, .. })) => return Ok(false),
                Ok(Event::Resize(cols, rows)) => self.set_size((cols, rows))?,
                Ok(event) => {
                    self.set_status(&format!("unhandled event: {:?}", event));
                    self.print_status()?;
                }
                Err(err) => {
                    self.set_status(&format!("event handling error: {:?}", err));
                    self.print_status()?;
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
