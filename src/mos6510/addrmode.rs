use super::exec_env::{ExecEnv, ExecEnvHandler};

pub struct AddrMode<'a> {
    pub handler: ExecEnvHandler,
    pub op_size: u8,
    pub zp_mode: Option<&'a AddrMode<'a>>,
}

impl<'a> PartialEq for AddrMode<'a> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

pub static IMPLIED: AddrMode = AddrMode {
    handler: ExecEnv::prep_implied,
    op_size: 0,
    zp_mode: None,
};

pub static BRANCH: AddrMode<'static> = AddrMode {
    handler: ExecEnv::prep_branch,
    op_size: 1,
    zp_mode: None,
};

pub static IMMEDIATE: AddrMode = AddrMode {
    handler: ExecEnv::prep_immediate,
    op_size: 1,
    zp_mode: None,
};

pub static ZERO_PAGE: AddrMode = AddrMode {
    handler: ExecEnv::prep_zero_page,
    op_size: 1,
    zp_mode: None,
};

pub static ZERO_PAGE_X: AddrMode = AddrMode {
    handler: ExecEnv::prep_zero_page_x,
    op_size: 1,
    zp_mode: None,
};

pub static ZERO_PAGE_Y: AddrMode = AddrMode {
    handler: ExecEnv::prep_zero_page_y,
    op_size: 1,
    zp_mode: None,
};

pub static INDEXED_INDIRECT_X: AddrMode = AddrMode {
    handler: ExecEnv::prep_indexed_indirect_x,
    op_size: 1,
    zp_mode: None,
};

pub static INDIRECT_INDEXED_Y: AddrMode = AddrMode {
    handler: ExecEnv::prep_indirect_indexed_y,
    op_size: 1,
    zp_mode: None,
};

pub static INDIRECT: AddrMode = AddrMode {
    handler: ExecEnv::prep_indirect,
    op_size: 2,
    zp_mode: None,
};

pub static ABSOLUTE: AddrMode = AddrMode {
    handler: ExecEnv::prep_absolute,
    op_size: 2,
    zp_mode: Some(&ZERO_PAGE),
};

pub static ABSOLUTE_X: AddrMode = AddrMode {
    handler: ExecEnv::prep_absolute_x,
    op_size: 2,
    zp_mode: Some(&ZERO_PAGE_X),
};

pub static ABSOLUTE_Y: AddrMode = AddrMode {
    handler: ExecEnv::prep_absolute_y,
    op_size: 2,
    zp_mode: Some(&ZERO_PAGE_Y),
};
