#[derive(PartialEq)]
pub enum Instruction {
    Adc,
    Sbc,
    And,
    Ora,
    Asl,
    Lsr,
    Eor,
    Rol,
    Ror,
    Bit,
    Cmp,
    Cpx,
    Cpy,
    Inc,
    Inx,
    Iny,
    Dec,
    Dex,
    Dey,
    Bcc,
    Bcs,
    Beq,
    Bmi,
    Bne,
    Bpl,
    Bvc,
    Bvs,
    Clc,
    Cld,
    Cli,
    Clv,
    Sec,
    Sed,
    Sei,
    Jmp,
    Jsr,
    Brk,
    Rti,
    Rts,
    Lda,
    Ldx,
    Ldy,
    Sta,
    Stx,
    Sty,
    Tax,
    Tay,
    Tsx,
    Txa,
    Tya,
    Txs,
    Pha,
    Php,
    Pla,
    Plp,
    Nop,
    Kil,
}

pub struct InstructionDef {
    pub id: Instruction,
    pub mnemonic: &'static str,
}

impl InstructionDef {
    fn new(id: Instruction, mnemonic: &'static str) -> Self {
        Self { id, mnemonic }
    }

    pub fn by_mnemonic(mnemonic: &str) -> Option<&InstructionDef> {
        let m = &mnemonic.to_uppercase();
        INSTRUCTIONS.iter().find(|e| e.mnemonic == m)
    }

    pub fn find(id: Instruction) -> Option<&'static InstructionDef> {
        INSTRUCTIONS.iter().find(|e| e.id == id)
    }
}

use Instruction::*;

pub static INSTRUCTIONS: [InstructionDef; 57] = [
    InstructionDef::new(Kil, "KIL"),
    InstructionDef::new(Adc, "ADC"),
    InstructionDef::new(Sbc, "SBC"),
    InstructionDef::new(And, "AND"),
    InstructionDef::new(Ora, "ORA"),
    InstructionDef::new(Asl, "ASL"),
    InstructionDef::new(Lsr, "LSR"),
    InstructionDef::new(Eor, "EOR"),
    InstructionDef::new(Rol, "ROL"),
    InstructionDef::new(Ror, "ROR"),
    InstructionDef::new(Bit, "BIT"),
    InstructionDef::new(Cmp, "CMP"),
    InstructionDef::new(Cpx, "CPX"),
    InstructionDef::new(Cpy, "CPY"),
    InstructionDef::new(Inc, "INC"),
    InstructionDef::new(Inx, "INX"),
    InstructionDef::new(Iny, "INY"),
    InstructionDef::new(Dec, "DEC"),
    InstructionDef::new(Dex, "DEX"),
    InstructionDef::new(Dey, "DEY"),
    InstructionDef::new(Bcc, "BCC"),
    InstructionDef::new(Bcs, "BCS"),
    InstructionDef::new(Beq, "BEQ"),
    InstructionDef::new(Bmi, "BMI"),
    InstructionDef::new(Bne, "BNE"),
    InstructionDef::new(Bpl, "BPL"),
    InstructionDef::new(Bvc, "BVC"),
    InstructionDef::new(Bvs, "BVS"),
    InstructionDef::new(Clc, "CLC"),
    InstructionDef::new(Cld, "CLD"),
    InstructionDef::new(Cli, "CLI"),
    InstructionDef::new(Clv, "CLV"),
    InstructionDef::new(Sec, "SEC"),
    InstructionDef::new(Sed, "SED"),
    InstructionDef::new(Sei, "SEI"),
    InstructionDef::new(Jmp, "JMP"),
    InstructionDef::new(Jsr, "JSR"),
    InstructionDef::new(Brk, "BRK"),
    InstructionDef::new(Rti, "RTI"),
    InstructionDef::new(Rts, "RTS"),
    InstructionDef::new(Lda, "LDA"),
    InstructionDef::new(Ldx, "LDX"),
    InstructionDef::new(Ldy, "LDY"),
    InstructionDef::new(Sta, "STA"),
    InstructionDef::new(Stx, "STX"),
    InstructionDef::new(Sty, "STY"),
    InstructionDef::new(Tax, "TAX"),
    InstructionDef::new(Tay, "TAY"),
    InstructionDef::new(Tsx, "TSX"),
    InstructionDef::new(Txa, "TXA"),
    InstructionDef::new(Tya, "TYA"),
    InstructionDef::new(Txs, "TXS"),
    InstructionDef::new(Pha, "PHA"),
    InstructionDef::new(Php, "PHP"),
    InstructionDef::new(Pla, "PLA"),
    InstructionDef::new(Plp, "PLP"),
    InstructionDef::new(Nop, "NOP"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_ok() {
        assert!(InstructionDef::by_mnemonic("LDX").unwrap().id == Ldx);
        assert!(InstructionDef::by_mnemonic("LDA").unwrap().id == Lda);
    }

    #[test]
    fn find_failed() {
        assert!(matches!(InstructionDef::by_mnemonic("JUH"), None));
    }

    #[test]
    fn find_mnemonic_by_instruction() {
        assert_eq!(InstructionDef::find(Lda).unwrap().mnemonic, "LDA");
        assert_eq!(InstructionDef::find(Txa).unwrap().mnemonic, "TXA");
        assert_eq!(InstructionDef::find(Kil).unwrap().mnemonic, "KIL");
        assert_eq!(InstructionDef::find(Jmp).unwrap().mnemonic, "JMP");
    }
}
