mod print_info;
mod textline;

use std::{
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    event::{self, poll, Event, KeyCode, KeyEvent},
    style::{Attribute::Reset, ContentStyle},
};

use crate::{backend::Backend, error::Result, frontend::Frontend, info::Info, terminal};

pub struct Console {
    size: (u16, u16),
    title: String,
    info: Info,
    command: String,
    status: String,
}

impl Drop for Console {
    fn drop(&mut self) {
        terminal::end_session()
    }
}

const PROMPT: &str = "> ";
const STATUS_OK: &str = "Ok";

impl Console {
    pub fn new(title: &str) -> Self {
        let mut console = Self {
            size: terminal::size(),
            title: String::from(title),
            info: Info::default(),
            command: String::default(),
            status: String::from(STATUS_OK),
        };
        terminal::begin_session();
        console.print();
        terminal::flush();
        console
    }

    fn print(&mut self) {
        terminal::hide_cursor();
        self.print_header();
        self.info.print();
        self.print_status();
        self.print_command();
        terminal::show_cursor();
    }

    fn print_header(&self) {
        terminal::move_cursor(0, 0);
        terminal::special();
        terminal::print(&self.title);
        terminal::normal();
        terminal::print(" ");
    }

    fn print_status(&self) {
        terminal::move_cursor(0, self.size.1);
        terminal::message();
        terminal::print(&format!("{:1$}", self.status, self.size.0 as usize));
    }

    fn print_command(&self) {
        terminal::move_cursor(0, self.size.1 - 1);
        terminal::normal();
        terminal::print(PROMPT);
        terminal::input();
        terminal::print(&self.command);
    }

    fn process_char(&mut self, c: char) {
        self.command.push(c);
        terminal::print(&self.command[self.command.len() - 1..]);
    }

    fn on_resize(&mut self, cols: u16, rows: u16) {
        if cols != self.size.0 || rows != self.size.1 {
            self.size = (cols, rows);
            self.status = format!("resize({},{})", cols, rows);
            terminal::clear();
            self.print();
        }
    }

    pub fn process(&mut self, backend: &mut Backend, frontend: &mut Frontend) -> bool {
        if poll(Duration::from_millis(2)).unwrap() {
            match event::read() {
                Ok(Event::Key(KeyEvent {
                    code: KeyCode::Char(c), ..
                })) => self.process_char(c),
                Ok(Event::Key(KeyEvent {
                    code: KeyCode::Backspace, ..
                })) => terminal::backspace(),
                Ok(Event::Key(KeyEvent { code: KeyCode::Esc, .. })) => return false,
                Ok(Event::Resize(cols, rows)) => self.on_resize(cols, rows),
                Ok(event) => {
                    self.status = format!("unhandled event: {:?}", event);
                    terminal::store_cursor();
                    self.print_status();
                    terminal::restore_cursor();
                }
                Err(err) => {
                    self.status = format!("event handling error: {:?}", err);
                    self.print_status();
                }
            }
            terminal::flush();
        }
        true
    }
}
