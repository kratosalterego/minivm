// tests/assembler.rs

use minivm::assembler::compile;
use minivm::bytecode::{OP_ADD, OP_HALT, OP_NOP, OP_PUSH};

#[test]
fn test_assemble_empty_and_whitespace() {
    // Empty programs or programs with only comments/newlines should emit empty bytecode vectors
    let source = "   \n\n ; This is just a comment\n    \t\n";
    let result = compile(source);
    
    assert!(result.is_ok());
    let bytecode = result.unwrap();
    // Note: If your encoder automatically injects headers, adjust this check to match your header length!
    assert!(bytecode.is_empty() || bytecode.len() == 6); 
}

#[test]
fn test_assemble_simple_instructions() {
    // Assemble simple nullary instructions
    let source = "nop\nadd\nhalt";
    let result = compile(source);
    
    assert!(result.is_ok());
    let bytecode = result.unwrap();
    
    // If your encoder appends headers, skip them to test raw instruction bytes
    let instruction_bytes = if bytecode.starts_with(&[0x00, 0x6D, 0x56, 0x4D]) {
        &bytecode[6..]
    } else {
        &bytecode[..]
    };

    assert_eq!(instruction_bytes[0], OP_NOP);
    assert_eq!(instruction_bytes[1], OP_ADD);
    assert_eq!(instruction_bytes[2], OP_HALT);
}

#[test]
fn test_assemble_push_instruction() {
    // Test instruction parsing with register and immediate values
    let source = "push r1, 42\npush r5, -10";
    let result = compile(source);
    
    assert!(result.is_ok());
    let bytecode = result.unwrap();
    
    let inst_bytes = if bytecode.starts_with(&[0x00, 0x6D, 0x56, 0x4D]) {
        &bytecode[6..]
    } else {
        &bytecode[..]
    };

    // First instruction: push r1, 42
    // Layout: [Opcode (1B)] [Reg (1B)] [Immediate (8B Little Endian)]
    assert_eq!(inst_bytes[0], OP_PUSH);
    assert_eq!(inst_bytes[1], 1); // r1
    let val1 = i64::from_le_bytes(inst_bytes[2..10].try_into().unwrap());
    assert_eq!(val1, 42);

    // Second instruction: push r5, -10
    assert_eq!(inst_bytes[10], OP_PUSH);
    assert_eq!(inst_bytes[11], 5); // r5
    let val2 = i64::from_le_bytes(inst_bytes[12..20].try_into().unwrap());
    assert_eq!(val2, -10);
}

#[test]
fn test_assemble_labels_and_jumps() {
    // Test that labels are resolved correctly to byte locations
    let source = "
    start:
        nop
        jmp start
    ";
    let result = compile(source);
    assert!(result.is_ok());
    
    let bytecode = result.unwrap();
    let inst_bytes = if bytecode.starts_with(&[0x00, 0x6D, 0x56, 0x4D]) {
        &bytecode[6..]
    } else {
        &bytecode[..]
    };

    // 'start' points to address 0x0000 (where 'nop' is)
    // Layout: [nop (1B)] [jmp (1B)] [target (8B)]
    assert_eq!(inst_bytes[0], OP_NOP);
    assert_eq!(inst_bytes[1], 0x30); // OP_JMP
    
    let jump_target = u64::from_le_bytes(inst_bytes[2..10].try_into().unwrap());
    assert_eq!(jump_target, 0); // Points back to index 0 (nop)
}

#[test]
fn test_lexer_error_invalid_char() {
    // Characters like '@' aren't supported in our lexer rules
    let source = "push r1, @42";
    let result = compile(source);
    
    assert!(result.is_err());
    let err_msg = result.err().unwrap().to_string();
    assert!(err_msg.contains("Lexical Analysis Error") || err_msg.contains("Unexpected character"));
}

#[test]
fn test_parser_error_missing_comma() {
    // Intentionally missing a comma between operands
    let source = "push r1 42";
    let result = compile(source);
    
    assert!(result.is_err());
    let err_msg = result.err().unwrap().to_string();
    assert!(err_msg.contains("Parsing Error") || err_msg.contains("Expected newline after instruction"));
}

#[test]
fn test_encoder_error_duplicate_label() {
    // Declaring the exact same label twice should crash the assembler safely
    let source = "
    loop:
        nop
    loop:
        halt
    ";
    let result = compile(source);
    
    assert!(result.is_err());
    let err_msg = result.err().unwrap().to_string();
    assert!(err_msg.contains("Duplicate label definition"));
}