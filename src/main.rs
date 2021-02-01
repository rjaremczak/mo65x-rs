#[macro_use]
extern crate lazy_static;

mod console;
mod emulator;
mod error;
mod mos6510;
mod terminal;
mod video;

use console::Console;
use error::AppError;
use mos6510::{assembler, disassembler::disassemble_file};
use std::io::Write;
use std::{fs::File, path::PathBuf};
use structopt::StructOpt;

const APP_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug, StructOpt)]
#[structopt(about = "My Own 65xx emulator, assembler and disassembler")]
struct CliOpt {
    /// Execution mode
    #[structopt(subcommand)]
    mode: Option<Mode>,
}

#[derive(Debug, StructOpt)]
enum Mode {
    /// Assemble source to machine code
    Asm {
        /// Source file path
        #[structopt(parse(from_os_str))]
        src: PathBuf,
        /// Binary file path
        #[structopt(short = "o", parse(from_os_str))]
        bin: Option<PathBuf>,
        /// Dump symbol table
        #[structopt(short = "s")]
        dump_symbols: bool,
    },
    /// Disassemble machine code
    Dasm {
        /// Binary file path
        #[structopt(parse(from_os_str))]
        bin: PathBuf,
        /// Start address
        #[structopt(parse(try_from_str = parse_hex))]
        start_addr: u16,
        /// End address
        #[structopt(parse(try_from_str = parse_hex))]
        end_addr: Option<u16>,
    },
    /// Interactive console
    Console {
        /// Frequency of CPU clock in MHz
        #[structopt(default_value = "1.0")]
        clock_mhz: f64,
    },
}

fn parse_hex(hex: &str) -> Result<u16, AppError> {
    u16::from_str_radix(hex, 16).map_err(|e| AppError::ParseIntError(String::from(hex), e))
}

fn assemble(src: PathBuf, bin: Option<PathBuf>, dump_symbols: bool) -> Result<(), AppError> {
    println!("source file {:?}, assembling ...", src);
    let (origin, code, symbols) = assembler::assemble_file(&src)?;
    println!("code: {} B [{:04X}-{:04X}]", code.len(), origin, origin as usize + code.len() - 1);
    let bin = bin.unwrap_or({
        let mut path = PathBuf::new();
        path.set_file_name(src.file_name().unwrap());
        path.set_extension("bin");
        path
    });
    println!("writing file {:#?} ...", bin);
    File::create(&bin)?.write_all(&code)?;

    if dump_symbols {
        println!("symbol table ({} items):", symbols.len());
        symbols.iter().for_each(|(k, v)| println!("\"{}\" = {:04X}", *k, *v as u16));
    }

    Ok(())
}

fn print_disassembly_line(columns: &(String, String, String)) {
    println!("{}{}{}", columns.0, columns.1, columns.2)
}

fn disassemble(start_addr: u16, end_addr: Option<u16>, bin: PathBuf) -> Result<(), AppError> {
    print!("binary file {:?}, disassemble from address {:04X} ", bin, start_addr);
    match end_addr {
        Some(addr) => println!("to {:04X} ...", addr),
        None => println!("..."),
    }
    disassemble_file(start_addr, end_addr, bin)?.iter().for_each(print_disassembly_line);
    Ok(())
}

fn main() {
    let cliopt = CliOpt::from_args();
    let result = match cliopt.mode.unwrap_or(Mode::Console { clock_mhz: 1.0 }) {
        Mode::Asm { src, bin, dump_symbols } => assemble(src, bin, dump_symbols),
        Mode::Dasm { start_addr, end_addr, bin } => disassemble(start_addr, end_addr, bin),
        Mode::Console { clock_mhz } => Console::start(APP_NAME, clock_mhz * 1e6),
    };
    if let Err(apperr) = result {
        println!("\nerror: {:?}", apperr)
    }
}
