use crate::assembler::parser::{AssembledProgram, ParsedInstruction, Operand};
use crate::bytecode::write_header;
use std::collections::HashMap;

pub struct Encoder {
    bytecode: Vec<u8>,
}

impl Encoder {
    pub fn new() -> Self {
        Self {
            bytecode: Vec::new(),
        }
    }

    pub fn encode(&mut self, program: &AssembledProgram) -> Result<Vec<u8>, String> {
        self.bytecode.clear();


        let mut instruction_offsets = Vec::with_capacity(program.instructions.len());
        let mut label_addresses = HashMap::new();
        let mut current_offset = 0;

        for (idx, inst) in program.instructions.iter().enumerate() {
            instruction_offsets.push(current_offset);

            for (&label_name, &target_inst_idx) in &program.labels {
                if target_inst_idx == idx {
                    label_addresses.insert(label_name, current_offset);
                }
            }

            current_offset += self.calculate_instruction_size(inst)?;
        }

        for (&label_name, &target_inst_idx) in &program.labels {
            if target_inst_idx == program.instructions.len() {
                label_addresses.insert(label_name, current_offset);
            }
        }

        for (idx, inst) in program.instructions.iter().enumerate() {
            let inst_offset = instruction_offsets[idx];
            self.encode_instruction(inst, inst_offset, &label_addresses)?;
        }

        // Prepend the standard binary header expected by consumers/tests
        let mut out = Vec::new();
        write_header(&mut out);
        out.extend_from_slice(&self.bytecode);

        Ok(out)
    }

    fn calculate_instruction_size(&self, inst: &ParsedInstruction) -> Result<usize, String> {
        let base_size = 1; 

        let operands_size = match inst.opcode.to_lowercase().as_str() {
            "halt" | "nop" | "add" | "sub" | "mul" | "div" | "ret" => 0,

            "pop" | "syscall" => 1,

            "jmp" | "jz" | "jnz" => 8,

            "push" | "load" | "store" => 1 + 8,

            _ => {
                return Err(format!(
                    "Unknown opcode '{}' at line {}, col {}",
                    inst.opcode, inst.loc.line, inst.loc.column
                ));
            }
        };

        Ok(base_size + operands_size)
    }

    fn encode_instruction(
        &mut self, 
        inst: &ParsedInstruction, 
        current_offset: usize,
        label_addresses: &HashMap<&str, usize>
    ) -> Result<(), String> {
        match inst.opcode.to_lowercase().as_str() {
            "nop"  => self.bytecode.push(0x00),
            "halt" => self.bytecode.push(0xFF),
            
            "add"  => self.bytecode.push(0x10),
            "sub"  => self.bytecode.push(0x11),
            "mul"  => self.bytecode.push(0x12),
            "div"  => self.bytecode.push(0x13),
            "ret"  => self.bytecode.push(0x14),

            "pop" => {
                self.bytecode.push(0x20);
                self.encode_register(&inst.operands[0], inst)?;
            }

            "push" => {
                self.bytecode.push(0x21);
                self.encode_register(&inst.operands[0], inst)?;
                self.encode_immediate(&inst.operands[1], inst)?;
            }

            "jmp" => {
                self.bytecode.push(0x30);
                self.encode_jump_target(&inst.operands[0], current_offset, label_addresses, inst)?;
            }
            "jz" => {
                self.bytecode.push(0x31);
                self.encode_jump_target(&inst.operands[0], current_offset, label_addresses, inst)?;
            }
            "jnz" => {
                self.bytecode.push(0x32);
                self.encode_jump_target(&inst.operands[0], current_offset, label_addresses, inst)?;
            }

            "load" => {
                self.bytecode.push(0x40);
                self.encode_register(&inst.operands[0], inst)?;
                self.encode_address(&inst.operands[1], inst)?;
            }
            "store" => {
                self.bytecode.push(0x41);
                self.encode_register(&inst.operands[0], inst)?;
                self.encode_address(&inst.operands[1], inst)?;
            }

            "syscall" => {
                self.bytecode.push(0x50);
                self.encode_trap_code(&inst.operands[0], inst)?;
            }

            _ => {
                return Err(format!(
                    "Unknown opcode '{}' at line {}, col {}",
                    inst.opcode, inst.loc.line, inst.loc.column
                ));
            }
        }

        Ok(())
    }


    fn encode_register(&mut self, operand: &Operand, inst: &ParsedInstruction) -> Result<(), String> {
        if let Operand::Register(reg_str) = operand {
            let reg_id = if *reg_str == "sp" {
                254
            } else if *reg_str == "pc" {
                255
            } else if reg_str.starts_with('r') {
                reg_str[1..].parse::<u8>().map_err(|_| {
                    format!("Invalid register identifier '{}' in instruction at line {}", reg_str, inst.loc.line)
                })?
            } else {
                return Err(format!("Expected valid register, got '{}' at line {}", reg_str, inst.loc.line));
            };

            self.bytecode.push(reg_id);
            Ok(())
        } else {
            Err(format!("Expected register operand for opcode '{}' at line {}", inst.opcode, inst.loc.line))
        }
    }

    fn encode_immediate(&mut self, operand: &Operand, inst: &ParsedInstruction) -> Result<(), String> {
        if let Operand::Integer(val) = operand {
            let bytes = val.to_le_bytes();
            self.bytecode.extend_from_slice(&bytes);
            Ok(())
        } else {
            Err(format!("Expected integer literal operand for opcode '{}' at line {}", inst.opcode, inst.loc.line))
        }
    }

    fn encode_address(&mut self, operand: &Operand, inst: &ParsedInstruction) -> Result<(), String> {
        match operand {
            Operand::Integer(val) => {
                if *val < 0 {
                    return Err(format!(
                        "Expected non-negative memory address for opcode '{}' at line {}",
                        inst.opcode, inst.loc.line
                    ));
                }
                self.bytecode.extend_from_slice(&(*val as u64).to_le_bytes());
                Ok(())
            }
            _ => Err(format!(
                "Expected address literal for opcode '{}' at line {}",
                inst.opcode, inst.loc.line
            )),
        }
    }

    fn encode_trap_code(&mut self, operand: &Operand, inst: &ParsedInstruction) -> Result<(), String> {
        match operand {
            Operand::Integer(val) => {
                if *val < 0 || *val > u8::MAX as i64 {
                    return Err(format!(
                        "Trap code must fit in a byte for opcode '{}' at line {}",
                        inst.opcode, inst.loc.line
                    ));
                }
                self.bytecode.push(*val as u8);
                Ok(())
            }
            _ => Err(format!(
                "Expected trap code literal for opcode '{}' at line {}",
                inst.opcode, inst.loc.line
            )),
        }
    }

    fn encode_jump_target(
        &mut self,
        operand: &Operand,
        _current_offset: usize,
        label_addresses: &HashMap<&str, usize>,
        inst: &ParsedInstruction
    ) -> Result<(), String> {
        let target_absolute_address = match operand {
            Operand::Integer(val) => *val as u64,
            Operand::LabelRef(label_name) => {
                let &target_offset = label_addresses.get(label_name).ok_or_else(|| {
                    format!("Undeclared label reference '{}' found at line {}", label_name, inst.loc.line)
                })?;
                
                target_offset as u64
            }
            _ => return Err(format!("Invalid jump target for opcode '{}' at line {}", inst.opcode, inst.loc.line)),
        };

        self.bytecode.extend_from_slice(&target_absolute_address.to_le_bytes());
        Ok(())
    }
}