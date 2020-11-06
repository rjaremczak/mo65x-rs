pub struct ExecEnv<'a> {
    inval_lo: u8,
    inval_hi: u8,
    outref_lo: &'a mut u8,
    outref_hi: &'a mut u8,
    sink_lo: u8,
    sink_hi: u8,
    page_crossed: bool,
    cycles: u8,
}
