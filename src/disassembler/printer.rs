use crate::isa::Instruction; 

pub fn format_instruction(inst: &Instruction) -> String {
    match inst {
        Instruction::Nop => "nop".to_string(),
        Instruction::Halt => "halt".to_string(),
        Instruction::Add => "add".to_string(),
        Instruction::Sub => "sub".to_string(),
        Instruction::Mul => "mul".to_string(),
        Instruction::Div => "div".to_string(),
        Instruction::Ret => "ret".to_string(),

        Instruction::Pop(reg) => {
            format!("pop  {}", format_register(*reg))
        }

        Instruction::Push(reg, value) => {
            format!("push {}, {}", format_register(*reg), value)
        }
        Instruction::Load(reg, address) => {
            format!("load {}, 0x{:X}", format_register(*reg), address)
        }
        Instruction::Store(reg, address) => {
            format!("store {}, 0x{:X}", format_register(*reg), address)
        }

        Instruction::Jmp(target) => {
            format!("jmp  0x{:04X}", target)
        }
        Instruction::Jz(target) => {
            format!("jz   0x{:04X}", target)
        }
        Instruction::Jnz(target) => {
            format!("jnz  0x{:04X}", target)
        }
    }
}

fn format_register(reg_id: u8) -> String {
    match reg_id {
        254 => "sp".to_string(), 
        255 => "pc".to_string(),  
        id  => format!("r{}", id),
    }
}