// src/main.rs

// 1. Declare all the structural project submodules
mod bytecode;
mod cli;          // <-- MAKE SURE THIS LINE IS PRESENT AND NOT MISSING OR TYPO'D!
mod isa;
mod error;
mod assembler;
mod vm;
mod disassembler;
mod util;

// 2. Bring items into global scope for clean routing
use clap::Parser;
use cli::{Cli, Commands}; // This pulls in your custom arguments structures
use vm::VmConfig;
use std::process;

// ... the rest of your main.rs functions (handle_assemble, handle_run, etc.)

fn main() {
    let args = Cli::parse();

    if let Err(err) = match args.command {
        Commands::Assemble(asm_args) => handle_assemble(asm_args),
        Commands::Run(run_args) => handle_run(run_args),
        Commands::Disassemble(dis_args) => handle_disassemble(dis_args),
    } {
        eprintln!("\x1b[1;31merror\x1b[0m: {}", err);
        process::exit(1);
    }
}

fn handle_assemble(args: cli::AssembleArgs) -> error::Result<()> {
        let output_path = args.output.unwrap_or_else(|| {
        let mut path = args.input.clone();
        path.set_extension("bin");
        path
    });

    println!("Assembling {} ...", args.input.display());

    let source_code = util::file::read_source_file(&args.input)?;

    match assembler::compile(&source_code) {
        Ok(bytecode_body) => {
            util::file::write_binary_file(&output_path, &bytecode_body)?;
            println!(
                "\x1b[1;32mSuccess\x1b[0m: Written to {}",
                output_path.display()
            );
            Ok(())
        }
        Err(compile_err) => Err(format!(
            "Compilation stopped due to previous errors.\nDetails: {}",
            compile_err
        )
        .into()),
    }
}
// src/main.rs -> inside handle_run()

fn handle_run(args: cli::RunArgs) -> error::Result<()> {
            // 1. Load the binary file from disk
    let raw_bytes = util::file::read_binary_file(&args.input)?;

    // 2. Verify the header and get the starting index offset (should be 6)
    let header_offset = bytecode::verify_header(&raw_bytes)
        .map_err(|e| error::MiniVmError::InvalidBinaryHeader(e))?;
    
    // 3. CRITICAL: Slice the array to completely skip the header bytes!
    let executable_bytecode = &raw_bytes[header_offset..];

    let config = VmConfig {
        stack_size: args.stack_size,
        globals_size: args.globals_size,
        enable_tracing: args.trace,
    };

    println!("Executing {} ...", args.input.display());
    
    let mut vm = vm::Vm::new(config);
    
    // 4. CRITICAL: Pass the sliced 'executable_bytecode', NOT 'raw_bytes'!
    vm.execute(executable_bytecode)?;

    Ok(())
}

fn handle_disassemble(args: cli::DisassembleArgs) -> error::Result<()> {
    
    let raw_bytes = util::file::read_binary_file(&args.input)?;

    let header_offset =
        bytecode::verify_header(&raw_bytes).map_err(error::MiniVmError::InvalidBinaryHeader)?;

    let executable_bytecode = &raw_bytes[header_offset..];

    let disassembly_output = disassembler::disassemble_bytes(executable_bytecode)?;

    if let Some(output_path) = args.output {
        util::file::write_text_file(&output_path, &disassembly_output)?;
        println!(
            "\x1b[1;32mSuccess\x1b[0m: Disassembly saved to {}",
            output_path.display()
        );
    } else {
        println!("{}", disassembly_output);
    }

    Ok(())
}
