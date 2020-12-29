use crate::state::State;
use crate::terminal;

impl State {
    fn label(&self, label: &str, text: &str) {
        terminal::normal();
        terminal::queue(label);
        terminal::queue(":");
        terminal::bold();
        terminal::queue(text);
    }

    pub fn queue(&self) {
        self.label("PC", &format!("{:04X}", self.regs.pc));
        self.label(" SP", &format!("{:04X}", self.regs.sp as u16 | 0x100));
        self.label(" A", &format!("{:02X}", self.regs.a));
        self.label(" X", &format!("{:02X}", self.regs.x));
        self.label(" Y", &format!("{:02X}", self.regs.y));
        self.label(" P", &format!("{:08b}", self.flags.to_byte()));
        if self.cycles > 0 {
            self.label(" f", &format!("{}", self.frequency()));
        }
        terminal::bold();
        terminal::queue(match self.trap {
            true => " step",
            false => " run",
        });
    }
}
