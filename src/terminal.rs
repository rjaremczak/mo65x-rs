use std::io::{stdout, Write};

const SPACE: &str = " ";

use crossterm::{
    cursor::{position, EnableBlinking, Hide, MoveLeft, MoveTo, MoveToColumn, MoveToNextLine, RestorePosition, SavePosition, Show},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, DisableLineWrap, EnableLineWrap, EnterAlternateScreen, LeaveAlternateScreen,
    },
    ExecutableCommand, QueueableCommand,
};

#[inline]
pub fn bold() {
    stdout()
        .queue(SetAttribute(Attribute::Bold))
        .unwrap()
        .queue(SetForegroundColor(Color::White))
        .unwrap();
}

#[inline]
pub fn normal() {
    stdout().queue(SetAttribute(Attribute::Reset)).unwrap().queue(ResetColor).unwrap();
}

#[inline]
pub fn reverse() {
    stdout().queue(SetAttribute(Attribute::Reverse)).unwrap();
}

#[inline]
pub fn highlight() {
    stdout().queue(SetForegroundColor(Color::Yellow)).unwrap();
}

#[inline]
pub fn dim() {
    stdout().queue(SetForegroundColor(Color::DarkGrey)).unwrap();
}

#[inline]
pub fn print(text: &str) {
    stdout().queue(Print(text)).unwrap();
}

#[inline]
pub fn clear_line() {
    stdout().queue(Clear(ClearType::CurrentLine)).unwrap();
}

#[inline]
pub fn set_cursor_col(col: u16) {
    stdout().queue(MoveToColumn(col)).unwrap();
}

#[inline]
pub fn flush() {
    stdout().flush().unwrap();
}

#[inline]
pub fn set_cursor_pos(col: u16, row: u16) {
    stdout().queue(MoveTo(col, row)).unwrap();
}

#[inline]
pub fn clear() {
    stdout().execute(Clear(ClearType::All)).unwrap();
}

#[inline]
pub fn size() -> (u16, u16) {
    crossterm::terminal::size().unwrap()
}

#[inline]
pub fn cursor_pos() -> (u16, u16) {
    crossterm::cursor::position().unwrap()
}

#[inline]
pub fn show_cursor() {
    stdout().execute(Show).unwrap();
}

#[inline]
pub fn store_cursor() {
    stdout().execute(SavePosition).unwrap();
}

#[inline]
pub fn restore_cursor() {
    stdout().execute(RestorePosition).unwrap();
}

pub fn backspace() {
    execute!(stdout(), MoveLeft(1), Print(" "), MoveLeft(1)).unwrap();
}

pub fn begin_session() {
    enable_raw_mode().unwrap();
    execute!(stdout(), EnterAlternateScreen, DisableLineWrap, Clear(ClearType::All)).unwrap();
}

pub fn end_session() {
    execute!(stdout(), LeaveAlternateScreen, EnableLineWrap, EnableBlinking).unwrap();
    disable_raw_mode().unwrap();
}

pub fn newline() {
    stdout().queue(MoveToNextLine(1));
}
