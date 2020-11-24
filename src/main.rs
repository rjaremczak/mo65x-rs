mod emulator;
mod mos6510;

use emulator::Emulator;
use mos6510::{assembler, error::AppError};
use std::io;
use std::path::PathBuf;
use std::time::Duration;
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
        #[structopt(short, parse(from_os_str))]
        src: PathBuf,
        /// Binary file path
        #[structopt(short, parse(from_os_str))]
        bin: PathBuf,
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
    let result = match cliopt.mode {
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

fn assemble(src: PathBuf, bin_path: PathBuf) -> Result<(), AppError> {
    print!("assembling file {:#?} ... ", src);
    let (origin, code) = assembler::assemble_file(src)?;
    println!("ok");
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
    println!("Emulator is running, press ENTER to quit");
    std::thread::sleep(Duration::from_secs(1));
    let _ = io::stdin().read_line(&mut String::new());
    emulator.stop();
    println!("stop");
    Ok(())
}

fn console() -> Result<(), AppError> {
    eprint!("not yet implemented");
    Ok(())
}
