// tests/disassembler.rs

use minivm::disassembler::disassemble_bytes;
use minivm::bytecode::{OP_NOP, OP_ADD, OP_HALT, OP_PUSH, OP_POP};

#[test]
fn test_disassemble_empty_stream() {
    let bytecode: [u8; 0] = [];
    let result = disassemble_bytes(&bytecode);
    
    assert!(result.is_ok());
    let output = result.unwrap();
    
    // It should gracefully output just the decorative file header comment
    assert!(output.contains("; --- Disassembled Bytecode Stream ---"));
}

#[test]
fn test_disassemble_nullary_sequence() {
    // Binary layout for sequential execution: nop -> add -> halt
    let bytecode = vec![OP_NOP, OP_ADD, OP_HALT];
    let result = disassemble_bytes(&bytecode);
    
    assert!(result.is_ok());
    let output = result.unwrap();
    
    // Verify addresses are calculated correctly and instruction strings match
    assert!(output.contains("0x0000:  nop"));
    assert!(output.contains("0x0001:  add"));
    assert!(output.contains("0x0002:  halt"));
}

#[test]
fn test_disassemble_operand_instructions() {
    // Generate a raw vector layout manually
    // 1. push r3, 512  -> [OP_PUSH, 3, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    // 2. pop r4        -> [OP_POP, 4]
    let mut bytecode = vec![OP_PUSH, 3];
    let value: i64 = 512;
    bytecode.extend_from_slice(&value.to_le_bytes());
    
    bytecode.push(OP_POP);
    bytecode.push(4); // r4

    let result = disassemble_bytes(&bytecode);
    assert!(result.is_ok());
    let output = result.unwrap();
    
    // Verify register formats and operand separation syntax matches printer rules
    assert!(output.contains("0x0000:  push r3, 512"));
    assert!(output.contains("0x000A:  pop  r4"));
}

#[test]
fn test_disassemble_invalid_opcode_trap() {
    // 0x99 is an undefined instruction variant in our core isa.rs specifications
    let bytecode = vec![OP_NOP, 0x99, OP_HALT];
    let result = disassemble_bytes(&bytecode);
    
    // The disassembler design throws a premature exit error when hitting junk bytes
    assert!(result.is_err());
    let err_msg = result.err().unwrap().to_string();
    assert!(err_msg.contains("Disassembly halted prematurely") || err_msg.contains("0x0001"));
}