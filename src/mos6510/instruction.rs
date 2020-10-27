#[derive(Debug, PartialEq)]
pub struct Instruction<'a> {
    mnemonic: &'a str,
}

impl Instruction<'_> {
    const fn new(mnemonic: &str) -> Instruction {
        Instruction { mnemonic }
    }

    pub fn from(mnemonic: &str) -> Option<&Instruction> {
        INSTRUCTIONS.iter().find(|e| e.mnemonic == mnemonic).map(|e| *e)
    }
}

pub static ADC: Instruction = Instruction::new("ADC");
pub static SBC: Instruction = Instruction::new("SBC");
pub static AND: Instruction = Instruction::new("AND");
pub static ORA: Instruction = Instruction::new("ORA");
pub static ASL: Instruction = Instruction::new("ASL");
pub static LSR: Instruction = Instruction::new("LSR");
pub static EOR: Instruction = Instruction::new("EOR");
pub static ROL: Instruction = Instruction::new("ROL");
pub static ROR: Instruction = Instruction::new("ROR");
pub static BIT: Instruction = Instruction::new("BIT");
pub static CMP: Instruction = Instruction::new("CMP");
pub static CPX: Instruction = Instruction::new("CPX");
pub static CPY: Instruction = Instruction::new("CPY");
pub static INC: Instruction = Instruction::new("INC");
pub static INX: Instruction = Instruction::new("INX");
pub static INY: Instruction = Instruction::new("INY");
pub static DEC: Instruction = Instruction::new("DEC");
pub static DEX: Instruction = Instruction::new("DEX");
pub static DEY: Instruction = Instruction::new("DEY");
pub static BCC: Instruction = Instruction::new("BCC");
pub static BCS: Instruction = Instruction::new("BCS");
pub static BEQ: Instruction = Instruction::new("BEQ");
pub static BMI: Instruction = Instruction::new("BMI");
pub static BNE: Instruction = Instruction::new("BNE");
pub static BPL: Instruction = Instruction::new("BPL");
pub static BVC: Instruction = Instruction::new("BVC");
pub static BVS: Instruction = Instruction::new("BVS");
pub static CLC: Instruction = Instruction::new("CLC");
pub static CLD: Instruction = Instruction::new("CLD");
pub static CLI: Instruction = Instruction::new("CLI");
pub static CLV: Instruction = Instruction::new("CLV");
pub static SEC: Instruction = Instruction::new("SEC");
pub static SED: Instruction = Instruction::new("SED");
pub static SEI: Instruction = Instruction::new("SEI");
pub static JMP: Instruction = Instruction::new("JMP");
pub static JSR: Instruction = Instruction::new("JSR");
pub static BRK: Instruction = Instruction::new("BRK");
pub static RTI: Instruction = Instruction::new("RTI");
pub static RTS: Instruction = Instruction::new("RTS");
pub static LDA: Instruction = Instruction::new("LDA");
pub static LDX: Instruction = Instruction::new("LDX");
pub static LDY: Instruction = Instruction::new("LDY");
pub static STA: Instruction = Instruction::new("STA");
pub static STX: Instruction = Instruction::new("STX");
pub static STY: Instruction = Instruction::new("STY");
pub static TAX: Instruction = Instruction::new("TAX");
pub static TAY: Instruction = Instruction::new("TAY");
pub static TSX: Instruction = Instruction::new("TSX");
pub static TXA: Instruction = Instruction::new("TXA");
pub static TYA: Instruction = Instruction::new("TYA");
pub static TXS: Instruction = Instruction::new("TXS");
pub static PHA: Instruction = Instruction::new("PHA");
pub static PHP: Instruction = Instruction::new("PHP");
pub static PLA: Instruction = Instruction::new("PLA");
pub static PLP: Instruction = Instruction::new("PLP");
pub static NOP: Instruction = Instruction::new("NOP");
pub static KIL: Instruction = Instruction::new("KIL");

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
        assert_eq!(Instruction::from("LDX"), Some(&LDX));
        assert_eq!(Instruction::from("LDA"), Some(&LDA));
    }

    #[test]
    fn find_failed() {
        assert!(matches!(Instruction::from("JUH"), None));
    }

    #[test]
    fn find_mnemonic_by_instruction() {
        assert_eq!(LDA.mnemonic, "LDA");
        assert_eq!(TXA.mnemonic, "TXA");
        assert_eq!(KIL.mnemonic, "KIL");
        assert_eq!(JMP.mnemonic, "JMP");
    }
}
