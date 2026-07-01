// src/main.rs

mod bytecode;
mod cli;
mod isa;
mod error;
mod assembler;
mod vm;
mod disassembler;
mod util;

use clap::Parser;
use std::process;

use cli::{Cli, Commands};
use vm::VmConfig;

fn main() {
    // 1. Parse command line arguments via clap
    let args = Cli::parse();

    // 2. Route incoming commands to their respective toolchain subsystem
    if let Err(err) = match args.command {
        Commands::Assemble(asm_args) => handle_assemble(asm_args),
        Commands::Run(run_args) => handle_run(run_args),
        Commands::Disassemble(dis_args) => handle_disassemble(dis_args),
    } {
        // Fallback generic pretty-printer for system-level errors
        eprintln!("\x1b[1;31merror\x1b[0m: {}", err);
        process::exit(1);
    }
}

/// Orchestrates the compilation pipeline: Text (.tasm) -> Binary (.bin)
fn handle_assemble(args: cli::AssembleArgs) -> error::Result<()> {
    // Determine target output path if not explicitly provided
    let output_path = args.output.unwrap_or_else(|| {
        let mut path = args.input.clone();
        path.set_extension("bin");
        path
    });

    println!("Assembling {} ...", args.input.display());

    // Read raw assembly text
    let source_code = util::file::read_source_file(&args.input)?;

    // Run compiler pipeline
    match assembler::compile(&source_code) {
        Ok(bytecode) => {
            // Write completed binary payload out to disk
            util::file::write_binary_file(&output_path, &bytecode)?;
            println!("\x1b[1;32mSuccess\x1b[0m: Written to {}", output_path.display());
            Ok(())
        }
        Err(compile_err) => {
            // Because the compiler threw a text error string, format it contextually.
            // In a mature iteration, you'd pass a structural Diagnostic struct out here.
            Err(format!("Compilation stopped due to previous errors.\nDetails: {}", compile_err).into())
        }
    }
}

/// Orchestrates the execution pipeline: Binary (.bin) -> Runtime VM Loop
fn handle_run(args: cli::RunArgs) -> error::Result<()> {
    // Load pre-compiled binary bytes from disk
    let raw_bytes = util::file::read_binary_file(&args.input)?;

    // Strip and validate binary format header frames
    let header_offset = bytecode::verify_header(&raw_bytes)
        .map_err(|e| error::MiniVmError::InvalidBinaryHeader(e))?;
    
    let executable_bytecode = &raw_bytes[header_offset..];

    // Build the VM instance configuration state
    let config = VmConfig {
        stack_size: args.stack_size,
        globals_size: args.globals_size,
        enable_tracing: args.trace,
    };

    println!("Executing {} ...", args.input.display());
    
    // Initialize and run the machine context
    let mut vm = vm::Vm::new(config);
    vm.execute(executable_bytecode)?;

    Ok(())
}

/// Orchestrates the reverse-engineering pipeline: Binary (.bin) -> Text (.tasm)
fn handle_disassemble(args: cli::DisassembleArgs) -> error::Result<()> {
    // Load pre-compiled binary bytes from disk
    let raw_bytes = util::file::read_binary_file(&args.input)?;

    // Strip and validate bytecode format header constraints
    let header_offset = bytecode::verify_header(&raw_bytes)
        .map_err(|e| error::MiniVmError::InvalidBinaryHeader(e))?;
    
    let executable_bytecode = &raw_bytes[header_offset..];

    // Perform reverse token mapping
    let disassembly_output = disassembler::disassemble_bytes(executable_bytecode)?;

    // Check if we write back out to a file or stream straight to standard console output
    if let Some(output_path) = args.output {
        util::file::write_text_file(&output_path, &disassembly_output)?;
        println!("\x1b[1;32mSuccess\x1b[0m: Disassembly saved to {}", output_path.display());
    } else {
        println!("{}", disassembly_output);
    }

    Ok(())
}