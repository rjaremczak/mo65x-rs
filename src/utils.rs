use crate::mos6510::error::AppError;
use std::{fs::File, io::Read, io::Write, path::Path};

pub fn read_file_to_string<F: AsRef<Path>>(fname: F) -> Result<String, AppError> {
    let mut strbuf = String::new();
    File::open(&fname)
        .map_err(AppError::from_io)?
        .read_to_string(&mut strbuf)
        .map_err(AppError::from_io)?;
    Ok(strbuf)
}

pub fn write_string_to_file<F: AsRef<Path>>(bin: &Vec<u8>, fname: F) -> Result<(), AppError> {
    File::create(fname)
        .map_err(AppError::from_io)?
        .write_all(bin)
        .map_err(AppError::from_io)?;
    Ok(())
}
