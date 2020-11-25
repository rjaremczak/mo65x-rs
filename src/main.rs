mod emulator;
mod mos6510;

use emulator::Emulator;
use mos6510::{assembler, error::AppError};
use std::io;
use std::path::PathBuf;
use std::time::Duration;
use structopt::StructOpt;

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{cursor, execute};
use std::io::{stdout, Write};

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
        #[structopt(short, parse(from_os_str))]
        src: PathBuf,
        /// Binary file path
        #[structopt(short, parse(from_os_str))]
        bin: Option<PathBuf>,
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
        /// Start address
        #[structopt(short)]
        addr: u16,
        /// Binary file path
        #[structopt(short, parse(from_os_str))]
        bin: PathBuf,
        /// Frequency of CPU clock in kHz
        #[structopt(short, default_value = "1000")]
        freq_khz: u32,
    },
    /// Interactive console
    Con,
}

fn main() {
    let cliopt = CliOpt::from_args();
    println!("cliopt: {:#?}", cliopt);
    let _ = match cliopt.mode {
        Mode::Asm { src, bin } => assemble(src, bin),
        Mode::Dis { addr: origin, bin } => disassemble(origin, bin),
        Mode::Run {
            addr: origin,
            bin,
            freq_khz,
        } => run(origin, bin, freq_khz),
        Mode::Con => console(),
    };
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

fn assemble(src: PathBuf, bin: Option<PathBuf>) -> Result<(), AppError> {
    print!("assembling file {:#?} ... ", src);
    let (origin, code) = assembler::assemble_file(&src)?;
    println!("ok, {} B [${:04X} - ${:04X}]", code.len(), origin, origin as usize + code.len() - 1);
    let bin = bin.unwrap_or({
        let mut path = src.clone();
        path.set_extension("bin");
        path
    });
    println!("output file: {:#?}", bin);
    Ok(())
}

fn disassemble(origin: u16, bin: PathBuf) -> Result<(), AppError> {
    eprint!("not yet implemented");
    Ok(())
}

fn run(origin: u16, bin: PathBuf, freq_khz: u32) -> Result<(), AppError> {
    let mut emulator = Emulator::new();
    emulator.init();
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
