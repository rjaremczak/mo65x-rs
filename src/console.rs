mod print_state;
mod textline;

use std::{
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor::{DisableBlinking, Hide, MoveTo, MoveToNextLine},
    event::{self, poll, Event, KeyCode, KeyEvent},
    style::{
        style,
        Attribute::{Reset, Reverse},
        ContentStyle, PrintStyledContent,
    },
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, DisableLineWrap, EnableLineWrap, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand, QueueableCommand,
};

use textline::TextLine;

use crate::{backend::Backend, error::Result, frontend::Frontend, state::State};

pub struct Console {
    size: (u16, u16),
    header: TextLine,
    state: State,
    command: TextLine,
    status: TextLine,
}

impl Drop for Console {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        stdout().execute(EnableLineWrap).unwrap();
        stdout().execute(LeaveAlternateScreen).unwrap();
    }
}

impl Console {
    pub fn new(title: &str) -> Result<Self> {
        let mut console = Self {
            size: size()?,
            header: TextLine::new(title, ContentStyle::new().attribute(Reverse)),
            state: State::default(),
            command: TextLine::new("command...", ContentStyle::new().foreground(crossterm::style::Color::White)),
            status: TextLine::new("ok", ContentStyle::new().attribute(Reset)),
        };
        enable_raw_mode()?;
        stdout().queue(EnterAlternateScreen)?.queue(DisableLineWrap)?;
        console.clear()?;
        console.update_size();
        console.print()?;
        stdout().flush()?;
        Ok(console)
    }

    fn update_size(&mut self) {
        self.header.width = self.cols();
        self.header.update();

        self.command.width = self.cols();
        self.command.update();

        self.status.width = self.cols();
        self.status.update();
    }

    pub fn cols(&self) -> u16 {
        self.size.0
    }

    pub fn rows(&self) -> u16 {
        self.size.1
    }

    fn clear(&self) -> Result<()> {
        stdout().queue(Clear(ClearType::All))?;
        Ok(())
    }

    fn print(&mut self) -> Result<()> {
        stdout().queue(Hide)?.queue(DisableBlinking)?.queue(MoveTo(0, 0))?;
        self.header.print()?;
        stdout().queue(MoveToNextLine(1))?;
        self.state.print();
        stdout().queue(MoveTo(0, self.rows() - 1))?;
        self.status.print()
    }

    fn process_char(&mut self, c: char) -> Result<()> {
        self.status.text = format!("received: {}", c);
        self.status.update();
        self.status.print()
    }

    fn on_resize(&mut self, cols: u16, rows: u16) -> Result<()> {
        if cols != self.size.0 || rows != self.size.1 {
            self.size = (cols, rows);
            self.status.text = format!("resize({},{})", cols, rows);
            self.update_size();
            self.clear()?;
            self.print()?;
        }
        Ok(())
    }

    pub fn process(&mut self, backend: &mut Backend, frontend: &mut Frontend) -> Result<bool> {
        if poll(Duration::from_millis(2))? {
            match event::read() {
                Ok(Event::Key(KeyEvent {
                    code: KeyCode::Char(c), ..
                })) => self.process_char(c)?,
                Ok(Event::Key(KeyEvent { code: KeyCode::Esc, .. })) => return Ok(false),
                Ok(Event::Resize(cols, rows)) => self.on_resize(cols, rows)?,
                Ok(event) => {
                    self.status.text = format!("unhandled event: {:?}", event);
                    self.status.update();
                    self.status.print()?;
                }
                Err(err) => {
                    self.status.text = format!("event handling error: {:?}", err);
                    self.status.update();
                    self.status.print()?;
                }
            }
            stdout().flush()?;
        }
        Ok(true)
    }
}
