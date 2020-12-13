#[macro_use]
extern crate lazy_static;

mod backend;
mod frontend;
mod gui;
mod mos6510;

use backend::Backend;
use frontend::Frontend;
use mos6510::{assembler, disassembler::disassemble_file, error::AppError};
use std::{fs::File, num::ParseIntError, path::PathBuf, sync::RwLock};
use std::{io::Write, thread};
use std::{sync::Arc, time::Duration};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(author, about = "My Own 65xx emulator and more...")]
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
    /// Run machine code
    Exec {
        /// Binary file path
        #[structopt(parse(from_os_str))]
        bin: PathBuf,
        /// Start address in hex
        #[structopt(parse(try_from_str = parse_hex))]
        start_addr: u16,
        /// Frequency of CPU clock in Hz
        #[structopt(short, default_value = "1e6")]
        freq: f64,
    },
    /// Interactive console
    Interactive,
}

fn parse_hex(hex: &str) -> Result<u16, ParseIntError> {
    u16::from_str_radix(hex, 16)
}

fn main() {
    let cliopt = CliOpt::from_args();
    println!("cliopt: {:#?}", cliopt);
    let result = match cliopt.mode.unwrap_or(Mode::Interactive) {
        Mode::Asm { src, bin, dump_symbols } => assemble(src, bin, dump_symbols),
        Mode::Dasm { start_addr, end_addr, bin } => disassemble(start_addr, end_addr, bin),
        Mode::Exec { start_addr, bin, freq } => execute(start_addr, bin, freq),
        Mode::Interactive => interactive(),
    };
    if result.is_err() {
        eprintln!("exit result: {:?}", result.err().unwrap())
    }
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

fn disassemble(start_addr: u16, end_addr: Option<u16>, bin: PathBuf) -> Result<(), AppError> {
    print!("binary file {:?}, disassemble from address {:04X} ", bin, start_addr);
    match end_addr {
        Some(addr) => println!("to {:04X} ...", addr),
        None => println!("..."),
    }
    disassemble_file(start_addr, end_addr, bin)?.iter().for_each(|l| println!("{}", l));
    Ok(())
}

fn execute(start_addr: u16, fname: PathBuf, freq: f64) -> Result<(), AppError> {
    let lock = Arc::new(RwLock::new(Backend::new()));
    {
        let mut backend = lock.write().unwrap();
        backend.init();
        print!("uploading file {:?} ... ", fname);
        let size = backend.upload(start_addr, fname)?;
        println!("ok, {} B [{:04X}-{:04X}]", size, start_addr, start_addr + size as u16 - 1);
        println!("clock speed: {} MHz", freq / 1e6);
        println!("start address: {:04X}", start_addr);
        backend.set_reg_pc(start_addr);
    }
    let thread_lock = lock.clone();
    let handle = thread::spawn(move || thread_lock.write().unwrap().run(Duration::from_secs_f64(1.0 / freq)));
    let backend = lock.read().unwrap();
    let mut frontend = Frontend::new();
    println!("running, press a key to stop...");
    while frontend.is_running() {
        // TODO: read and process command from UI
        frontend.update();
        frontend.sleep();
    }
    backend.set_step_mode(true);
    println!("stopping...");
    handle.join().unwrap();
    println!("stopped");
    Ok(())
}

fn interactive() -> Result<(), AppError> {
    eprint!("not yet implemented");
    Ok(())
}
