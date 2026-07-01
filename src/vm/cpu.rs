use crate::error::Result;
use crate::isa::Instruction;
use crate::vm::decoder::Decoder;
use crate::vm::stack::Stack;
use crate::vm::globals::Globals;

pub const NUM_REGISTERS: usize = 16;

pub struct Cpu {
    pub registers: [i64; NUM_REGISTERS],
    pub pc: usize,          
    pub sp: usize,         
    pub running: bool,
    pub stack: Stack,
    pub globals: Globals,
}

impl Cpu {
    pub fn new(stack_size: usize, globals_size: usize) -> Self {
        Self {
            registers: [0; NUM_REGISTERS],
            pc: 0,
            sp: 0,
            running: false,
            stack: Stack::new(stack_size),
            globals: Globals::new(globals_size),
        }
    }

    pub fn reset(&mut self) {
        self.registers = [0; NUM_REGISTERS];
        self.pc = 0;
        self.sp = 0;
        self.running = false;
        self.stack.clear();
        self.globals.clear();
    }

    pub fn run(&mut self, bytecode: &[u8]) -> Result<()> {
        self.running = true;
        self.pc = 0;

        let mut decoder = Decoder::new(bytecode);

        while self.running && self.pc < bytecode.len() {
            decoder.set_offset(self.pc);
            
            let current_instruction = match decoder.decode_next()? {
                Some(inst) => inst,
                None => {
                    self.running = false;
                    break;
                }
            };

            let next_pc = decoder.current_offset();

            self.execute(current_instruction, next_pc)?;
        }

        Ok(())
    }

    fn execute(&mut self, inst: Instruction, next_pc: usize) -> Result<()> {
        self.pc = next_pc;

        match inst {
            Instruction::Nop => {}
            
            Instruction::Halt => {
                self.running = false;
            }

            Instruction::Push(reg, immediate) => {
                self.set_register_val(reg, immediate)?;
                self.stack.push(immediate)?;
                self.sp = self.stack.len();
            }

            Instruction::Pop(reg) => {
                let val = self.stack.pop()?;
                self.set_register_val(reg, val)?;
                self.sp = self.stack.len();
            }

            Instruction::Add => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                self.stack.push(a.wrapping_add(b))?;
            }

            Instruction::Sub => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                self.stack.push(a.wrapping_sub(b))?;
            }

            Instruction::Mul => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                self.stack.push(a.wrapping_mul(b))?;
            }

            Instruction::Div => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                if b == 0 {
                    return Err("Runtime Error: Division by zero trap tripped.".into());
                }
                self.stack.push(a / b)?;
            }

            Instruction::Jmp(target_address) => {
                self.pc = target_address as usize;
            }

            Instruction::Jz(target_address) => {
                let condition = self.stack.pop()?;
                if condition == 0 {
                    self.pc = target_address as usize;
                }
            }

            Instruction::Jnz(target_address) => {
                let condition = self.stack.pop()?;
                if condition != 0 {
                    self.pc = target_address as usize;
                }
            }

            Instruction::Load(reg, addr) => {
                let val = self.globals.read(addr as usize)?;
                self.set_register_val(reg, val)?;
            }

            Instruction::Store(reg, addr) => {
                let val = self.get_register_val(reg)?;
                self.globals.write(addr as usize, val)?;
            }

            Instruction::Ret => {
                let return_address = self.stack.pop()?;
                self.pc = return_address as usize;
            }
        }

        Ok(())
    }

    fn get_register_val(&self, reg_id: u8) -> Result<i64> {
        if (reg_id as usize) < NUM_REGISTERS {
            Ok(self.registers[reg_id as usize])
        } else {
            Err(format!("Runtime Error: Invalid register read target index 'r{}'.", reg_id).into())
        }
    }

    fn set_register_val(&mut self, reg_id: u8, val: i64) -> Result<()> {
        if (reg_id as usize) < NUM_REGISTERS {
            self.registers[reg_id as usize] = val;
            Ok(())
        } else {
            Err(format!("Runtime Error: Invalid register write target index 'r{}'.", reg_id).into())
        }
    }
}