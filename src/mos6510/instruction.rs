use std::collections::BTreeMap;
use Instruction::*;

use super::error::AppError;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
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

impl Instruction {
    pub fn mnemonic(&self) -> &'static str {
        MNEMONICS.get(self).unwrap()
    }

    pub fn parse(mnemonic: &str) -> Result<Instruction, AppError> {
        let m = &mnemonic.to_uppercase();
        MNEMONICS
            .iter()
            .find(|kv| kv.1 == m)
            .map(|kv| *kv.0)
            .ok_or(AppError::InvalidMnemonic(String::from(mnemonic)))
    }
}

lazy_static! {
    static ref MNEMONICS: BTreeMap<Instruction, &'static str> = {
        let mut m = BTreeMap::new();
        m.insert(Kil, "KIL");
        m.insert(Adc, "ADC");
        m.insert(Sbc, "SBC");
        m.insert(And, "AND");
        m.insert(Ora, "ORA");
        m.insert(Asl, "ASL");
        m.insert(Lsr, "LSR");
        m.insert(Eor, "EOR");
        m.insert(Rol, "ROL");
        m.insert(Ror, "ROR");
        m.insert(Bit, "BIT");
        m.insert(Cmp, "CMP");
        m.insert(Cpx, "CPX");
        m.insert(Cpy, "CPY");
        m.insert(Inc, "INC");
        m.insert(Inx, "INX");
        m.insert(Iny, "INY");
        m.insert(Dec, "DEC");
        m.insert(Dex, "DEX");
        m.insert(Dey, "DEY");
        m.insert(Bcc, "BCC");
        m.insert(Bcs, "BCS");
        m.insert(Beq, "BEQ");
        m.insert(Bmi, "BMI");
        m.insert(Bne, "BNE");
        m.insert(Bpl, "BPL");
        m.insert(Bvc, "BVC");
        m.insert(Bvs, "BVS");
        m.insert(Clc, "CLC");
        m.insert(Cld, "CLD");
        m.insert(Cli, "CLI");
        m.insert(Clv, "CLV");
        m.insert(Sec, "SEC");
        m.insert(Sed, "SED");
        m.insert(Sei, "SEI");
        m.insert(Jmp, "JMP");
        m.insert(Jsr, "JSR");
        m.insert(Brk, "BRK");
        m.insert(Rti, "RTI");
        m.insert(Rts, "RTS");
        m.insert(Lda, "LDA");
        m.insert(Ldx, "LDX");
        m.insert(Ldy, "LDY");
        m.insert(Sta, "STA");
        m.insert(Stx, "STX");
        m.insert(Sty, "STY");
        m.insert(Tax, "TAX");
        m.insert(Tay, "TAY");
        m.insert(Tsx, "TSX");
        m.insert(Txa, "TXA");
        m.insert(Tya, "TYA");
        m.insert(Txs, "TXS");
        m.insert(Pha, "PHA");
        m.insert(Php, "PHP");
        m.insert(Pla, "PLA");
        m.insert(Plp, "PLP");
        m.insert(Nop, "NOP");
        m
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_mnemonic_ok() {
        assert!(Instruction::parse("LDX").unwrap() == Ldx);
        assert!(Instruction::parse("LDA").unwrap() == Lda);
    }

    #[test]
    fn find_mnemonic_failed() {
        assert!(matches!(Instruction::parse("JUH"), None));
    }

    #[test]
    fn get_mnemonic() {
        assert_eq!(Lda.mnemonic(), "LDA");
        assert_eq!(Txa.mnemonic(), "TXA");
        assert_eq!(Kil.mnemonic(), "KIL");
        assert_eq!(Jmp.mnemonic(), "JMP");
    }
}
