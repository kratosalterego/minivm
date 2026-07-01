pub const MV_MAGIC: [u8; 4] = [0x00, 0x6D, 0x56, 0x4D];

pub const MV_VERSION_MAJOR: u8 = 1;
pub const MV_VERSION_MINOR: u8 = 0;

pub const OP_NOP: u8    = 0x00;
pub const OP_HALT: u8   = 0xFF;

pub const OP_ADD: u8    = 0x10;
pub const OP_SUB: u8    = 0x11;
pub const OP_MUL: u8    = 0x12;
pub const OP_DIV: u8    = 0x13;
pub const OP_RET: u8    = 0x14;

pub const OP_POP: u8    = 0x20;
pub const OP_PUSH: u8   = 0x21;

pub const OP_JMP: u8    = 0x30;
pub const OP_JZ: u8     = 0x31;
pub const OP_JNZ: u8    = 0x32;

pub const OP_LOAD: u8   = 0x40;
pub const OP_STORE: u8  = 0x41;

pub const OP_SYSCALL: u8 = 0x50;


pub fn write_header(buffer: &mut Vec<u8>) {
    buffer.extend_from_slice(&MV_MAGIC);
    buffer.push(MV_VERSION_MAJOR);
    buffer.push(MV_VERSION_MINOR);
}

pub fn verify_header(bytes: &[u8]) -> Result<usize, String> {
    if bytes.len() < 6 {
        return Err("Malformed binary: File data is too short to contain a valid header prefix.".to_string());
    }

    if bytes[0..4] != MV_MAGIC {
        return Err("Validation Error: Invalid file magic prefix. Not a valid miniVM compiled asset.".to_string());
    }

    if bytes[4] != MV_VERSION_MAJOR {
        return Err(format!(
            "Compatibility Error: Unsupported binary layout major version (Expected v{}, found v{})",
            MV_VERSION_MAJOR, bytes[4]
        ));
    }

    Ok(6)
}