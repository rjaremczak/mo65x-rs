mod mos6510;

//mod gui_iced;
//mod gui_druid;

fn main() {
    //gui_iced::run();
    //let _ = gui_druid::run();
    //let mut memory: mos6510::Memory;
    // println!("opcodes {}", mos6510::opcode::OPCODES[0].size);
    let mut assembler = mos6510::assembler::Assembler::new(0);
    let mut memory = mos6510::memory::Memory::new();
    let mut cpu = mos6510::cpu::Cpu::new();
    cpu.reset(&memory);
}
