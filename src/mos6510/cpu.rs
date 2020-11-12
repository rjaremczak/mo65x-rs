pub mod exec_env;

mod flags;
mod registers;

use self::{exec_env::ExecEnv, flags::Flags, registers::Registers};
use super::{memory::Memory, opcode::OPCODES};

pub type InstructionHandler = fn(&mut Cpu, &mut ExecEnv, &mut u8);

type PrepAddrModeFn<'a> = fn(&mut ExecEnv, &'a mut Memory, &'a mut Registers) -> &'a mut u8;
type ExecInstFn<'a> = fn(&mut Cpu<'a>, &mut ExecEnv, &mut Memory, &mut u8);

struct DecodeTableEntry<'a> {
    pub prep_handler: PrepAddrModeFn<'a>,
    pub exec_handler: ExecInstFn<'a>,
}

pub struct Cpu<'a> {
    regs: Registers,
    flags: Flags,
    decode_table: [DecodeTableEntry<'a>; 1],
}

impl<'a> Cpu<'a> {
    pub fn new(pc: u16) -> Self {
        Self {
            regs: Registers::new(pc, 0xfd),
            flags: Flags::new(),
            decode_table: [DecodeTableEntry {
                prep_handler: ExecEnv::prep_implied,
                exec_handler: Cpu::exec_inc,
            }],
        }
    }

    pub fn exec_instruction(&mut self, memory: &mut Memory) -> u8 {
        let opcode = &OPCODES[memory[self.regs.pc] as usize];
        let mut env = ExecEnv::new(self.regs.pc, opcode.cycles);
        // let outref = (opcode.addrmode.handler)(&mut env, memory, &mut self.regs);
        // (opcode.instruction.handler)(self, &mut env, outref);
        self.regs.pc = self.regs.pc.wrapping_add(opcode.size as u16);
        0
    }

    pub fn exec_adc(&mut self, env: &mut ExecEnv, _: &mut Memory) {
        let mut result = self.regs.a as u16 + env.arg + self.flags.c as u16;
        if self.flags.d {
            self.flags.c = decimal_correction(&mut result);
            self.flags.compute_nz(result);
        } else {
            self.flags.compute_nzc(result);
        }
        self.flags.compute_v(self.regs.a as u16, env.arg, result);
        self.regs.a = result as u8;
        env.add_cycle_when_page_crossed();
    }

    pub fn exec_sbc(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_and(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_ora(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_asl(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_lsr(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_eor(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_rol(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_ror(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_bit(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_cmp(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_cpx(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_cpy(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}

    pub fn exec_inc(&mut self, env: &mut ExecEnv, memory: &mut Memory, outref: &mut u8) {
        let result = *outref as u16 + 1;
        *outref = result as u8;
        self.flags.compute_nz(result);
    }

    pub fn exec_inx(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_iny(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_dec(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_dex(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_dey(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_bcc(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_bcs(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_beq(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_bmi(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_bne(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_bpl(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_bvc(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_bvs(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_clc(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_cld(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_cli(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_clv(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_sec(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_sed(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_sei(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_jmp(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_jsr(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_brk(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_rti(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_rts(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_lda(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_ldx(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_ldy(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_sta(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_stx(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_sty(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_tax(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_tay(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_tsx(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_txa(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_tya(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_txs(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_pla(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_plp(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_pha(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_php(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_nop(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
    pub fn exec_kil(&mut self, env: &mut ExecEnv, memory: &mut Memory) {}
}

fn decimal_correction(result: &mut u16) -> bool {
    if (*result & 0x0f) > 0x09 {
        *result += 0x06;
    }
    if (*result & 0xf0) > 0x90 {
        *result += 0x60;
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use crate::mos6510::memory::RESET_VECTOR;

    use super::*;

    #[test]
    fn test_init() {
        let mut memory = Memory::new();
        let pc = memory.word(RESET_VECTOR);
        let mut cpu = Cpu::new(pc);
        assert_eq!(cpu.regs.pc, pc);
    }
}
