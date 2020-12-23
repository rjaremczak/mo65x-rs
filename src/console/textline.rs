use crate::error::Result;
use crossterm::{
    cursor::MoveTo,
    style::{ContentStyle, PrintStyledContent, StyledContent},
    QueueableCommand,
};
use std::io::stdout;

pub struct TextLine {
    pub col: u16,
    pub row: u16,
    pub width: u16,
    pub text: String,
    pub style: ContentStyle,
    content: StyledContent<String>,
}

impl TextLine {
    pub fn new(text: &str, style: ContentStyle) -> Self {
        Self {
            col: 0,
            row: 0,
            width: 0,
            text: String::from(text),
            style,
            content: StyledContent::new(style, String::from(text)),
        }
    }

    pub fn print(&self) -> Result<()> {
        stdout()
            .queue(MoveTo(self.col, self.row))?
            .queue(PrintStyledContent(self.content.clone()))?;
        Ok(())
    }

    pub fn update(&mut self) {
        self.content = StyledContent::new(self.style, format!("{:width$}", self.text, width = self.width as usize));
    }
}
