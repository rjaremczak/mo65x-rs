use super::{env::Env, registers::Registers, Cpu};
use crate::mos6510::{addrmode::AddrMode, instruction::Instruction, memory::Memory, operation::Operation};

pub type PrepAddrModeFn = fn(&mut Env, &mut Memory, &mut Registers);
pub type ExecInstFn = fn(&mut Cpu, &mut Env, &mut Memory);

#[derive(Copy, Clone)]
pub struct OpCodeEntry {
    pub prep_handler: PrepAddrModeFn,
    pub exec_handler: ExecInstFn,
    pub size: u8,
    pub cycles: u8,
}

impl OpCodeEntry {
    pub fn from(instruction: Instruction, addrmode: AddrMode, cycles: u8) -> Self {
        Self {
            prep_handler: Self::resolve_prep_handler(addrmode),
            exec_handler: Self::resolve_exec_handler(instruction),
            size: addrmode.len() + 1,
            cycles,
        }
    }

    fn resolve_prep_handler(addrmode: AddrMode) -> PrepAddrModeFn {
        match addrmode {
            AddrMode::Implied => Env::prep_implied,
            AddrMode::Relative => Env::prep_branch,
            AddrMode::Immediate => Env::prep_immediate,
            AddrMode::ZeroPage => Env::prep_zero_page,
            AddrMode::ZeroPageX => Env::prep_zero_page_x,
            AddrMode::ZeroPageY => Env::prep_zero_page_y,
            AddrMode::IndexedIndirectX => Env::prep_indexed_indirect_x,
            AddrMode::IndirectIndexedY => Env::prep_indirect_indexed_y,
            AddrMode::Indirect => Env::prep_indirect,
            AddrMode::Absolute => Env::prep_absolute,
            AddrMode::AbsoluteX => Env::prep_absolute_x,
            AddrMode::AbsoluteY => Env::prep_absolute_y,
        }
    }

    fn resolve_exec_handler(instruction: Instruction) -> ExecInstFn {
        match instruction {
            Instruction::Adc => Cpu::exec_adc,
            Instruction::Sbc => Cpu::exec_sbc,
            Instruction::And => Cpu::exec_and,
            Instruction::Ora => Cpu::exec_ora,
            Instruction::Asl => Cpu::exec_asl,
            Instruction::Lsr => Cpu::exec_lsr,
            Instruction::Eor => Cpu::exec_eor,
            Instruction::Rol => Cpu::exec_rol,
            Instruction::Ror => Cpu::exec_ror,
            Instruction::Bit => Cpu::exec_bit,
            Instruction::Cmp => Cpu::exec_cmp,
            Instruction::Cpx => Cpu::exec_cpx,
            Instruction::Cpy => Cpu::exec_cpy,
            Instruction::Inc => Cpu::exec_inc,
            Instruction::Inx => Cpu::exec_inx,
            Instruction::Iny => Cpu::exec_iny,
            Instruction::Dec => Cpu::exec_dec,
            Instruction::Dex => Cpu::exec_dex,
            Instruction::Dey => Cpu::exec_dey,
            Instruction::Bcc => Cpu::exec_bcc,
            Instruction::Bcs => Cpu::exec_bcs,
            Instruction::Beq => Cpu::exec_beq,
            Instruction::Bmi => Cpu::exec_bmi,
            Instruction::Bne => Cpu::exec_bne,
            Instruction::Bpl => Cpu::exec_bpl,
            Instruction::Bvc => Cpu::exec_bvc,
            Instruction::Bvs => Cpu::exec_bvs,
            Instruction::Clc => Cpu::exec_clc,
            Instruction::Cld => Cpu::exec_cld,
            Instruction::Cli => Cpu::exec_cli,
            Instruction::Clv => Cpu::exec_clv,
            Instruction::Sec => Cpu::exec_sec,
            Instruction::Sed => Cpu::exec_sed,
            Instruction::Sei => Cpu::exec_sei,
            Instruction::Jmp => Cpu::exec_jmp,
            Instruction::Jsr => Cpu::exec_jsr,
            Instruction::Brk => Cpu::exec_brk,
            Instruction::Rti => Cpu::exec_rti,
            Instruction::Rts => Cpu::exec_rts,
            Instruction::Lda => Cpu::exec_lda,
            Instruction::Ldx => Cpu::exec_ldx,
            Instruction::Ldy => Cpu::exec_ldy,
            Instruction::Sta => Cpu::exec_sta,
            Instruction::Stx => Cpu::exec_stx,
            Instruction::Sty => Cpu::exec_sty,
            Instruction::Tax => Cpu::exec_tax,
            Instruction::Tay => Cpu::exec_tay,
            Instruction::Tsx => Cpu::exec_tsx,
            Instruction::Txa => Cpu::exec_txa,
            Instruction::Tya => Cpu::exec_tya,
            Instruction::Txs => Cpu::exec_txs,
            Instruction::Pha => Cpu::exec_pha,
            Instruction::Php => Cpu::exec_php,
            Instruction::Pla => Cpu::exec_pla,
            Instruction::Plp => Cpu::exec_plp,
            Instruction::Nop => Cpu::exec_nop,
            Instruction::Kil => Cpu::exec_kil,
        }
    }
}

pub type OpCodeTable = [OpCodeEntry; 256];

pub fn opcode_table() -> OpCodeTable {
    let mut oct: OpCodeTable = [OpCodeEntry::from(Instruction::Kil, AddrMode::Implied, 0); 256];
    for code in u8::MIN..u8::MAX {
        let operation = Operation::get(code);
        if operation.instruction != Instruction::Kil {
            oct[code as usize] = OpCodeEntry::from(operation.instruction, operation.addrmode, operation.cycles);
        }
    }
    oct
}
