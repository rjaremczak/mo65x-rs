mod gui_iced;
mod mos6510;

fn main() {
    let mut assembler = mos6510::assembler::Assembler::new(0);
    let mut memory = mos6510::memory::Memory::new();
    let mut cpu = mos6510::cpu::Cpu::new();
    cpu.reset(&memory);
    cpu.exec_inst(&mut memory);
    gui_iced::run();
}
