// tests/integration.rs

use std::fs;
use std::path::PathBuf;
use minivm::assembler::compile;
use minivm::bytecode::verify_header;
use minivm::vm::{Vm, VmConfig};

/// Helper to resolve the workspace path to the examples directory safely
fn get_example_path(filename: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("examples");
    path.push(filename);
    path
}

#[test]
fn test_e2e_arithmetic_execution() {
    // 1. Locate the real source file path
    let source_path = get_example_path("arith.tasm");
    
    // Ensure the file exists or create a dummy one for the sake of the test environment
    if !source_path.exists() {
        fs::create_dir_all(source_path.parent().unwrap()).unwrap();
        fs::write(
            &source_path,
            "push r0, 20\npush r1, 22\nadd\npop r0\nhalt\n", // 20 + 22 = 42 -> stored in r0
        ).unwrap();
    }

    // 2. Read the source code from disk
    let source_code = fs::read_to_string(&source_path)
        .expect("Failed to read integration test source file");

    // 3. Compile: Text (.tasm) -> Bytecode Binary (.bin space layout)
    let compiled_bytecode = compile(&source_code)
        .expect("Integration Test Failure: Assembler compilation phase failed.");

    // 4. Validate and strip headers exactly like main.rs does
    let header_offset = verify_header(&compiled_bytecode)
        .expect("Integration Test Failure: Compiled binary header is invalid.");
    let executable_bytes = &compiled_bytecode[header_offset..];

    // 5. Instatiate the VM with custom strict boundaries
    let config = VmConfig {
        stack_size: 64,       // Small footprint for testing bounds
        globals_size: 16,
        enable_tracing: false, // Turn off stdout spam during testing runs
    };
    let mut vm = Vm::new(config);

    // 6. Execute the bytecode payload inside the runtime engine
    let execution_result = vm.execute(executable_bytes);
    assert!(execution_result.is_ok(), "VM crashed during integration runtime execution loop!");

    // 7. Assert on outcomes: Check if our computation matches expectations
    // 20 + 22 should equal 42, and it should reside safely inside register r0
    let target_cpu = vm.cpu();
    assert_eq!(target_cpu.registers[0], 42); 
    assert_eq!(target_cpu.running, false, "CPU loop failed to acknowledge Halt instruction state.");
    assert!(target_cpu.stack.is_empty(), "Evaluation stack frame failed to clear cleanly after pop operations.");
}

#[test]
fn test_e2e_runtime_division_by_zero_trap() {
    // Directly inject an invalid mathematical sequence to test VM error isolation
    let invalid_source = "
        push r0, 10
        push r1, 0
        div
        halt
    ";

    let compiled_bytecode = compile(invalid_source).unwrap();
    let header_offset = verify_header(&compiled_bytecode).unwrap();
    let executable_bytes = &compiled_bytecode[header_offset..];

    let mut vm = Vm::new(VmConfig::default());
    let execution_result = vm.execute(executable_bytes);

    // The integration test expects the execution loop to report an explicit hardware runtime panic
    assert!(execution_result.is_err());
    let err_message = execution_result.err().unwrap().to_string();
    assert!(err_message.contains("Division by zero"));
}