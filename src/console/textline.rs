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

impl Default for TextLine {
    fn default() -> Self {
        Self {
            col: 0,
            row: 0,
            width: 0,
            text: String::default(),
            style: ContentStyle::default(),
            content: StyledContent::new(ContentStyle::default(), String::default()),
        }
    }
}

impl TextLine {
    pub fn update_width(&mut self, width: u16) {
        self.width = width;
        self.update();
    }

    pub fn update_text(&mut self, text: &str) {
        self.text = format!("{:width$}", text, width = self.width as usize);
        self.update()
    }

    pub fn print(&self) {
        let mut stdout = stdout();
        stdout.queue(MoveTo(self.col, self.row)).unwrap();
        stdout.queue(PrintStyledContent(self.content.clone())).unwrap();
    }

    fn update(&mut self) {
        self.content = StyledContent::new(self.style, self.text.clone());
    }
}
