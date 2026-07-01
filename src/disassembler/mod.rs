pub mod printer;

use crate::error::Result;
use crate::vm::decoder::Decoder;
use crate::isa::Instruction; 

pub struct Disassembler<'a> {
    bytecode: &'a [u8],
}

impl<'a> Disassembler<'a> {
    pub fn new(bytecode: &'a [u8]) -> Self {
        Self { bytecode }
    }

    pub fn disassemble(&self) -> Result<String> {
        let mut output = String::new();
        let mut decoder = Decoder::new(self.bytecode);
        
        output.push_str("; --- Disassembled Bytecode Stream ---\n\n");

        while !decoder.is_at_end() {
            let current_offset = decoder.current_offset();

            match decoder.decode_next() {
                Ok(Some(instruction)) => {
                    let rendered_instruction = printer::format_instruction(&instruction);
                    
                    output.push_str(&format!("0x{:04X}:  {}\n", current_offset, rendered_instruction));
                    
                    if matches!(instruction, Instruction::Halt) {
                        break;
                    }
                }
                Ok(None) => break, 
                Err(decode_err) => {
                    output.push_str(&format!(
                        "\n; !!! DECODE ERROR at offset 0x{:04X}: {} !!!\n",
                        current_offset, decode_err
                    ));
                    return Err(format!("Disassembly halted prematurely: {}", decode_err).into());
                }
            }
        }

        Ok(output)
    }
}

pub fn disassemble_bytes(bytecode: &[u8]) -> Result<String> {
    let disasm = Disassembler::new(bytecode);
    disasm.disassemble()
}