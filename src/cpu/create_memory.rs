pub struct ArrayBuffer {
    pub buffer: Vec<u8>,
}

impl ArrayBuffer {
    pub fn new(length: usize) -> Self {
        Self {
            buffer: vec![0x00; length],
        }
    }
}

pub fn create_memory(length: usize) -> ArrayBuffer {
    let memory = ArrayBuffer::new(length);
    memory
}
