mod textline;

use std::{
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor::MoveTo,
    event::{self, poll, Event, KeyCode, KeyEvent},
    style::{style, PrintStyledContent},
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};

use textline::TextLine;

use crate::{backend::Backend, error::Result, frontend::Frontend, mos6510::memory::Memory, state::State};

pub struct Console {
    cols: u16,
    rows: u16,
    header: TextLine,
    command: TextLine,
    status: TextLine,
}

impl Drop for Console {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        stdout().execute(LeaveAlternateScreen).unwrap();
    }
}

impl Console {
    pub fn new() -> Self {
        Self {
            cols: 0,
            rows: 0,
            header: TextLine::default(),
            command: TextLine::default(),
            status: TextLine::default(),
        }
    }

    pub fn init(&mut self, title: &str) -> Result<()> {
        enable_raw_mode()?;
        stdout().queue(EnterAlternateScreen)?.queue(Clear(ClearType::All))?.flush()?;
        self.header.text = String::from(title);
        self.update_size();
        Ok(())
    }

    fn update_size(&mut self) {
        let (cols, rows) = size().unwrap();
        self.cols = cols;
        self.rows = rows;
        self.header.update_width(self.cols);
        self.status.update_width(self.cols);
    }

    fn print_all(&self) {
        self.header.print();
        self.status.print();
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
                Ok(Event::Resize(cols, rows)) => self.update_size(),
                Ok(event) => {
                    self.status.update_text(&format!("unhandled event: {:?}", event));
                    self.status.print();
                }
                Err(err) => {
                    self.status.update_text(&format!("event handling error: {:?}", err));
                    self.status.print();
                }
            }
            stdout().flush();
        }
        Ok(true)
    }
}
