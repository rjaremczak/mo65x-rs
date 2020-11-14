use super::{env::Env, registers::Registers, Cpu};
use crate::mos6510::{addrmode::AddrMode, instruction::Instruction, memory::Memory, opcode::OPCODES};

pub type PrepAddrModeFn = fn(&mut Env, &mut Memory, &mut Registers);
pub type ExecInstFn<'a> = fn(&mut Cpu<'a>, &mut Env, &mut Memory, &mut u8);

#[derive(Copy, Clone)]
pub struct OpCodeEntry<'a> {
    pub prep_handler: PrepAddrModeFn,
    pub exec_handler: ExecInstFn<'a>,
    pub size: u8,
}

impl<'a> OpCodeEntry<'a> {
    pub fn from(instruction: Instruction, addrmode: AddrMode) -> Self {
        Self {
            prep_handler: Self::resolve_prep_handler(addrmode),
            exec_handler: Self::resolve_exec_handler(instruction),
            size: addrmode.def().op_size + 1,
        }
    }

    fn resolve_prep_handler(addrmode: AddrMode) -> PrepAddrModeFn {
        Env::prep_implied
    }

    fn resolve_exec_handler(instruction: Instruction) -> ExecInstFn<'a> {
        Cpu::exec_kil
    }
}

pub type OpCodeTable<'a> = [OpCodeEntry<'a>; 256];

pub fn generate_opcode_table<'a>() -> OpCodeTable<'a> {
    let mut oct: OpCodeTable<'a> = [OpCodeEntry::from(Instruction::Kil, AddrMode::Implied); 256];
    for oc in &OPCODES {
        if oc.instruction != Instruction::Kil {
            oct[oc.code as usize] = OpCodeEntry::from(oc.instruction, oc.addrmode);
        }
    }
    oct
}
