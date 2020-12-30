use regex::{Captures, Regex};

pub enum Command {
    SetPC(u16),
    SetSP(u8),
    SetA(u8),
    SetX(u8),
    SetY(u8),
}

type Parser = fn(Captures) -> Command;

pub struct CommandParser {
    parsers: Vec<(Regex, Parser)>,
}

fn rx(pattern: &str) -> Regex {
    Regex::new(&format!("(?i){}\\s*", pattern)).unwrap()
}

fn rx_set(name: &str, max_digits: u8) -> Regex {
    Regex::new(&format!("(?i){}\\s*=\\s*([0-9a-f]{{1,{}}})\\s*", name, max_digits)).unwrap()
}

fn arg(captures: Captures) -> u16 {
    u16::from_str_radix(captures.get(1).unwrap().as_str(), 16).unwrap()
}

impl CommandParser {
    pub fn new() -> Self {
        Self {
            parsers: vec![
                (rx_set("pc", 4), |c| Command::SetPC(arg(c))),
                (rx_set("sp", 2), |c| Command::SetSP(arg(c) as u8)),
                (rx_set("a", 2), |c| Command::SetA(arg(c) as u8)),
                (rx_set("x", 2), |c| Command::SetX(arg(c) as u8)),
                (rx_set("y", 2), |c| Command::SetY(arg(c) as u8)),
            ],
        }
    }

    pub fn parse(&self, text: &str) -> Option<Command> {
        for (regex, parser) in self.parsers.iter() {
            if let Some(captures) = regex.captures(text) {
                return Some(parser(captures));
            }
        }
        None
    }
}
