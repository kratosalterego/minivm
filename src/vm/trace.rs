use crate::isa::Instruction;
use crate::vm::cpu::Cpu;

pub fn log_state(cpu: &Cpu, current_instruction: &Instruction, current_pc: usize) {
    let inst_str = format_instruction_brief(current_instruction);

    let mut stack_str = String::new();
    stack_str.push('[');
    
    let stack_len = cpu.stack.len();
    let preview_depth = std::cmp::min(stack_len, 4);
    
    for i in (0..preview_depth).rev() {
        if let Ok(val) = cpu.stack.peek_at_depth(i) {
            stack_str.push_str(&val.to_string());
            if i > 0 {
                stack_str.push_str(", ");
            }
        }
    }
    
    if stack_len > 4 {
        stack_str.push_str(", ...");
    }
    stack_str.push(']');

    println!(
        "\x1b[90m0x{:04X}:\x1b[0m  {:<18} | \x1b[36mr0:\x1b[0m{:<5} \x1b[36mr1:\x1b[0m{:<5} | \x1b[32mStack:\x1b[0m{}",
        current_pc,
        inst_str,
        cpu.registers[0],
        cpu.registers[1],
        stack_str
    );
}

fn format_instruction_brief(inst: &Instruction) -> String {
    match inst {
        Instruction::Nop => "nop".to_string(),
        Instruction::Halt => "halt".to_string(),
        Instruction::Add => "add".to_string(),
        Instruction::Sub => "sub".to_string(),
        Instruction::Mul => "mul".to_string(),
        Instruction::Div => "div".to_string(),
        Instruction::Ret => "ret".to_string(),
        Instruction::Pop(reg) => format!("pop  r{}", reg),
        Instruction::Push(reg, val) => format!("push r{}, {}", reg, val),
        Instruction::Load(reg, addr) => format!("load r{}, 0x{:X}", reg, addr),
        Instruction::Store(reg, addr) => format!("store r{}, 0x{:X}", reg, addr),
        Instruction::Jmp(target) => format!("jmp  0x{:04X}", target),
        Instruction::Jz(target) => format!("jz   0x{:04X}", target),
        Instruction::Jnz(target) => format!("jnz  0x{:04X}", target),
        Instruction::Syscall(code) => format!("syscall 0x{:02X}", code),
    }
}