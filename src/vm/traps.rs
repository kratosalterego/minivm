use crate::error::Result;
use crate::vm::cpu::Cpu;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrapCode {
    PrintInt   = 0x01,  
    PrintChar  = 0x02, 
    ReadInt    = 0x03,  
    DumpRegs   = 0x04,  
}

impl TrapCode {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x01 => Some(TrapCode::PrintInt),
            0x02 => Some(TrapCode::PrintChar),
            0x03 => Some(TrapCode::ReadInt),
            0x04 => Some(TrapCode::DumpRegs),
            _    => None,
        }
    }
}

pub fn handle_trap(trap_byte: u8, cpu: &mut Cpu) -> Result<()> {
    let trap = TrapCode::from_u8(trap_byte).ok_or_else(|| {
        format!(
            "Runtime Exception: Triggered undefined or malformed system trap vector '0x{:02X}' at PC address 0x{:04X}",
            trap_byte, cpu.pc
        )
    })?;

    match trap {
        TrapCode::PrintInt => {
            let val = cpu.stack.pop()?;
            print!("{}", val);
            io::stdout().flush().map_err(|e| format!("I/O Error: {}", e))?;
        }

        TrapCode::PrintChar => {
            let val = cpu.stack.pop()?;
            if let Some(c) = char::from_u32(val as u32) {
                print!("{}", c);
                io::stdout().flush().map_err(|e| format!("I/O Error: {}", e))?;
            } else {
                return Err(format!("Runtime Exception: Invalid Unicode character scalar conversion attempt for value '{}'", val).into());
            }
        }

        TrapCode::ReadInt => {
            let mut input = String::new();
            io::stdin().read_line(&mut input).map_err(|e| format!("I/O Error: {}", e))?;
            let parsed_val: i64 = input.trim().parse().map_err(|_| {
                "Runtime Exception: Invalid integer literal provided on standard input stream."
            })?;
            cpu.stack.push(parsed_val)?;
        }

        TrapCode::DumpRegs => {
            println!("\n--- [CPU REGISTER DIAGNOSTIC DUMP] ---");
            for (i, reg_val) in cpu.registers.iter().enumerate() {
                print!("r{:<2}: 0x{:016X} ({:<10})  ", i, reg_val, reg_val);
                if (i + 1) % 2 == 0 {
                    println!();
                }
            }
            println!("pc : 0x{:04X}               sp : {}", cpu.pc, cpu.stack.len());
            println!("---------------------------------------\n");
        }
    }

    Ok(())
}