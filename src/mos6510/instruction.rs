pub fn parse_instruction(mnemonic: &str) -> Option<Instruction> {
    MNEMONICS.iter().find(|e| e.1 == mnemonic).map_or(None, |e| Some(e.0))
}

pub fn find_mnemonic(instruction: Instruction) -> Option<&'static str> {
    MNEMONICS.iter().find(|e| e.0 == instruction).map_or(None, |e| Some(e.1))
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

pub static MNEMONICS: [(Instruction, &str); 57] = [
    (ADC, "ADC"),
    (SBC, "SBC"),
    (AND, "AND"),
    (ORA, "ORA"),
    (ASL, "ASL"),
    (LSR, "LSR"),
    (EOR, "EOR"),
    (ROL, "ROL"),
    (ROR, "ROR"),
    (BIT, "BIT"),
    (CMP, "CMP"),
    (CPX, "CPX"),
    (CPY, "CPY"),
    (INC, "INC"),
    (INX, "INX"),
    (INY, "INY"),
    (DEC, "DEC"),
    (DEX, "DEX"),
    (DEY, "DEY"),
    (BCC, "BCC"),
    (BCS, "BCS"),
    (BEQ, "BEQ"),
    (BMI, "BMI"),
    (BNE, "BNE"),
    (BPL, "BPL"),
    (BVC, "BVC"),
    (BVS, "BVS"),
    (CLC, "CLC"),
    (CLD, "CLD"),
    (CLI, "CLI"),
    (CLV, "CLV"),
    (SEC, "SEC"),
    (SED, "SED"),
    (SEI, "SEI"),
    (JMP, "JMP"),
    (JSR, "JSR"),
    (BRK, "BRK"),
    (RTI, "RTI"),
    (RTS, "RTS"),
    (LDA, "LDA"),
    (LDX, "LDX"),
    (LDY, "LDY"),
    (STA, "STA"),
    (STX, "STX"),
    (STY, "STY"),
    (TAX, "TAX"),
    (TAY, "TAY"),
    (TSX, "TSX"),
    (TXA, "TXA"),
    (TYA, "TYA"),
    (TXS, "TXS"),
    (PHA, "PHA"),
    (PHP, "PHP"),
    (PLA, "PLA"),
    (PLP, "PLP"),
    (NOP, "NOP"),
    (KIL, "KIL"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_instruction_by_mnemonic() {
        assert!(matches!(parse_instruction("LDA").unwrap(), LDA));
        assert!(matches!(parse_instruction("JUH"), None));
    }

    #[test]
    fn find_mnemonic_by_instruction() {
        assert!(matches!(find_mnemonic(TXA).unwrap(), "TXA"));
        assert!(matches!(find_mnemonic(KIL).unwrap(), "KIL"));
    }
}
