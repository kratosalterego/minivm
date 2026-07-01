use crate::error::Result;

#[derive(Debug)]
pub struct Globals {
    memory: Vec<i64>,
}

impl Globals {
    pub fn new(size: usize) -> Self {
        Self {
            memory: vec![0; size],
        }
    }

    pub fn clear(&mut self) {
        for val in self.memory.iter_mut() {
            *val = 0;
        }
    }

    pub fn read(&self, address: usize) -> Result<i64> {
        self.memory.get(address).cloned().ok_or_else(|| {
            format!(
                "Runtime Error: Out-of-bounds global memory read attempt at address 0x{:04X} (Max allocation size: {} slots)",
                address, self.memory.len()
            )
            .into()
        })
    }

    pub fn write(&mut self, address: usize, value: i64) -> Result<()> {
        if let Some(slot) = self.memory.get_mut(address) {
            *slot = value;
            Ok(())
        } else {
            Err(format!(
                "Runtime Error: Out-of-bounds global memory write attempt at address 0x{:04X} (Max allocation size: {} slots)",
                address, self.memory.len()
            )
            .into())
        }
    }

    pub fn len(&self) -> usize {
        self.memory.len()
    }
}