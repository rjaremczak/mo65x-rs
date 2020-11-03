use self::registers::Flags;

mod registers;

struct OperandPtr<'a> {
    lo: Option<&'a mut u8>,
    hi: Option<&'a mut u8>,
}

pub struct Cpu {
    // state: CpuState,
    // operand: OperandPtr,
    effective_address: u16,
    reg_a: u8,
    reg_x: u8,
    reg_y: u8,
    reg_sp: u8,
    reg_pc: u16,
    flags: Flags,
}
