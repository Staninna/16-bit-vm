pub struct ByteArray {
    buffer: Vec<u8>,
}

impl ByteArray {
    pub fn new(length: usize) -> Self {
        Self {
            buffer: vec![0x00; length],
        }
    }

    pub fn set_byte(&mut self, data: u8, index: usize) {
        self.buffer[index] = data;
    }

    pub fn get_byte(&self, index: usize) -> u8 {
        self.buffer[index]
    }
}

pub fn create_memory(length: usize) -> ByteArray {
    let memory = ByteArray::new(length);
    memory
}
