//mod gui_iced;
//mod gui_druid;
mod mos6510;

fn main() {
    //gui_iced::run();
    //let _ = gui_druid::run();
    println!("opcodes {}", mos6510::OPCODES[0].size);
}
