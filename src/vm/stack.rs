use crate::error::Result;

pub struct Stack {
    data: Vec<i64>,
    capacity: usize,
}

impl Stack {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            capacity,
        }
    }
    
    pub fn peek_at_depth(&self, depth: usize) -> Result<i64> {
        if depth >= self.data.len() {
            return Err("Bounds Error".into());
        }
        Ok(self.data[self.data.len() - 1 - depth])
    }

    #[inline(always)]
    pub fn push(&mut self, value: i64) -> Result<()> {
        if self.data.len() >= self.capacity {
            return Err(format!(
                "Runtime Error: Stack Overflow. Attempted to exceed maximum bounds capacity threshold of {} elements.",
                self.capacity
            ).into());
        }
        self.data.push(value);
        Ok(())
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Result<i64> {
        self.data.pop().ok_or_else(|| {
            "Runtime Error: Stack Underflow. Attempted to pop an evaluation element from an empty stack frame."
                .into()
        })
    }

    #[inline(always)]
    pub fn peek(&self) -> Result<i64> {
        self.data.last().cloned().ok_or_else(|| {
            "Runtime Error: Stack Underflow. Attempted to peek at an element on an empty stack frame."
                .into()
        })
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}