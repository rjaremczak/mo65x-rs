pub struct ExecEnv<'a> {
    pub inval_lo: u8,
    pub inval_hi: u8,
    pub outref_lo: Option<&'a mut u8>,
    pub outref_hi: Option<&'a mut u8>,
    pub page_crossed: bool,
    pub cycles: u8,
}

impl<'a> ExecEnv<'a> {
    pub fn new(cycles: u8) -> Self {
        Self {
            inval_lo: 0,
            inval_hi: 0,
            outref_lo: None,
            outref_hi: None,
            page_crossed: false,
            cycles,
        }
    }
}
