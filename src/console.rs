mod textline;

use std::{
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor::{DisableBlinking, Hide, MoveTo},
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

use crate::{backend::Backend, error::Result, frontend::Frontend};

pub struct Console {
    size: (u16, u16),
    header: TextLine,
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
            command: TextLine::new("command...", ContentStyle::new().foreground(crossterm::style::Color::White)),
            status: TextLine::new("ok", ContentStyle::new().attribute(Reset)),
        };
        enable_raw_mode()?;
        stdout().queue(EnterAlternateScreen)?.queue(DisableLineWrap)?;
        console.clear()?;
        console.update();
        console.print()?;
        stdout().flush()?;
        Ok(console)
    }

    fn update(&mut self) {
        self.header.width = self.cols();
        self.header.update();

        self.command.width = self.cols();
        self.command.row = self.rows() - 2;
        self.command.update();

        self.status.width = self.cols();
        self.status.row = self.rows() - 1;
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
        stdout().queue(Hide)?.queue(DisableBlinking)?;
        self.header.print()?;
        self.status.print()
    }

    fn process_char(&mut self, c: char) -> Result<()> {
        self.status.text = format!("received: {}", c);
        self.status.update();
        self.status.print()
    }

    pub fn process(&mut self, backend: &mut Backend, frontend: &mut Frontend) -> Result<bool> {
        if poll(Duration::from_millis(2))? {
            match event::read() {
                Ok(Event::Key(KeyEvent {
                    code: KeyCode::Char(c), ..
                })) => self.process_char(c)?,
                Ok(Event::Key(KeyEvent { code: KeyCode::Esc, .. })) => return Ok(false),
                Ok(Event::Resize(cols, rows)) => {
                    if cols != self.size.0 || rows != self.size.1 {
                        self.size = (cols, rows);
                        self.status.text = format!("resize({},{})", cols, rows);
                        self.update();
                        self.clear()?;
                        self.print()?;
                    } else {
                        self.status.text = String::from("resize event without actual change");
                        self.status.update();
                        self.status.print()?;
                    }
                }
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
