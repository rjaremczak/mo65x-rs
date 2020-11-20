use super::*;

fn assert_next(asm: &mut Assembler, line: &str, expected: &[u8]) {
    let r = asm.process_line(line);
    assert!(matches!(r, AsmError::Ok), "line \"{}\" : {:?}", line, r);
    assert!(asm.code.len() >= expected.len(), "line \"{}\" : code too short", line);
    let generated = &asm.code[(asm.code.len() - expected.len())..];
    assert_eq!(generated, expected, "generated code {:?} differs from {:?}", generated, expected);
}

fn assert_asm<'a>(line: &str, code: &[u8]) -> Assembler {
    let mut asm = Assembler::new();
    asm.set_generate_code(true);
    assert_next(&mut asm, line, code);
    asm
}

#[test]
fn init() {
    let asm = Assembler::new();
    assert_eq!(asm.generate_code, false);
    assert_eq!(asm.location_counter, 0);
    assert_eq!(asm.operand_parser.symbols().count(), 0);
}

#[test]
fn test_list_separator() {
    let asm = Assembler::new();
    let sl: Vec<&str> = asm.op_list_separator.split("20 30 40").collect();
    assert_eq!(sl.as_slice(), &["20", "30", "40"]);
    let sl: Vec<&str> = asm.op_list_separator.split("18").collect();
    assert_eq!(sl.as_slice(), &["18"]);
    let sl: Vec<&str> = asm.op_list_separator.split("120, 0x30 40, 023").collect();
    assert_eq!(sl.as_slice(), &["120", "0x30", "40", "023"]);
}

#[test]
fn empty_line() {
    assert_asm("", &[]);
}

#[test]
fn implied_mode() {
    assert_asm("SEI", &[0x78]);
    assert_asm("ASL", &[0x0a]);
}

#[test]
fn immediate_mode() {
    assert_asm("LDA #%00110101", &[0xa9, 0b00110101]);
    assert_asm("LDX #123", &[0xa2, 123]);
    assert_asm("LDY #255", &[0xa0, 0xff]);
}

#[test]
fn zero_page_mode() {
    assert_asm("LDY $8f", &[0xa4, 0x8f]);
}

#[test]
fn zero_page_x_mode() {
    assert_asm("LDA $a0,X", &[0xb5, 0xa0]);
}

#[test]
fn zero_page_y_mode() {
    assert_asm("STX $7a,Y", &[0x96, 0x7a]);
}

#[test]
fn absolute_mode() {
    let mut asm = assert_asm("ROR $3400", &[0x6e, 0x00, 0x34]);
    assert_next(&mut asm, "jmp $2000", &[0x4c, 0x00, 0x20]);
    asm.operand_parser.define_symbol("c", 0xfab0);
    assert_next(&mut asm, "jmp c", &[0x4c, 0xb0, 0xfa]);
}

#[test]
fn absolute_mode_x() {
    assert_asm("LSR $35f0,X", &[0x5e, 0xf0, 0x35]);
}

#[test]
fn absolute_mode_y() {
    assert_asm("EOR $f7a0,Y", &[0x59, 0xa0, 0xf7]);
}

#[test]
fn indirect_mode() {
    assert_asm("JMP ($ffa0)", &[0x6c, 0xa0, 0xff]);
}

#[test]
fn indirect_mode_x() {
    assert_asm("LDA ($8c,X)", &[0xa1, 0x8c]);
}

#[test]
fn indirect_indexed_y() {
    assert_asm("ORA ($a7),Y", &[0x11, 0xa7]);
}

#[test]
fn relative_mode() {
    assert_asm("BCC -1", &[0x90, u8::from_ne_bytes((-1 as i8).to_ne_bytes())]);
    assert_asm("BVS +8", &[0x70, 8]);
}

#[test]
fn set_location_counter() {
    let mut asm = assert_asm("  .ORG $3000 ;origin", &[]);
    assert_eq!(asm.origin.unwrap(), 0x3000);
    assert_eq!(asm.location_counter, 0x3000);
    assert_next(&mut asm, "  .ORG $4000 ;origin", &[]);
    assert_eq!(asm.location_counter, 0x4000);
    assert_eq!(asm.code.len(), 0x1000);
    assert_next(&mut asm, "  *= $5000 ;origin", &[]);
    assert_eq!(asm.location_counter, 0x5000);
    assert_eq!(asm.code.len(), 0x2000);
}

