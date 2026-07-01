// src/error.rs

use thiserror::Error;

/// The central Result type used across the miniVM project workspace.
pub type Result<T> = std::result::Result<T, MiniVmError>;

#[derive(Error, Debug)]
pub enum MiniVmError {
    /// Errors originating from OS/File system operations
    #[error("I/O System Error: {0}")]
    Io(#[from] std::io::Error),

    /// Errors caught during tokenization or structural string slicing
    #[error("Lexical Analysis Error at line {line}, col {column}: {message}")]
    Lexical {
        message: String,
        line: usize,
        column: usize,
    },

    /// Errors caught during the grammar translation parsing loop
    #[error("Parsing Syntax Error at line {line}, col {column}: {message}")]
    Syntax {
        message: String,
        line: usize,
        column: usize,
    },

    /// Errors caught during label resolution or instruction serialization sizing
    #[error("Encoding Conversion Error: {message}")]
    Encoding {
        message: String,
    },

    /// Structural binary layout mismatches (bad magic bytes, wrong versions)
    #[error("Invalid Bytecode Binary Format Header: {0}")]
    InvalidBinaryHeader(String),

    /// Runtime exceptions thrown by the virtual CPU loop during live execution
    #[error("VM Runtime Exception at PC offset 0x{pc:04X}: {message}")]
    RuntimeException {
        message: String,
        pc: usize,
    },

    /// A catch-all context variant to bridge generic string allocations smoothly
    #[error("Toolchain Error: {0}")]
    Generic(String),
}

// Convenient conversion from standard String allocations to our unified Error enum
impl From<String> for MiniVmError {
    fn from(msg: String) -> Self {
        MiniVmError::Generic(msg)
    }
}

impl From<&str> for MiniVmError {
    fn from(msg: &str) -> Self {
        MiniVmError::Generic(msg.to_string())
    }
}