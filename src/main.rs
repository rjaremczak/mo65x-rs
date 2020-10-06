//mod gui_iced;
//mod gui_druid;
mod mos6510;

fn main() {
    //gui_iced::run();
    //let _ = gui_druid::run();
    //let mut memory: mos6510::Memory;
    println!("opcodes {}", mos6510::OPCODES[0].size);
    let mut assembler = mos6510::Assembler::new();
}
