use super::cpu::{Cpu, InstructionHandler};

pub struct Instruction<'a> {
    pub handler: InstructionHandler,
    pub mnemonic: &'a str,
}

impl<'a> PartialEq for Instruction<'a> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

pub fn find_instruction(mnemonic: &str) -> Option<&Instruction> {
    let m = &mnemonic.to_uppercase();
    INSTRUCTIONS.iter().find(|e| e.mnemonic == m).map(|e| *e)
}

pub static ADC: Instruction = Instruction {
    handler: Cpu::exec_adc,
    mnemonic: "ADC",
};

pub static SBC: Instruction = Instruction {
    handler: Cpu::exec_sbc,
    mnemonic: "SBC",
};

pub static AND: Instruction = Instruction {
    handler: Cpu::exec_and,
    mnemonic: "AND",
};

pub static ORA: Instruction = Instruction {
    handler: Cpu::exec_ora,
    mnemonic: "ORA",
};

pub static ASL: Instruction = Instruction {
    handler: Cpu::exec_asl,
    mnemonic: "ASL",
};

pub static LSR: Instruction = Instruction {
    handler: Cpu::exec_lsr,
    mnemonic: "LSR",
};

pub static EOR: Instruction = Instruction {
    handler: Cpu::exec_eor,
    mnemonic: "EOR",
};

pub static ROL: Instruction = Instruction {
    handler: Cpu::exec_rol,
    mnemonic: "ROL",
};

pub static ROR: Instruction = Instruction {
    handler: Cpu::exec_ror,
    mnemonic: "ROR",
};

pub static BIT: Instruction = Instruction {
    handler: Cpu::exec_bit,
    mnemonic: "BIT",
};

pub static CMP: Instruction = Instruction {
    handler: Cpu::exec_cmp,
    mnemonic: "CMP",
};

pub static CPX: Instruction = Instruction {
    handler: Cpu::exec_cpx,
    mnemonic: "CPX",
};

pub static CPY: Instruction = Instruction {
    handler: Cpu::exec_cpy,
    mnemonic: "CPY",
};

pub static INC: Instruction = Instruction {
    handler: Cpu::exec_inc,
    mnemonic: "INC",
};

pub static INX: Instruction = Instruction {
    handler: Cpu::exec_inx,
    mnemonic: "INX",
};

pub static INY: Instruction = Instruction {
    handler: Cpu::exec_iny,
    mnemonic: "INY",
};

pub static DEC: Instruction = Instruction {
    handler: Cpu::exec_dec,
    mnemonic: "DEC",
};

pub static DEX: Instruction = Instruction {
    handler: Cpu::exec_dex,
    mnemonic: "DEX",
};

pub static DEY: Instruction = Instruction {
    handler: Cpu::exec_dey,
    mnemonic: "DEY",
};

pub static BCC: Instruction = Instruction {
    handler: Cpu::exec_bcc,
    mnemonic: "BCC",
};

pub static BCS: Instruction = Instruction {
    handler: Cpu::exec_bcs,
    mnemonic: "BCS",
};

pub static BEQ: Instruction = Instruction {
    handler: Cpu::exec_beq,
    mnemonic: "BEQ",
};

pub static BMI: Instruction = Instruction {
    handler: Cpu::exec_bmi,
    mnemonic: "BMI",
};

pub static BNE: Instruction = Instruction {
    handler: Cpu::exec_bne,
    mnemonic: "BNE",
};

pub static BPL: Instruction = Instruction {
    handler: Cpu::exec_bpl,
    mnemonic: "BPL",
};

pub static BVC: Instruction = Instruction {
    handler: Cpu::exec_bvc,
    mnemonic: "BVC",
};

pub static BVS: Instruction = Instruction {
    handler: Cpu::exec_bvs,
    mnemonic: "BVS",
};

pub static CLC: Instruction = Instruction {
    handler: Cpu::exec_clc,
    mnemonic: "CLC",
};

pub static CLD: Instruction = Instruction {
    handler: Cpu::exec_cld,
    mnemonic: "CLD",
};

pub static CLI: Instruction = Instruction {
    handler: Cpu::exec_cli,
    mnemonic: "CLI",
};

pub static CLV: Instruction = Instruction {
    handler: Cpu::exec_clv,
    mnemonic: "CLV",
};

pub static SEC: Instruction = Instruction {
    handler: Cpu::exec_sec,
    mnemonic: "SEC",
};

