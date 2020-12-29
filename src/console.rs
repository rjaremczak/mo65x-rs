mod print_state;
mod textline;

use std::{
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor::{position, DisableBlinking, EnableBlinking, Hide, MoveTo, MoveToNextLine},
    event::{self, poll, Event, KeyCode, KeyEvent},
    style::{
        style,
        Attribute::{self, Reset, Reverse},
        ContentStyle, Print, PrintStyledContent, SetAttribute,
    },
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, DisableLineWrap, EnableLineWrap, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand, QueueableCommand,
};

use textline::TextLine;

use crate::{backend::Backend, error::Result, frontend::Frontend, state::State, terminal};

pub struct Console {
    size: (u16, u16),
    title: String,
    state: State,
    command: TextLine,
    status: TextLine,
}

impl Drop for Console {
    fn drop(&mut self) {
        terminal::end_session()
    }
}

impl Console {
    pub fn new(title: &str) -> Result<Self> {
        let mut console = Self {
            size: size()?,
            title: String::from(title),
            state: State::default(),
            command: TextLine::new("command...", ContentStyle::new().foreground(crossterm::style::Color::White)),
            status: TextLine::new("ok", ContentStyle::new().attribute(Reset)),
        };
        terminal::begin_session();
        console.update_size();
        console.print()?;
        stdout().flush()?;
        Ok(console)
    }

    fn update_size(&mut self) {
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

    fn print(&mut self) -> Result<()> {
        stdout().queue(Hide)?.queue(MoveTo(0, 0))?;
        terminal::reverse();
        terminal::queue(&format!(" {} ", self.title));
        terminal::normal();
        self.state.queue();
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
            terminal::clear();
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
