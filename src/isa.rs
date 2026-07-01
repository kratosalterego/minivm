// src/isa.rs

/// High-level, structural representation of an instruction and its arguments.
/// This is what the Parser outputs, the Disassembler prints, and the CPU executes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    // Nullary instructions (0 operands)
    Nop,
    Halt,
    Add,
    Sub,
    Mul,
    Div,
    Ret,

    // Monadic instructions (1 operand)
    Pop(u8),         // Pop into Register ID (u8)
    Jmp(u64),        // Jump to absolute byte address (u64)
    Jz(u64),         // Jump to absolute address if top of stack is zero
    Jnz(u64),        // Jump to absolute address if top of stack is non-zero
    Syscall(u8),     // Trigger system interrupt vector (u8)

    // Dyadic instructions (2 operands)
    Push(u8, i64),   // Push Register ID, immediate integer constant (i64)
    Load(u8, u64),   // Load from Global Address (u64) into Register ID
    Store(u8, u64),  // Store Register ID value into Global Address (u64)
}

/// Raw 1-byte tokens representing individual operation codes.
/// This maps directly to your binary bytecode format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    Nop     = 0x00,
    Halt    = 0xFF,
    
    Add     = 0x10,
    Sub     = 0x11,
    Mul     = 0x12,
    Div     = 0x13,
    Ret     = 0x14,

    Pop     = 0x20,
    Push    = 0x21,

    Jmp     = 0x30,
    Jz      = 0x31,
    Jnz     = 0x32,

    Load    = 0x40,
    Store   = 0x41,

    Syscall = 0x50,
}

impl Opcode {
    /// Safely attempts to match a raw byte into a recognized ISA Opcode.
    pub fn from_u8(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(Opcode::Nop),
            0xFF => Some(Opcode::Halt),
            0x10 => Some(Opcode::Add),
            0x11 => Some(Opcode::Sub),
            0x12 => Some(Opcode::Mul),
            0x13 => Some(Opcode::Div),
            0x14 => Some(Opcode::Ret),
            0x20 => Some(Opcode::Pop),
            0x21 => Some(Opcode::Push),
            0x30 => Some(Opcode::Jmp),
            0x31 => Some(Opcode::Jz),
            0x32 => Some(Opcode::Jnz),
            0x40 => Some(Opcode::Load),
            0x41 => Some(Opcode::Store),
            0x50 => Some(Opcode::Syscall),
            _    => None,
        }
    }
}