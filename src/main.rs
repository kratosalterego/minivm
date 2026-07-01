use clap::Parser;
use minivm::assembler;
use minivm::bytecode;
use minivm::cli::{Cli, Commands};
use minivm::disassembler;
use minivm::error;
use minivm::util;
use minivm::vm::{self, VmConfig};
use std::process;

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

fn handle_assemble(args: minivm::cli::AssembleArgs) -> error::Result<()> {
    let output_path = args.output.unwrap_or_else(|| {
        let mut path = args.input.clone();
        path.set_extension("bin");
        path
    });

    println!("Assembling {} ...", args.input.display());

    let source_code = util::file::read_source_file(&args.input)?;

    match assembler::compile(&source_code) {
        Ok(bytecode_body) => {
            let mut output = Vec::new();
            bytecode::write_header(&mut output);
            output.extend_from_slice(&bytecode_body);
            util::file::write_binary_file(&output_path, &output)?;
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

fn handle_run(args: minivm::cli::RunArgs) -> error::Result<()> {
    let raw_bytes = util::file::read_binary_file(&args.input)?;

    let header_offset =
        bytecode::verify_header(&raw_bytes).map_err(error::MiniVmError::InvalidBinaryHeader)?;

    let executable_bytecode = &raw_bytes[header_offset..];

    let config = VmConfig {
        stack_size: args.stack_size,
        globals_size: args.globals_size,
        enable_tracing: args.trace,
    };

    println!("Executing {} ...", args.input.display());

    let mut vm = vm::Vm::new(config);
    vm.execute(executable_bytecode)?;

    Ok(())
}

fn handle_disassemble(args: minivm::cli::DisassembleArgs) -> error::Result<()> {
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
