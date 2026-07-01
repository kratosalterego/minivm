// tests/vm.rs

use minivm::isa::Instruction;
use minivm::vm::{Vm, VmConfig};
use minivm::bytecode::{OP_NOP, OP_HALT, OP_ADD, OP_SUB, OP_MUL, OP_DIV, OP_PUSH, OP_POP};

/// Helper to wrap raw instruction payloads in a clean, isolated default VM execution run
fn run_vm_bytes(bytes: &[u8]) -> Result<Vm, String> {
    let mut vm = Vm::new(VmConfig::default());
    vm.execute(bytes).map_err(|e| e.to_string())?;
    Ok(vm)
}

#[test]
fn test_vm_nop_and_halt() {
    let bytecode = vec![OP_NOP, OP_NOP, OP_HALT];
    let result = run_vm_bytes(&bytecode);
    
    assert!(result.is_ok());
    let vm = result.unwrap();
    
    // CPU should halt gracefully and track the correct final PC position
    assert_eq!(vm.cpu().running, false);
    assert_eq!(vm.cpu().pc, 3); 
}

#[test]
fn test_vm_push_pop_registers() {
    // Manually serialize: push r2, 584 -> pop r5 -> halt
    let mut bytecode = vec![OP_PUSH, 2];
    let value: i64 = 584;
    bytecode.extend_from_slice(&value.to_le_bytes());
    bytecode.push(OP_POP);
    bytecode.push(5); // r5
    bytecode.push(OP_HALT);

    let vm = run_vm_bytes(&bytecode).unwrap();
    let cpu = vm.cpu();

    // Verify values made the full roundtrip through stack allocation boundaries into registers
    assert_eq!(cpu.registers[2], 584);
    assert_eq!(cpu.registers[5], 584);
    assert!(cpu.stack.is_empty());
}

#[test]
fn test_vm_math_operations() {
    // Sequence: push r0, 20 -> push r0, 4 -> mul -> push r0, 5 -> sub -> halt
    // Formula: (20 * 4) - 5 = 75
    let mut bytecode = Vec::new();
    
    // push r0, 20
    bytecode.push(OP_PUSH); bytecode.push(0); bytecode.extend_from_slice(&20i64.to_le_bytes());
    // push r0, 4
    bytecode.push(OP_PUSH); bytecode.push(0); bytecode.extend_from_slice(&4i64.to_le_bytes());
    // mul
    bytecode.push(OP_MUL);
    // push r0, 5
    bytecode.push(OP_PUSH); bytecode.push(0); bytecode.extend_from_slice(&5i64.to_le_bytes());
    // sub
    bytecode.push(OP_SUB);
    // pop r1 (to check result)
    bytecode.push(OP_POP); bytecode.push(1);
    bytecode.push(OP_HALT);

    let vm = run_vm_bytes(&bytecode).unwrap();
    assert_eq!(vm.cpu().registers[1], 75);
}

#[test]
fn test_vm_stack_underflow_protection() {
    // Executing an operational math instruction on an empty stack frame must crash safely
    let bytecode = vec![OP_ADD, OP_HALT];
    let result = run_vm_bytes(&bytecode);
    
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Stack Underflow"));
}

#[test]
fn test_vm_stack_overflow_protection() {
    // Build a miniature stack limit threshold to trigger boundaries quickly
    let small_config = VmConfig {
        stack_size: 2,
        globals_size: 10,
        enable_tracing: false,
    };
    
    let mut vm = Vm::new(small_config);
    
    // Attempting to push 3 items onto a size-2 stack capacity
    let mut bytecode = Vec::new();
    bytecode.push(OP_PUSH); bytecode.push(0); bytecode.extend_from_slice(&1i64.to_le_bytes());
    bytecode.push(OP_PUSH); bytecode.push(0); bytecode.extend_from_slice(&2i64.to_le_bytes());
    bytecode.push(OP_PUSH); bytecode.push(0); bytecode.extend_from_slice(&3i64.to_le_bytes());
    
    let result = vm.execute(&bytecode);
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Stack Overflow"));
}

#[test]
fn test_vm_conditional_jumps() {
    // Logic: push r0, 0 -> jz target -> push r1, 99 -> halt -> target: push r1, 77 -> halt
    // Since 0 is dropped off stack, it should skip 99 and jump straight to storing 77.
    let mut bytecode = Vec::new();
    
    // 0: push r0, 0
    bytecode.push(OP_PUSH); bytecode.push(0); bytecode.extend_from_slice(&0i64.to_le_bytes());
    
    // 10: jz to address 29 (where the target push starts)
    bytecode.push(0x31); // OP_JZ
    let target_addr: u64 = 10 + 1 + 8 + 1 + 1 + 8 + 1; // Calculate offset: first push + jz + push + halt
    bytecode.extend_from_slice(&target_addr.to_le_bytes());
    
    // 19: push r1, 99 (Should be skipped!)
    bytecode.push(OP_PUSH); bytecode.push(1); bytecode.extend_from_slice(&99i64.to_le_bytes());
    // 28: halt
    bytecode.push(OP_HALT);
    
    // 29: target label: push r1, 77
    bytecode.push(OP_PUSH); bytecode.push(1); bytecode.extend_from_slice(&77i64.to_le_bytes());
    // 38: halt
    bytecode.push(OP_HALT);

    let vm = run_vm_bytes(&bytecode).unwrap();
    assert_eq!(vm.cpu().registers[1], 77);
}