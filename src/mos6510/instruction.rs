pub fn parse_instruction(mnemonic: &str) -> Option<Instruction> {
    MNEMONICS.iter().find(|e| e.mnemonic == mnemonic).map(|e| e.instruction)
}

pub fn find_mnemonic(instruction: Instruction) -> Option<&'static str> {
    MNEMONICS.iter().find(|e| e.instruction == instruction).map(|e| e.mnemonic)
}

#[derive(Clone, Copy, PartialEq)]
pub enum Instruction {
    KIL, // every unsupported instruction that needs core to halt!
    ADC,
    SBC,
    AND,
    ORA,
    ASL,
    LSR,
    EOR,
    ROL,
    ROR,
    BIT,
    CMP,
    CPX,
    CPY,
    INC,
    INX,
    INY,
    DEC,
    DEX,
    DEY,
    BCC,
    BCS,
    BEQ,
    BMI,
    BNE,
    BPL,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    SEC,
    SED,
    SEI,
    JMP,
    JSR,
    BRK,
    RTI,
    RTS,
    LDA,
    LDX,
    LDY,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TYA,
    TXS,
    PHA,
    PHP,
    PLA,
    PLP,
    NOP,
}

use Instruction::*;

pub struct Mnemonic<'a> {
    instruction: Instruction,
    mnemonic: &'a str,
}

const fn mn(instruction: Instruction, mnemonic: &str) -> Mnemonic {
    Mnemonic { instruction, mnemonic }
}

pub static MNEMONICS: [Mnemonic; 57] = [
    mn(ADC, "ADC"),
    mn(SBC, "SBC"),
    mn(AND, "AND"),
    mn(ORA, "ORA"),
    mn(ASL, "ASL"),
    mn(LSR, "LSR"),
    mn(EOR, "EOR"),
    mn(ROL, "ROL"),
    mn(ROR, "ROR"),
    mn(BIT, "BIT"),
    mn(CMP, "CMP"),
    mn(CPX, "CPX"),
    mn(CPY, "CPY"),
    mn(INC, "INC"),
    mn(INX, "INX"),
    mn(INY, "INY"),
    mn(DEC, "DEC"),
    mn(DEX, "DEX"),
    mn(DEY, "DEY"),
    mn(BCC, "BCC"),
    mn(BCS, "BCS"),
    mn(BEQ, "BEQ"),
    mn(BMI, "BMI"),
    mn(BNE, "BNE"),
    mn(BPL, "BPL"),
    mn(BVC, "BVC"),
    mn(BVS, "BVS"),
    mn(CLC, "CLC"),
    mn(CLD, "CLD"),
    mn(CLI, "CLI"),
    mn(CLV, "CLV"),
    mn(SEC, "SEC"),
    mn(SED, "SED"),
    mn(SEI, "SEI"),
    mn(JMP, "JMP"),
    mn(JSR, "JSR"),
    mn(BRK, "BRK"),
    mn(RTI, "RTI"),
    mn(RTS, "RTS"),
    mn(LDA, "LDA"),
    mn(LDX, "LDX"),
    mn(LDY, "LDY"),
    mn(STA, "STA"),
    mn(STX, "STX"),
    mn(STY, "STY"),
    mn(TAX, "TAX"),
    mn(TAY, "TAY"),
    mn(TSX, "TSX"),
    mn(TXA, "TXA"),
    mn(TYA, "TYA"),
    mn(TXS, "TXS"),
    mn(PHA, "PHA"),
    mn(PHP, "PHP"),
    mn(PLA, "PLA"),
    mn(PLP, "PLP"),
    mn(NOP, "NOP"),
    mn(KIL, "KIL"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_ok() {
        assert!(matches!(parse_instruction("LDX"), Some(LDX)));
        assert!(matches!(parse_instruction("LDA").unwrap(), LDA));
    }

    #[test]
    fn find_failed() {
        assert!(matches!(parse_instruction("JUH"), None));
    }

    #[test]
    fn find_mnemonic_by_instruction() {
        assert!(matches!(find_mnemonic(LDA).unwrap(), "LDA"));
        assert!(matches!(find_mnemonic(TXA).unwrap(), "TXA"));
        assert!(matches!(find_mnemonic(KIL).unwrap(), "KIL"));
    }
}
