mod emulator;
mod gui_iced;
mod mos6510;

use emulator::Emulator;

fn main() {
    let mut emulator = Emulator::new();
    let mut assembler = mos6510::assembler::Assembler::new(0);
    emulator.init();
    gui_iced::run();
}
