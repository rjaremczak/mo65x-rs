use crate::error::Result;
use crossterm::{
    cursor::MoveTo,
    queue,
    style::{ContentStyle, PrintStyledContent, StyledContent},
    QueueableCommand,
};
use std::io::stdout;

pub struct TextLine {
    pub width: u16,
    pub text: String,
    pub style: ContentStyle,
    content: StyledContent<String>,
}

impl TextLine {
    pub fn new(text: &str, style: ContentStyle) -> Self {
        Self {
            width: 0,
            text: String::from(text),
            style,
            content: StyledContent::new(style, String::from(text)),
        }
    }

    pub fn print(&self) -> Result<()> {
        stdout().queue(PrintStyledContent(self.content.clone()))?;
        Ok(())
    }

    pub fn update(&mut self) {
        self.content = StyledContent::new(self.style, format!("{:width$}", self.text, width = self.width as usize));
    }
}
