use crossterm::style::{style, ContentStyle, StyledContent};

#[derive(Default)]
pub struct TextLine {
    pub col: u16,
    pub row: u16,
    pub width: u16,
    pub text: String,
    pub style: ContentStyle,
    rendered: Option<StyledContent<String>>,
}

impl TextLine {
    pub fn content(&self) -> StyledContent<String> {
        self.rendered.unwrap()
    }

    pub fn update_width(&mut self, width: u16) {
        self.width = width;
        self.render();
    }

    fn render(&mut self) {
        self.rendered = Some(StyledContent::new(self.style, self.text.clone()));
    }
}
