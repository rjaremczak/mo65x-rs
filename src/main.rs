mod emulator;
mod gui;
mod mos6510;
mod utils;

use crossterm::event::{read, Event};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use emulator::Emulator;
use mos6510::{assembler, error::AppError};
use std::{num::ParseIntError, path::PathBuf};
use structopt::StructOpt;
use utils::write_string_to_file;

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
        /// Start address
        #[structopt(short)]
        addr: u16,
        /// Binary file path
        #[structopt(short, parse(from_os_str))]
        bin: PathBuf,
    },
    /// Run machine code
    Run {
        /// Binary file path
        #[structopt(parse(from_os_str))]
        bin: PathBuf,
        /// Start address in hex
        #[structopt(parse(try_from_str = parse_hex))]
        addr: u16,
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
        Mode::Dis { addr: origin, bin } => disassemble(origin, bin),
        Mode::Run {
            addr: origin,
            bin,
            freq_khz,
        } => run(origin, bin, freq_khz),
        Mode::Con => console(),
    };
    if result.is_err() {
        eprintln!("exit result: {:?}", result.err().unwrap())
    }
}

fn wait_for_any_key() {
    enable_raw_mode().unwrap();
    loop {
        if let Event::Key(_) = read().unwrap() {
            break;
        }
    }

    disable_raw_mode().unwrap();
}

fn assemble(src: PathBuf, bin: Option<PathBuf>, dump_symbols: bool) -> Result<(), AppError> {
    print!("assembling file {:#?} ... ", src);
    let (origin, code, symbols) = assembler::assemble_file(&src)?;
    println!("ok, {} B [${:04X} - ${:04X}]", code.len(), origin, origin as usize + code.len() - 1);
    let bin = bin.unwrap_or({
        let mut path = PathBuf::new();
        path.set_file_name(src.file_name().unwrap());
        path.set_extension("bin");
        path
    });
    print!("writting file: {:#?} ... ", bin);
    write_string_to_file(&code, &bin)?;
    println!("ok");

    if dump_symbols {
        println!("symbol table ({} items):", symbols.len());
        symbols.iter().for_each(|(k, v)| println!("\"{}\" = {}", *k, *v));
    }

    Ok(())
}

fn disassemble(origin: u16, bin: PathBuf) -> Result<(), AppError> {
    eprint!("not yet implemented");
    Ok(())
}

fn run(origin: u16, bin: PathBuf, freq_khz: u32) -> Result<(), AppError> {
    let mut emulator = Emulator::new();
    emulator.init();
    // emulator.load
    emulator.run();
    println!("Emulator is running, press any key to quit");
    wait_for_any_key();
    emulator.stop();
    println!("stop");
    Ok(())
}

fn console() -> Result<(), AppError> {
    eprint!("not yet implemented");
    Ok(())
}
