mod create_memory;
use create_memory::{create_memory, ArrayBuffer};
use std::collections::HashMap;

// The CPU class for virtual machine.
struct CPU {
    // Memory
    memory: ArrayBuffer,

    // Registers
    registers_names: Vec<String>,
    registers: ArrayBuffer,
    registers_map: HashMap<String, usize>,
}

// Logic for the CPU class
impl CPU {
    // Construct a new CPU instance
    fn new(memory: ArrayBuffer) -> Self {
        // Names fort all registers
        let registers_names = vec![
            String::from("ip"),  // Instruction pointer
            String::from("acc"), // Accumulator (math operations result)
            String::from("r1"),  // General purpose register
            String::from("r2"),  // General purpose register
            String::from("r3"),  // General purpose register
            String::from("r4"),  // General purpose register
            String::from("r5"),  // General purpose register
            String::from("r6"),  // General purpose register
            String::from("r7"),  // General purpose register
            String::from("r8"),  // General purpose register
        ];

        // Registers memory
        let registers = create_memory(registers_names.len() * 2);

        // Map the registers names
        let mut registers_map = HashMap::new();
        for (index, name) in registers_names.iter().enumerate() {
            registers_map.insert(name.clone(), index * 2);
        }

        // Return new CPU
        Self {
            memory,
            registers_names,
            registers,
            registers_map,
        }
    }
}
