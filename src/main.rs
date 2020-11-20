mod emulator;
mod mos6510;

use emulator::Emulator;
use std::env;

fn main() {
    println!("cwd: {:#?}", env::current_dir().unwrap());
    let mut emulator = Emulator::new();
    let mut assembler = mos6510::assembler::Assembler::new();
    emulator.init();
}
