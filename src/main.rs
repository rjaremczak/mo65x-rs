#[macro_use]
extern crate lazy_static;

mod emulator;
mod gui;
mod mos6510;

use emulator::Emulator;
use mos6510::{assembler, disassembler::disassemble_file, error::AppError};
use std::io::Write;
use std::time::Duration;
use std::{fs::File, num::ParseIntError, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(author, about = "My Own 65xx emulator and more...")]
struct CliOpt {
    /// Execution mode
    #[structopt(subcommand)]
    mode: Mode,
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
    Dis {
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
    /// Run machine code
    Run {
        /// Binary file path
        #[structopt(parse(from_os_str))]
        bin: PathBuf,
        /// Start address in hex
        #[structopt(parse(try_from_str = parse_hex))]
        start_addr: u16,
        /// Frequency of CPU clock in kHz
        #[structopt(short, default_value = "1000")]
        freq_khz: u32,
    },
    /// Interactive console
    Con,
}

fn parse_hex(hex: &str) -> Result<u16, ParseIntError> {
    u16::from_str_radix(hex, 16)
}

fn main() {
    let cliopt = CliOpt::from_args();
    println!("cliopt: {:#?}", cliopt);
    let result = match cliopt.mode {
        Mode::Asm { src, bin, dump_symbols } => assemble(src, bin, dump_symbols),
        Mode::Dis { start_addr, end_addr, bin } => disassemble(start_addr, end_addr, bin),
        Mode::Run { start_addr, bin, freq_khz } => run(start_addr, bin, freq_khz),
        Mode::Con => console(),
    };
    if result.is_err() {
        eprintln!("exit result: {:?}", result.err().unwrap())
    }
}

fn assemble(src: PathBuf, bin: Option<PathBuf>, dump_symbols: bool) -> Result<(), AppError> {
    print!("assembling file {:?} ... ", src);
    let (origin, code, symbols) = assembler::assemble_file(&src)?;
    println!("ok, {} B [{:04X}-{:04X}]", code.len(), origin, origin as usize + code.len() - 1);
    let bin = bin.unwrap_or({
        let mut path = PathBuf::new();
        path.set_file_name(src.file_name().unwrap());
        path.set_extension("bin");
        path
    });
    print!("writting file: {:#?} ... ", bin);
    File::create(&bin)?.write_all(&code)?;
    println!("ok");

    if dump_symbols {
        println!("symbol table ({} items):", symbols.len());
        symbols.iter().for_each(|(k, v)| println!("\"{}\" = {:04X}", *k, *v as u16));
    }

    Ok(())
}

fn disassemble(start_addr: u16, end_addr: Option<u16>, bin: PathBuf) -> Result<(), AppError> {
    print!("disassembling file {:?} from address {:04X} ", bin, start_addr);
    match end_addr {
        Some(addr) => println!("to {:04X} ...", addr),
        None => println!("..."),
    }
    disassemble_file(start_addr, end_addr, bin)?.iter().for_each(|l| println!("{}", l));
    Ok(())
}

fn run(start_addr: u16, fname: PathBuf, freq_khz: u32) -> Result<(), AppError> {
    let mut emulator = Emulator::new();
    emulator.init();
    print!("uploading file {:?} ... ", fname);
    let size = emulator.upload(start_addr, fname)?;
    println!("ok, {} B [{:04X}-{:04X}]", size, start_addr, start_addr + size as u16 - 1);
    println!("clock speed: {} kHz", freq_khz);
    println!("start address: {:04X})", start_addr);
    println!("running, press a key to stop...");
    emulator.run(start_addr, Duration::from_secs(1) / (freq_khz * 1000));
    emulator.stop();
    println!("stopped");
    Ok(())
}

fn console() -> Result<(), AppError> {
    eprint!("not yet implemented");
    Ok(())
}
