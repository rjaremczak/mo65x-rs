use regex::{Captures, Regex};

pub enum Command {
    SetPC(u16),
    SetSP(u8),
    SetA(u8),
    SetX(u8),
    SetY(u8),
    SetFlagN(bool),
    SetFlagV(bool),
    SetFlagD(bool),
    SetFlagI(bool),
    SetFlagZ(bool),
    SetFlagC(bool),
    SetByte(u16, u8),
    SetWord(u16, u16),
    Load(u16, String),
    Disassemble(u16),
    MemoryDump(u16),
    Reset,
    Nmi,
    Irq,
}

type Parser = fn(&Captures) -> Command;

pub struct CommandParser {
    parsers: Vec<(Regex, Parser)>,
}

fn rx(text: &str) -> Regex {
    Regex::new(&format!("(?i){}\\s*", text)).unwrap()
}

fn set(name: &str, max_digits: u8) -> Regex {
    Regex::new(&format!("(?i)({})\\s*=\\s*([0-9a-f]{{1,{}}})\\s*", name, max_digits)).unwrap()
}

fn set_bool(name: &str) -> Regex {
    Regex::new(&format!("(?i)({})\\s*=\\s*([0|1])\\s*", name)).unwrap()
}

fn arg(captures: &Captures, i: usize) -> String {
    String::from(captures.get(i).unwrap().as_str())
}

fn hex(captures: &Captures, i: usize) -> u16 {
    u16::from_str_radix(&arg(captures, i), 16).unwrap()
}

fn bin(captures: &Captures) -> bool {
    if u16::from_str_radix(&arg(captures, 2), 16).unwrap() == 0 {
        false
    } else {
        true
    }
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
                (set_bool("n"), |c| Command::SetFlagN(bin(c))),
                (set_bool("v"), |c| Command::SetFlagV(bin(c))),
                (set_bool("d"), |c| Command::SetFlagD(bin(c))),
                (set_bool("i"), |c| Command::SetFlagI(bin(c))),
                (set_bool("z"), |c| Command::SetFlagZ(bin(c))),
                (set_bool("c"), |c| Command::SetFlagC(bin(c))),
                (rx("sb\\s*([0-9a-f]{1,4})\\s+([0-9a-f]{1,2})"), |c| {
                    Command::SetByte(hex(c, 1), hex(c, 2) as u8)
                }),
                (rx("sw\\s*([0-9a-f]{1,4})\\s+([0-9a-f]{1,4})"), |c| {
                    Command::SetWord(hex(c, 1), hex(c, 2))
                }),
                (rx("l\\s*([0-9a-f]{1,4})\\s+(\\S+)"), |c| Command::Load(hex(c, 1), arg(c, 2))),
                (rx("d\\s*([0-9a-f]{1,4})"), |c| Command::Disassemble(hex(c, 1))),
                (rx("m\\s*([0-9a-f]{1,4})"), |c| Command::MemoryDump(hex(c, 1))),
                (rx("reset"), |_| Command::Reset),
                (rx("nmi"), |_| Command::Nmi),
                (rx("irq"), |_| Command::Irq),
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
