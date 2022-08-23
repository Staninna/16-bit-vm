// Memory class for memory management
#[derive(Debug)]
pub struct Memory {
    buffer: Vec<u8>,
    pub length: u16,
}

// Logic for the Memory
impl Memory {
    // Create a new Memory
    pub fn new(length: usize) -> Self {
        Self {
            buffer: vec![0x00; length],
            length: length as u16,
        }
    }

    // Change a byte to the Memory
    pub fn set_byte(&mut self, data: u8, address: usize) {
        self.buffer[address] = data;
    }

    // Get a byte from the Memory
    pub fn get_byte(&self, address: usize) -> u8 {
        self.buffer[address]
    }
}

// Create a new memory for the virtual machine
pub fn create_memory(length: usize) -> Memory {
    Memory::new(length)
}
