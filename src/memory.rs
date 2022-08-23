// Memory class for memory management
pub struct Memory {
    buffer: Vec<u8>,
}

// Logic for the Memory
impl Memory {
    // Create a new Memory
    pub fn new(length: usize) -> Self {
        Self {
            buffer: vec![0x00; length],
        }
    }

    // Change a byte to the Memory
    pub fn set_byte(&mut self, data: u8, index: usize) {
        self.buffer[index] = data;
    }

    // Get a byte from the Memory
    pub fn get_byte(&self, index: usize) -> u8 {
        self.buffer[index]
    }
}

// Create a new memory for the virtual machine
pub fn create_memory(length: usize) -> Memory {
    Memory::new(length)
}
