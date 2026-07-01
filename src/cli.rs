// src/cli.rs

use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "minivm",
    author = "Your Name",
    version = "1.0",
    about = "A production-grade custom virtual machine compiler and runtime toolchain",
    long_about = "minivm is a lightweight educational runtime architecture featuring an assembler, structural bytecode serializer, isolated VM runner, and static disassembler."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Compiles a human-readable text assembly file (.tasm) into raw binary bytecode (.bin)
    Assemble(AssembleArgs),

    /// Executes a pre-compiled binary bytecode file (.bin) inside the virtual machine runtime
    Run(RunArgs),

    /// Reverses a compiled binary file (.bin) back into standard human-readable text assembly notation
    Disassemble(DisassembleArgs),
}

#[derive(Args, Debug)]
pub struct AssembleArgs {
    /// Path to the input source file (e.g., source.tasm)
    #[arg(short, long, value_name = "FILE")]
    pub input: PathBuf,

    /// Path to output the compiled binary file (defaults to input name with a .bin extension)
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct RunArgs {
    /// Path to the compiled virtual machine binary file to run (e.g., program.bin)
    #[arg(short, long, value_name = "FILE")]
    pub input: PathBuf,

    /// Spawns a diagnostic timeline tracing execution metrics (PC, registers, stack) out to stdout after every cycle step
    #[arg(short, long, default_value_t = false)]
    pub trace: bool,

    /// Explicitly override the default maximum runtime stack space allocation ceiling
    #[arg(short, long, default_value_t = 1024)]
    pub stack_size: usize,

    /// Explicitly override the default maximum globals memory address buffer space mapping ceiling
    #[arg(short, long, default_value_t = 256)]
    pub globals_size: usize,
}

#[derive(Args, Debug)]
pub struct DisassembleArgs {
    /// Path to the compiled input binary file to decode back to text notation
    #[arg(short, long, value_name = "FILE")]
    pub input: PathBuf,

    /// Path to save the textual disassembled code output (prints directly to stdout if omitted)
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,
}