use std::ops::Index;
use std::ops::IndexMut;

// TODO: make internals private
pub struct Memory {
    memory: [u8; 0x1000],
}

impl Memory {
    pub fn new() -> Self {
        Memory { memory: [0; 0x1000] }
    }

    // TODO: Figure out if this is a slice.
    pub fn slice(&self, start: &u16, length: &u16) -> &[u8] {
        &self.memory[(*start as usize)..((*start + *length) as usize)]
    }
}

impl Index<u16> for Memory {
    type Output = u8;
    fn index(&self, index: u16) -> &u8 {
        &self.memory[index as usize]
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut u8 {
        &mut self.memory[index as usize]
    }
}