pub static SED: Instruction = Instruction {
    handler: Cpu::exec_sed,
    mnemonic: "SED",
};

pub static SEI: Instruction = Instruction {
    handler: Cpu::exec_sei,
    mnemonic: "SEI",
};

pub static JMP: Instruction = Instruction {
    handler: Cpu::exec_jmp,
    mnemonic: "JMP",
};

pub static JSR: Instruction = Instruction {
    handler: Cpu::exec_jsr,
    mnemonic: "JSR",
};

pub static BRK: Instruction = Instruction {
    handler: Cpu::exec_brk,
    mnemonic: "BRK",
};

pub static RTI: Instruction = Instruction {
    handler: Cpu::exec_rti,
    mnemonic: "RTI",
};

pub static RTS: Instruction = Instruction {
    handler: Cpu::exec_rts,
    mnemonic: "RTS",
};

pub static LDA: Instruction = Instruction {
    handler: Cpu::exec_lda,
    mnemonic: "LDA",
};

pub static LDX: Instruction = Instruction {
    handler: Cpu::exec_ldx,
    mnemonic: "LDX",
};

pub static LDY: Instruction = Instruction {
    handler: Cpu::exec_ldy,
    mnemonic: "LDY",
};

pub static STA: Instruction = Instruction {
    handler: Cpu::exec_sta,
    mnemonic: "STA",
};

pub static STX: Instruction = Instruction {
    handler: Cpu::exec_stx,
    mnemonic: "STX",
};

pub static STY: Instruction = Instruction {
    handler: Cpu::exec_sty,
    mnemonic: "STY",
};

pub static TAX: Instruction = Instruction {
    handler: Cpu::exec_tax,
    mnemonic: "TAX",
};

pub static TAY: Instruction = Instruction {
    handler: Cpu::exec_tay,
    mnemonic: "TAY",
};

pub static TSX: Instruction = Instruction {
    handler: Cpu::exec_tsx,
    mnemonic: "TSX",
};

pub static TXA: Instruction = Instruction {
    handler: Cpu::exec_txa,
    mnemonic: "TXA",
};

pub static TYA: Instruction = Instruction {
    handler: Cpu::exec_tya,
    mnemonic: "TYA",
};

pub static TXS: Instruction = Instruction {
    handler: Cpu::exec_txs,
    mnemonic: "TXS",
};

pub static PHA: Instruction = Instruction {
    handler: Cpu::exec_pha,
    mnemonic: "PHA",
};

pub static PHP: Instruction = Instruction {
    handler: Cpu::exec_php,
    mnemonic: "PHP",
};

pub static PLA: Instruction = Instruction {
    handler: Cpu::exec_pla,
    mnemonic: "PLA",
};

pub static PLP: Instruction = Instruction {
    handler: Cpu::exec_plp,
    mnemonic: "PLP",
};

pub static NOP: Instruction = Instruction {
    handler: Cpu::exec_nop,
    mnemonic: "NOP",
};

pub static KIL: Instruction = Instruction {
    handler: Cpu::exec_kil,
    mnemonic: "KIL",
};

pub static INSTRUCTIONS: [&Instruction; 57] = [
    &KIL, &ADC, &SBC, &AND, &ORA, &ASL, &LSR, &EOR, &ROL, &ROR, &BIT, &CMP, &CPX, &CPY, &INC, &INX, &INY, &DEC, &DEX, &DEY, &BCC, &BCS,
    &BEQ, &BMI, &BNE, &BPL, &BVC, &BVS, &CLC, &CLD, &CLI, &CLV, &SEC, &SED, &SEI, &JMP, &JSR, &BRK, &RTI, &RTS, &LDA, &LDX, &LDY, &STA,
    &STX, &STY, &TAX, &TAY, &TSX, &TXA, &TYA, &TXS, &PHA, &PHP, &PLA, &PLP, &NOP,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_ok() {
        assert!(find_instruction("LDX").unwrap() == &LDX);
        assert!(find_instruction("LDA").unwrap() == &LDA);
    }

    #[test]
    fn find_failed() {
        assert!(matches!(find_instruction("JUH"), None));
    }

    #[test]
    fn find_mnemonic_by_instruction() {
        assert_eq!(LDA.mnemonic, "LDA");
        assert_eq!(TXA.mnemonic, "TXA");
        assert_eq!(KIL.mnemonic, "KIL");
        assert_eq!(JMP.mnemonic, "JMP");
    }
}
