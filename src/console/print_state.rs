use crate::error::Result;
use std::io::{stdout, Stdout, Write};

use crossterm::{
    queue,
    style::{Attribute, Print, PrintStyledContent, SetAttribute, Styler},
    QueueableCommand,
};

use crate::state::State;

impl State {
    fn label(&self, label: &str, text: &str) -> Result<()> {
        stdout()
            .queue(SetAttribute(Attribute::NormalIntensity))?
            .queue(Print(label))?
            .queue(Print(":"))?
            .queue(SetAttribute(Attribute::Bold))?
            .queue(Print(text))?;
        Ok(())
    }

    pub fn print(&self) -> Result<()> {
        self.label("PC", &format!("{:04X}", self.regs.pc))?;
        self.label(" SP", &format!("{:04X}", self.regs.sp as u16 | 0x100))?;
        self.label(" A", &format!("{:02X}", self.regs.a))?;
        self.label(" X", &format!("{:02X}", self.regs.x))?;
        self.label(" Y", &format!("{:02X}", self.regs.y))?;
        self.label(" P", &format!("{:08b}", self.flags.to_byte()))?;
        self.label(" trap", &format!("{}", self.trap))?;
        self.label(" clock", &format!("{}", self.frequency()))?;
        Ok(())
    }
}
