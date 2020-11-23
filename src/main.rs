mod emulator;
mod mos6510;

use emulator::Emulator;
use std::path::PathBuf;
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
        src: Option<PathBuf>,
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
        bin: Option<PathBuf>,
    },
    /// Run machine code
    Run {
        /// Start address
        #[structopt(short)]
        addr: u16,
        /// Binary file path
        #[structopt(short, parse(from_os_str))]
        bin: Option<PathBuf>,
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
    match cliopt.mode {
        Mode::Asm { src, bin } => assemble(src.unwrap(), bin.unwrap()),
        Mode::Dis { addr: origin, bin } => disassemble(origin, bin.unwrap()),
        Mode::Run {
            addr: origin,
            bin,
            freq_khz,
        } => run(origin, bin.unwrap(), freq_khz),
        Mode::Con => console(),
    }
}

fn assemble(src: PathBuf, bin: PathBuf) {
    let mut assembler = mos6510::assembler::Assembler::new();
}

fn disassemble(origin: u16, bin: PathBuf) {
    eprint!("not yet implemented");
}

fn run(origin: u16, bin: PathBuf, freq_khz: u32) {
    let mut emulator = Emulator::new();
    emulator.init();
    println!("running...");
    loop {}
}

fn console() {
    eprint!("not yet implemented");
}
