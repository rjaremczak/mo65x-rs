use regex::{Captures, Regex};

pub enum Command {
    SetPC(u16),
    SetSP(u8),
    SetA(u8),
    SetX(u8),
    SetY(u8),
    SetMemoryByte(u16, u8),
    Load(u16, String),
}

type Parser = fn(&Captures) -> Command;

const HEX_U16: &str = "[0-9a-f]{1,4}";
const HEX_U8: &str = "[0-9a-f]{1,4}";

pub struct CommandParser {
    parsers: Vec<(Regex, Parser)>,
}

fn rx(text: &str) -> Regex {
    Regex::new(&format!("(?i){}\\s*", text)).unwrap()
}

fn set(name: &str, max_digits: u8) -> Regex {
    Regex::new(&format!("(?i)({})\\s*=\\s*([0-9a-f]{{1,{}}})\\s*", name, max_digits)).unwrap()
}

fn arg(captures: &Captures, i: usize) -> String {
    String::from(captures.get(i).unwrap().as_str())
}

fn hex(captures: &Captures, i: usize) -> u16 {
    u16::from_str_radix(&arg(captures, i), 16).unwrap()
}

impl CommandParser {
    pub fn new() -> Self {
        Self {
            parsers: vec![
                (set("pc", 4), |c| Command::SetPC(hex(c, 2))),
                (set("sp", 2), |c| Command::SetSP(hex(c, 2) as u8)),
                (set("a", 2), |c| Command::SetA(hex(c, 2) as u8)),
                (set("x", 2), |c| Command::SetX(hex(c, 2) as u8)),
                (set("y", 2), |c| Command::SetY(hex(c, 2) as u8)),
                (set(HEX_U16, 2), |c| Command::SetMemoryByte(hex(c, 1), hex(c, 2) as u8)),
                (rx(&format!("l\\s+{}\\s+\\S+", HEX_U16)), |c| Command::Load(hex(c, 1), arg(c, 2))),
            ],
        }
    }

    pub fn parse(&self, text: &str) -> Option<Command> {
        for (regex, parser) in self.parsers.iter() {
            if let Some(captures) = regex.captures(text) {
                return Some(parser(&captures));
            }
        }
        None
    }
}