#[test]
fn test_comments() {
    assert_asm("lda ($8c,X)  ;komentarz", &[0xa1, 0x8c]);
    assert_asm("  ;  komentarz", &[]);
    assert_asm("label: ;komentarz numer 2", &[]);
    assert_asm("LSR $35f0,X ;comment", &[0x5e, 0xf0, 0x35]);
}

#[test]
fn test_label() {
    let mut asm = assert_asm("Label_001:", &[]);
    assert_next(&mut asm, "LDA ($8c,X)", &[0xa1, 0x8c]);
    assert_next(&mut asm, "Label_002:", &[]);
    assert_eq!(asm.operand_parser.get_symbol("Label_001").unwrap(), 0);
    assert_eq!(asm.operand_parser.get_symbol("Label_002").unwrap(), 2);
}

#[test]
fn test_symbols() {
    let mut asm = Assembler::new();
    asm.set_location_counter(1000);
    asm.set_generate_code(true);
    asm.operand_parser.define_symbol("dziabaDucha", 0xaf02);
    assert_next(&mut asm, "TestLabel_01:  SEI   ; disable interrupts ", &[0x78]);
    assert_next(&mut asm, "c:lda dziabaDucha", &[0xad, 0x02, 0xaf]);
    assert_eq!(asm.operand_parser.get_symbol("TestLabel_01").unwrap(), 1000);
    assert_eq!(asm.operand_parser.get_symbol("TestLabel_02"), None);
    assert_eq!(asm.code.len(), 4);
    assert_eq!(asm.location_counter, 1004);
}

#[test]
fn emit_bytes() {
    assert_asm(".BYTE 20", &[20]);
    assert_asm(".BYTE $20 45 $4a", &[0x20, 45, 0x4a]);
    assert_asm(".BYTE $20, $3f,$4a ,$23 , 123", &[0x20, 0x3f, 0x4a, 0x23, 123]);
    assert_asm(
        "dcb 0,0,0,0,0,0,0,0,0,$b,$b,$c,$f,$f,$f,$f",
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0xb, 0xb, 0xc, 0xf, 0xf, 0xf, 0xf],
    );
}

#[test]
fn emit_words() {
    assert_asm(".word $20ff $23af $fab0 ;komm", &[0xff, 0x20, 0xaf, 0x23, 0xb0, 0xfa]);
    assert_asm(".word $3000 $15ad 1024", &[0x00, 0x30, 0xad, 0x15, 0x00, 0x04]);
}

#[test]
fn lo_byte_modifier() {
    let mut asm = assert_asm("LDA #<$1afc", &[0xa9, 0xfc]);
    asm.operand_parser.define_symbol("label", 0x2afe);
    assert_next(&mut asm, "LDA #<label", &[0xa9, 0xfe]);
    assert_next(&mut asm, "dcb <label, 2", &[0xfe, 2]);
}

#[test]
fn hi_byte_modifier() {
    let mut asm = assert_asm("LDA #>$1afc", &[0xa9, 0x1a]);
    asm.operand_parser.define_symbol("label", 0x3afe);
    assert_next(&mut asm, "LDA #>label", &[0xa9, 0x3a]);
    assert_next(&mut asm, "dcb >label, 2", &[0x3a, 2]);
    asm.operand_parser.define_symbol("a", 0xfa20);
    asm.operand_parser.define_symbol("b", 0x10a0);
    assert_next(&mut asm, "dcb >a, <b", &[0xfa, 0xa0]);
    assert_next(&mut asm, "dcb <a, >b", &[0x20, 0x10]);
}

#[test]
fn define_symbol() {
    let mut asm = assert_asm(".org $1000", &[]);
    asm.operand_parser.define_symbol("init", 0x1234);
    assert_next(&mut asm, "lda init", &[0xad, 0x34, 0x12]);
    assert_eq!(asm.location_counter, 0x1003);
}
