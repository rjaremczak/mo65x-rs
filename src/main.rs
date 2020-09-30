//mod gui_iced;
mod gui_druid;
mod mos6510;

use mos6510::*;

fn main() {
    //gui_iced::run();
    //let _ = gui_druid::run();
    println!("opcodes {}", op_code::OPCODES[0].size);
}
