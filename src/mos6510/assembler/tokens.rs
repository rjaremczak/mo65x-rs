use regex::Captures;

pub struct Tokens<'a> {
    pub captures: Captures<'a>,
}

impl<'a> Tokens<'a> {
    pub fn new(captures: Captures) -> Tokens {
        Tokens { captures }
    }

    pub fn label(&self) -> Option<&str> {
        self.get_str(1)
    }

    pub fn operation(&self) -> Option<&str> {
        self.get_str(2)
    }

    pub fn operand(&self) -> Option<&str> {
        self.get_str(3)
    }

    fn get_str(&self, i: usize) -> Option<&str> {
        self.captures.get(i).map_or(None, |m| Some(m.as_str().trim()))
    }
}
