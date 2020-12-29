use crate::info::Info;
use crate::terminal;

impl Info {
    fn label(&self, label: &str, text: &str) {
        terminal::dim();
        terminal::print(label);
        terminal::print(":");
        terminal::bold();
        terminal::print(text);
    }

    pub fn print(&self) {
        self.label("PC", &format!("{:04X}", self.regs.pc));
        self.label(" SP", &format!("{:04X}", self.regs.sp as u16 | 0x100));
        self.label(" A", &format!("{:02X}", self.regs.a));
        self.label(" X", &format!("{:02X}", self.regs.x));
        self.label(" Y", &format!("{:02X}", self.regs.y));
        self.label(" P", &format!("{:08b}", self.flags.to_byte()));
        self.label(
            " T",
            match self.trap {
                true => "on",
                false => "off",
            },
        );
        if self.cycles > 0 {
            self.label(" F", &format!("{}", self.frequency()));
        }
    }
}
