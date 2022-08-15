// Imports
pub mod create_memory;
pub mod instructions;
use create_memory::*;
use instructions::*;
use std::collections::HashMap;
use std::process;

// The CPU class for virtual machine.
pub struct CPU {
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
    pub fn new(memory: ArrayBuffer) -> Self {
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

    // Read the value of a register memory
    // TODO DEBUG - remove `pub`
    pub fn get_register(&self, name: &str) -> u16 {
        // Check if register exists
        if !self.registers_map.contains_key(name) {
            panic!("Register {} not found", name);
        }

        // Read offset in memory
        let offset = self.registers_map.get(name).unwrap();

        // Read bytes from memory
        let memory = [
            self.registers.buffer[*offset],
            self.registers.buffer[*offset + 1],
        ];

        // Read from memory
        u16::from_be_bytes(memory)
    }

    // Write the value of a register memory
    fn set_register(&mut self, name: &str, value: u16) {
        // Check if register exists
        if !self.registers_map.contains_key(name) {
            panic!("Register {} not found", name);
        }

        // Read offset in memory
        let offset = self.registers_map.get(name).unwrap();

        // Split u16 to [u8; 2]
        let bytes = value.to_be_bytes();

        // Write to memory
        for (index, byte) in bytes.iter().enumerate() {
            self.registers.buffer[*offset + index] = *byte;
        }
    }

    // Read instruction from memory
    fn fetch(&mut self) -> u8 {
        // Read instruction pointer
        let ip = self.get_register("ip");

        // Read instruction from memory
        let instruction = self.memory.buffer[ip as usize];

        // Increment instruction pointer
        self.set_register("ip", ip + 1);

        // Return instruction
        instruction
    }

    // Read instruction from memory
    fn fetch16(&mut self) -> u16 {
        // Read instruction pointer
        let ip = self.get_register("ip");

        // Read instruction from memory
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
            // Move literal to register
            MOV_LIT_REG => {
                // Read data from memory
                let literal = self.fetch16();
                let register = (self.fetch() as usize % self.registers_names.len()) * 2;

                // Split u16
                let value: [u8; 2] = literal.to_be_bytes();

                // Write to register memory
                self.registers.buffer[register] = value[0];
                self.registers.buffer[register + 1] = value[1];

                return;
            }

            // Move register to register
            MOV_REG_REG => {
                // Read data from memory
                let register_from = (self.fetch() as usize % self.registers_names.len()) * 2;
                let register_to = (self.fetch() as usize % self.registers_names.len()) * 2;

                // Read from_register memory
                let value = [
                    self.registers.buffer[register_from],
                    self.registers.buffer[register_from + 1],
                ];

                // Write to_register memory
                self.registers.buffer[register_to] = value[0];
                self.registers.buffer[register_to + 1] = value[1];

                return;
            }

            // Move register to memory
            MOV_REG_MEM => {
                // Read data from memory
                let register_from = (self.fetch() as usize % self.registers_names.len()) * 2;
                let address = self.fetch16() as usize;

                // Read from_register memory
                let value = [
                    self.registers.buffer[register_from],
                    self.registers.buffer[register_from + 1],
                ];

                // Write memory
                self.memory.buffer[address] = value[0];
                self.memory.buffer[address + 1] = value[1];

                return;
            }

            // Move memory to register
            MOV_MEM_REG => {
                // Read data from memory
                let address = self.fetch16() as usize;
                let register_to = (self.fetch() as usize % self.registers_names.len()) * 2;

                // Read from memory
                let value = [self.memory.buffer[address], self.memory.buffer[address + 1]];

                // Write to register
                self.registers.buffer[register_to] = value[0];
                self.registers.buffer[register_to + 1] = value[1];

                return;
            }

            // Jump if not equal
            JMP_NOT_EQ => {
                // Read data from memory
                let value = self.fetch16();
                let address = self.fetch16();

                // Perform jump if not equal
                if value != self.get_register("acc") {
                    self.set_register("ip", address);
                }

                return;
            }

            // Add register to register
            ADD_REG_REG => {
                // Read data from memory
                let r1_offset = self.fetch() * 2;
                let r2_offset = self.fetch() * 2;

                // Read registers memory
                let r1_memory = [
                    self.registers.buffer[r1_offset as usize],
                    self.registers.buffer[r1_offset as usize + 1],
                ];
                let r2_memory = [
                    self.registers.buffer[r2_offset as usize],
                    self.registers.buffer[r2_offset as usize + 1],
                ];

                // Convert to u16
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
    pub fn step(&mut self) {
        // Read instruction from memory
        let instruction = self.fetch();
        self.execute(instruction);
    }

    pub fn debug(&self) {
        for (_, name) in self.registers_names.iter().enumerate() {
            println!("{}: 0x{:02X}", name, self.get_register(name));
        }
        println!("");
    }

    pub fn view_memory(&self, address: u16, size: usize) {
        print!("0x{:04X}: ", address);
        for i in 0..size {
            print!("0x{:02X} ", self.memory.buffer[address as usize + i]);
        }
        println!("");
    }
}
