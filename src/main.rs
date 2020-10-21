mod mos6510;

use mos6510::asm::assembler::Assembler;

//mod gui_iced;
//mod gui_druid;

fn main() {
    //gui_iced::run();
    //let _ = gui_druid::run();
    //let mut memory: mos6510::Memory;
    println!("opcodes {}", mos6510::opcode::OPCODES[0].size);
    let mut assembler = Assembler::new();
}
