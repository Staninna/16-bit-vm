// Imports
mod create_memory;
mod instructions;
use create_memory::*;
use instructions::*;
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

    // Get the value of a register memory
    fn get_register(&self, name: &str) -> u16 {
        // Check if register exists
        if !self.registers_map.contains_key(name) {
            panic!("Register {} not found", name);
        }

        // Get offset in memory
        let offset = self.registers_map.get(name).unwrap();

        // Get bytes from memory
        let memory = [
            self.registers.buffer[*offset],
            self.registers.buffer[*offset + 1],
        ];

        // Read from memory
        u16::from_be_bytes(memory)
    }

    // Set the value of a register memory
    fn set_register(&mut self, name: &str, value: u16) {
        // Check if register exists
        if !self.registers_map.contains_key(name) {
            panic!("Register {} not found", name);
        }

        // Get offset in memory
        let offset = self.registers_map.get(name).unwrap();

        // Split u16 to [u8; 2]
        let bytes = value.to_be_bytes();

        // Write to memory
        for (index, byte) in bytes.iter().enumerate() {
            self.registers.buffer[*offset + index] = *byte;
        }
    }

    // Get instruction from memory
    fn fetch(&mut self) -> u8 {
        // Get instruction pointer
        let ip = self.get_register("ip");

        // Get instruction from memory
        let instruction = self.memory.buffer[ip as usize];

        // Increment instruction pointer
        self.set_register("ip", ip + 1);

        // Return instruction
        instruction
    }

    // Get instruction from memory
    fn fetch16(&mut self) -> u16 {
        // Get instruction pointer
        let ip = self.get_register("ip");

        // Get instruction from memory
        let instruction = [
            self.memory.buffer[ip as usize],
            self.memory.buffer[ip as usize + 1],
        ];

        // Increment instruction pointer
        self.set_register("ip", ip + 2);

        // Return instruction
        u16::from_be_bytes(instruction)
    }

    // Execute an instruction
    fn execute(&mut self, instruction: u8) {
        match instruction {
            MOV_LIT_R1 => {
                let literal = self.fetch16();
                self.set_register("r1", literal);
                return;
            }

            MOV_LIT_R2 => {
                let literal = self.fetch16();
                self.set_register("r2", literal);
                return;
            }

            ADD_REG_REG => {
                // Get offsets
                let r1_offset = self.fetch() * 2;
                let r2_offset = self.fetch() * 2;

                // Get bytes from memory
                let r1_memory = [
                    self.registers.buffer[r1_offset as usize],
                    self.registers.buffer[r1_offset as usize + 1],
                ];
                let r2_memory = [
                    self.registers.buffer[r2_offset as usize],
                    self.registers.buffer[r2_offset as usize + 1],
                ];

                // Read from memory
                let value_r1 = u16::from_be_bytes(r1_memory);
                let value_r2 = u16::from_be_bytes(r2_memory);

                // Add values
                self.set_register("acc", value_r1 + value_r2);

                return;
            }

            _ => {
                panic!("Instruction: {} not found", instruction);
            }
        }
    }

    // Run step trough the virtual machine
    fn step(&mut self) {
        // Get instruction from memory
        let instruction = self.fetch();
        self.execute(instruction);
    }

    fn debug(&self) {
        for (_, name) in self.registers_names.iter().enumerate() {
            println!("{}: {}", name, self.get_register(name));
        }
        println!("");
    }

    fn view_memory(&self, address: u16, size: usize) {
        print!("0x{:04X}: ", address);
        for i in 0..size {
            print!("0x{:02X} ", self.memory.buffer[address as usize + i]);
        }
        println!("");
    }
}
