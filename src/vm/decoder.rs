use crate::error::Result;
use crate::isa::Instruction;
use crate::util::endian::{read_i64_le, read_u64_le};

pub struct Decoder<'a> {
    bytecode: &'a [u8],
    cursor: usize,
}

impl<'a> Decoder<'a> {
    pub fn new(bytecode: &'a [u8]) -> Self {
        Self {
            bytecode,
            cursor: 0,
        }
    }

    pub fn current_offset(&self) -> usize {
        self.cursor
    }

    pub fn set_offset(&mut self, offset: usize) {
        self.cursor = offset;
    }

    pub fn is_at_end(&self) -> bool {
        self.cursor >= self.bytecode.len()
    }

    pub fn decode_next(&mut self) -> Result<Option<Instruction>> {
        if self.is_at_end() {
            return Ok(None);
        }

        let opcode = self.bytecode[self.cursor];
        self.cursor += 1;

        let inst = match opcode {
            0x00 => Instruction::Nop,
            0xFF => Instruction::Halt,
            
            0x10 => Instruction::Add,
            0x11 => Instruction::Sub,
            0x12 => Instruction::Mul,
            0x13 => Instruction::Div,
            0x14 => Instruction::Ret,

            0x20 => {
                let reg = self.read_u8()?;
                Instruction::Pop(reg)
            }

            0x21 => {
                let reg = self.read_u8()?;
                let immediate = self.read_i64()?;
                Instruction::Push(reg, immediate)
            }

            0x30 => {
                let target = self.read_u64()?;
                Instruction::Jmp(target)
            }
            0x31 => {
                let target = self.read_u64()?;
                Instruction::Jz(target)
            }
            0x32 => {
                let target = self.read_u64()?;
                Instruction::Jnz(target)
            }

            0x40 => {
                let reg = self.read_u8()?;
                let addr = self.read_u64()?;
                Instruction::Load(reg, addr)
            }
            0x41 => {
                let reg = self.read_u8()?;
                let addr = self.read_u64()?;
                Instruction::Store(reg, addr)
            }

            0x50 => {
                let trap_code = self.read_u8()?;
                Instruction::Syscall(trap_code)
            }

            unknown => {
                return Err(format!(
                    "Decoding Error: Encountered undefined opcode '0x{:02X}' at address 0x{:04X}",
                    unknown, self.cursor - 1
                ).into());
            }
        };

        Ok(Some(inst))
    }


    fn read_u8(&mut self) -> Result<u8> {
        if self.cursor >= self.bytecode.len() {
            return Err("Decoding Error: Unexpected EOF while fetching u8 operand.".into());
        }
        let val = self.bytecode[self.cursor];
        self.cursor += 1;
        Ok(val)
    }

    fn read_i64(&mut self) -> Result<i64> {
        if self.cursor + 8 > self.bytecode.len() {
            return Err("Decoding Error: Unexpected EOF while fetching i64 operand.".into());
        }
        let val = read_i64_le(self.bytecode, self.cursor);
        self.cursor += 8;
        Ok(val)
    }

    fn read_u64(&mut self) -> Result<u64> {
        if self.cursor + 8 > self.bytecode.len() {
            return Err("Decoding Error: Unexpected EOF while fetching u64 operand.".into());
        }
        let val = read_u64_le(self.bytecode, self.cursor);
        self.cursor += 8;
        Ok(val)
    }
}