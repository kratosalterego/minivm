pub mod cpu;
pub mod decoder;
pub mod globals;
pub mod stack;
pub mod trace;
pub mod traps;

use crate::error::Result;
use self::cpu::Cpu;

pub struct VmConfig {
    pub stack_size: usize,
    pub globals_size: usize,
    pub enable_tracing: bool,
}

impl Default for VmConfig {
    fn default() -> Self {
        Self {
            stack_size: 1024,  
            globals_size: 256,   
            enable_tracing: false,
        }
    }
}

pub struct Vm {
    config: VmConfig,
    cpu: Cpu,
}

impl Vm {
    pub fn new(config: VmConfig) -> Self {
        let cpu = Cpu::new(config.stack_size, config.globals_size);
        Self { config, cpu }
    }

    pub fn execute(&mut self, bytecode: &[u8]) -> Result<()> {
        if self.config.enable_tracing {
            println!("; --- VM Execution Trace Start ---");
        }

        let execution_result = self.cpu.run(bytecode);

        if self.config.enable_tracing {
            println!("; --- VM Execution Terminated ---");
        }

        execution_result
    }

    pub fn cpu(&self) -> &Cpu {
        &self.cpu
    }

    pub fn cpu_mut(&mut self) -> &mut Cpu {
        &mut self.cpu
    }
}