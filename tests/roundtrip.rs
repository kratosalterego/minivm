// tests/roundtrip.rs

use minivm::assembler::compile;
use minivm::bytecode::verify_header;
use minivm::disassembler::disassemble_bytes;

/// Core test runner that validates the compiler toolchain consistency loop:
/// Source A -> Bytes A -> Disassembled Source B -> Bytes B (Bytes A == Bytes B)
fn assert_roundtrip_integrity(original_source: &str) {
    // Phase 1: Compile original source text into machine code bytes
    let bytes_a = compile(original_source)
        .expect("Roundtrip Failure: Initial compilation phase failed.");

    // Phase 2: Strip headers to isolate raw structural executable instruction bytes
    let header_offset = verify_header(&bytes_a)
        .expect("Roundtrip Failure: Generated binary header failed validation.");
    let executable_bytes_a = &bytes_a[header_offset..];

    // Phase 3: Disassemble executable bytes back into human-readable text notation
    let disassembled_text = disassemble_bytes(executable_bytes_a)
        .expect("Roundtrip Failure: Disassembly parsing phase failed.");

    // Phase 4: Compile the freshly generated disassembly string back into a new byte array
    let bytes_b = compile(&disassembled_text)
        .expect("Roundtrip Failure: Secondary compilation of disassembled text failed.");
    
    let header_offset_b = verify_header(&bytes_b).unwrap();
    let executable_bytes_b = &bytes_b[header_offset_b..];

    // Phase 5: Final strict assertion. The binary payloads must be identical.
    assert_eq!(
        executable_bytes_a, 
        executable_bytes_b, 
        "Roundtrip divergence detected!\n\n[Original Source]:\n{}\n\n[Disassembled Context]:\n{}",
        original_source, 
        disassembled_text
    );
}

#[test]
fn test_roundtrip_basic_arithmetic() {
    assert_roundtrip_integrity("
        push r0, 100
        push r1, 200
        add
        pop  r2
        halt
    ");
}

#[test]
fn test_roundtrip_control_flow() {
    assert_roundtrip_integrity("
        start:
            push r0, 1
            push r1, 0
            sub
            jz   end
            jmp  start
        end:
            halt
    ");
}

#[test]
fn test_roundtrip_memory_and_traps() {
    assert_roundtrip_integrity("
        load  r4, 0x1000
        push  r5, 5
        mul
        store r4, 0x1008
        ret
    ");
}