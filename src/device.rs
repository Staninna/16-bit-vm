// Memory class
pub struct Memory {
    buffer: Vec<u8>,
    pub length: u16,
}

// Memory implementation
impl Memory {
    pub fn new(length: usize) -> Self {
        Self {
            buffer: vec![0x00; length],
            length: length as u16,
        }
    }

    // Write a byte
    pub fn set_byte(&mut self, data: u8, address: usize) {
        self.buffer[address] = data;
    }

    // Read a byte
    pub fn get_byte(&self, address: usize) -> u8 {
        self.buffer[address]
    }
}
