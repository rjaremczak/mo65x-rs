use std::io::{stdout, Write};

const SPACE: &str = " ";

use crossterm::{
    cursor::{position, EnableBlinking, Hide, MoveTo, Show},
    execute,
    style::{Attribute, Print, SetAttribute},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, DisableLineWrap, EnableLineWrap, EnterAlternateScreen, LeaveAlternateScreen,
    },
    ExecutableCommand, QueueableCommand,
};

#[inline]
pub fn bold() {
    stdout().queue(SetAttribute(Attribute::Bold)).unwrap();
}

#[inline]
pub fn normal() {
    stdout()
        .queue(SetAttribute(Attribute::NormalIntensity))
        .unwrap()
        .queue(SetAttribute(Attribute::NoReverse))
        .unwrap();
}

#[inline]
pub fn reverse() {
    stdout().queue(SetAttribute(Attribute::Reverse)).unwrap();
}

#[inline]
pub fn queue(text: &str) {
    stdout().queue(Print(text)).unwrap();
}

pub fn fill(endpos: usize) {
    stdout()
        .queue(Print(SPACE.repeat(endpos - position().unwrap().0 as usize)))
        .unwrap();
}

#[inline]
pub fn flush() {
    stdout().flush();
}

#[inline]
pub fn move_cursor(col: usize, row: usize) {
    stdout().queue(MoveTo(col as u16, row as u16)).unwrap();
}

#[inline]
pub fn clear() {
    stdout().execute(Clear(ClearType::All)).unwrap();
}

pub fn hide_cursor() {
    stdout().execute(Hide);
}

pub fn show_cursor() {
    stdout().execute(Show);
}

pub fn begin_session() {
    enable_raw_mode().unwrap();
    execute!(stdout(), EnterAlternateScreen, DisableLineWrap, Clear(ClearType::All)).unwrap();
}

pub fn end_session() {
    execute!(stdout(), LeaveAlternateScreen, EnableLineWrap, EnableBlinking).unwrap();
    disable_raw_mode().unwrap();
}
