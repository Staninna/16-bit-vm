// Imports
use crate::memory::Memory;
use crate::memory_mapper::MemoryMapper;
use std::collections::HashMap;

// Define instructions for the CPU
pub const MOV_LIT_REG: u8 = 0x10;
pub const MOV_REG_REG: u8 = 0x11;
pub const MOV_REG_MEM: u8 = 0x12;
pub const MOV_MEM_REG: u8 = 0x13;
pub const ADD_REG_REG: u8 = 0x14;
pub const JMP_NOT_EQ: u8 = 0x15;
pub const PSH_LIT: u8 = 0x16;
pub const PSH_REG: u8 = 0x17;
pub const POP: u8 = 0x18;
pub const CAL_LIT: u8 = 0x19;
pub const CAL_REG: u8 = 0x1a;
pub const RET: u8 = 0x1b;
pub const HLT: u8 = 0x1c;

// The CPU class for virtual machine.
pub struct CPU {
    memory_mapper: MemoryMapper,
    registers_names: Vec<String>,
    registers: Memory,
    registers_map: HashMap<String, usize>,
    stack_frame_size: u16,
}

// Logic for the CPU class
impl CPU {
    // Construct a new CPU instance
    pub fn new(memory_mapper: MemoryMapper) -> Self {
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
            String::from("sp"),  // Stack pointer
            String::from("fp"),  // Frame pointer
        ];

        // Registers memory
        let mut registers = Memory::new(registers_names.len() * 2);

        // Map the registers names
        let mut registers_map = HashMap::new();
        for (index, name) in registers_names.iter().enumerate() {
            registers_map.insert(name.clone(), index * 2);
        }

        // Read offsets of pointers
        let sp_offset = registers_map.get("sp").unwrap();
        let fp_offset = registers_map.get("fp").unwrap();

        // Split u16 to [u8; 2]
        let memory_position = ((0x00FF - 1) as u16).to_be_bytes();

        // Write to registers memory
        registers.set_byte(memory_position[0], *sp_offset);
        registers.set_byte(memory_position[0], *fp_offset);
        registers.set_byte(memory_position[1], *sp_offset + 1);
        registers.set_byte(memory_position[1], *fp_offset + 1);

        let stack_frame_size = 0;

        // Return new CPU
        Self {
            memory_mapper,
            registers_names,
            registers,
            registers_map,
            stack_frame_size,
        }
    }

    // Read the value of a register memory
    fn get_register(&self, name: &str) -> u16 {
        // Check if register exists
        if !self.registers_map.contains_key(name) {
            panic!("Register {} not found", name);
        }

        // Read offset in memory
        let offset = self.registers_map.get(name).unwrap();

        // Read bytes from memory
        let memory = [
            self.registers.get_byte(*offset),
            self.registers.get_byte(*offset + 1),
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
        self.registers.set_byte(bytes[0], *offset);
        self.registers.set_byte(bytes[1], *offset + 1);
    }

    // Read instruction from memory
    fn fetch8(&mut self) -> u8 {
        // Read instruction pointer
        let ip = self.get_register("ip");

        // Read instruction from memory
        let instruction = self.memory_mapper.get_byte(ip);

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
            self.memory_mapper.get_byte(ip),
            self.memory_mapper.get_byte(ip + 1),
        ];

        // Increment instruction pointer
        self.set_register("ip", ip + 2);

        // Return instruction
        u16::from_be_bytes(instruction)
    }

    // Push u16 on the stack
    fn push(&mut self, bytes: [u8; 2]) {
        // Read stack pointer
        let sp_address = self.get_register("sp");

        // Write memory
        self.memory_mapper.set_byte(bytes[0], sp_address);
        self.memory_mapper.set_byte(bytes[1], sp_address + 1);

        // Decrement stack pointer
        self.set_register("sp", sp_address - 2);
        self.stack_frame_size += 2;
    }

    // Pop u16 from the stack
    fn pop(&mut self) -> u16 {
        // Increment stack pointer
        let next_sp_address = self.get_register("sp") + 2;
        self.set_register("sp", next_sp_address);
        self.stack_frame_size -= 2;

        // Read stack memory
        u16::from_be_bytes([
            self.memory_mapper.get_byte(next_sp_address),
            self.memory_mapper.get_byte(next_sp_address + 1),
        ])
    }

    // Push the current CPU state on the stack
    fn push_state(&mut self) {
        // Push register on to the stack
        self.push(self.get_register("r1").to_be_bytes());
        self.push(self.get_register("r2").to_be_bytes());
        self.push(self.get_register("r3").to_be_bytes());
        self.push(self.get_register("r4").to_be_bytes());
        self.push(self.get_register("r5").to_be_bytes());
        self.push(self.get_register("r6").to_be_bytes());
        self.push(self.get_register("r7").to_be_bytes());
        self.push(self.get_register("r8").to_be_bytes());

        // Push return address on stack
        self.push(self.get_register("ip").to_be_bytes());

        // Push stack frame size on to the stack
        self.push((self.stack_frame_size + 2).to_be_bytes());

        // Write new fp and stack frame size
        self.set_register("fp", self.get_register("sp"));
        self.stack_frame_size = 0;
    }

    // Pop the previous CPU state from the stack
    fn pop_state(&mut self) {
        // Read frame pointer
        let frame_pointer_address = self.get_register("fp");

        // Write new stack pointer
        self.set_register("sp", frame_pointer_address);

        // Pop stack frame size
        self.stack_frame_size = self.pop();
        let stack_frame_size = self.stack_frame_size;

        // Pop registers from the stack
        let ip = self.pop();
        let r8 = self.pop();
        let r7 = self.pop();
        let r6 = self.pop();
        let r5 = self.pop();
        let r4 = self.pop();
        let r3 = self.pop();
        let r2 = self.pop();
        let r1 = self.pop();

        // Write registers
        self.set_register("ip", ip);
        self.set_register("r8", r8);
        self.set_register("r7", r7);
        self.set_register("r6", r6);
        self.set_register("r5", r5);
        self.set_register("r4", r4);
        self.set_register("r3", r3);
        self.set_register("r2", r2);
        self.set_register("r1", r1);

        // Pop arguments from subroutine of the stack
        let subroutine_arguments = self.pop();
        for _ in 0..subroutine_arguments {
            self.pop();
        }

        // Reset frame pointer
        self.set_register("fp", frame_pointer_address + stack_frame_size);
    }

    // Get offset of given register
    fn fetch_register_index(&mut self) -> usize {
        (self.fetch8() as usize % self.registers_names.len()) * 2
    }

    // Execute an instruction
    fn execute(&mut self, instruction: u8) {
        match instruction {
            // Move literal to register
            MOV_LIT_REG => {
                // Read data from memory
                let literal = self.fetch16();
                let register = self.fetch_register_index();

                // Split u16
                let value: [u8; 2] = literal.to_be_bytes();

                // Write to register memory
                self.registers.set_byte(value[0], register);
                self.registers.set_byte(value[1], register + 1);
            }

            // Move register to register
            MOV_REG_REG => {
                // Read data from memory
                let register_from = self.fetch_register_index();
                let register_to = self.fetch_register_index();

                // Read from_register memory
                let value = [
                    self.registers.get_byte(register_from),
                    self.registers.get_byte(register_from + 1),
                ];

                // Write to_register memory
                self.registers.set_byte(value[0], register_to);
                self.registers.set_byte(value[1], register_to + 1);
            }

            // Move register to memory
            MOV_REG_MEM => {
                // Read data from memory
                let register_from = self.fetch_register_index();
                let address = self.fetch16();

                // Read from_register memory
                let value = [
                    self.registers.get_byte(register_from),
                    self.registers.get_byte(register_from + 1),
                ];

                // Write memory
                self.memory_mapper.set_byte(value[0], address);
                self.memory_mapper.set_byte(value[1], address + 1);
            }

            // Move memory to register
            MOV_MEM_REG => {
                // Read data from memory
                let address = self.fetch16();
                let register_to = self.fetch_register_index();

                // Read from memory
                let value = [
                    self.memory_mapper.get_byte(address),
                    self.memory_mapper.get_byte(address + 1),
                ];

                // Write to register
                self.registers.set_byte(value[0], register_to);
                self.registers.set_byte(value[1], register_to + 1);
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
            }

            // Add register to register
            ADD_REG_REG => {
                // Read data from memory
                let r1_offset = self.fetch_register_index();
                let r2_offset = self.fetch_register_index();

                // Read registers memory
                let r1_memory = [
                    self.registers.get_byte(r1_offset),
                    self.registers.get_byte(r1_offset + 1),
                ];
                let r2_memory = [
                    self.registers.get_byte(r2_offset),
                    self.registers.get_byte(r2_offset + 1),
                ];

                // Convert to u16
                let value_r1 = u16::from_be_bytes(r1_memory);
                let value_r2 = u16::from_be_bytes(r2_memory);

                // Add values
                self.set_register("acc", value_r1 + value_r2);
            }

            // Push value on to the stack
            PSH_LIT => {
                // Read data from memory
                let value = self.fetch16().to_be_bytes();

                // Push value on to the stack
                self.push(value);
            }

            // Push register on to the stack
            PSH_REG => {
                // Read data from memory
                let register = self.fetch_register_index();

                // Read register memory
                let value = [
                    self.registers.get_byte(register),
                    self.registers.get_byte(register + 1),
                ];

                // Push value on to the stack
                self.push(value);
            }

            // Pop value from the stack
            POP => {
                // Read data from memory
                let register = self.fetch_register_index();

                // Pop value from the stack
                let value = self.pop().to_be_bytes();

                // Write to register
                self.registers.set_byte(value[0], register);
                self.registers.set_byte(value[1], register + 1);
            }

            // Call subroutine from literal address
            CAL_LIT => {
                // Read data from memory
                let address = self.fetch16();

                // Push state to the stack
                self.push_state();

                // Jump to address
                self.set_register("ip", address);
            }

            // Call subroutine from register address
            CAL_REG => {
                // Read data from memory
                let register = self.fetch_register_index();

                // Read register memory
                let address = [
                    self.registers.get_byte(register),
                    self.registers.get_byte(register + 1),
                ];

                // Push state to the stack
                self.push_state();

                // Jump to address
                self.set_register("ip", u16::from_be_bytes(address));
            }

            // Return from subroutine
            RET => {
                // Restore state from the stack
                self.pop_state();
            }

            _ => {
                panic!("Instruction: 0x{:02X} not found", instruction);
            }
        }
    }

    // Run step trough the virtual machine
    fn step(&mut self, debug: bool) -> bool {
        // Read instruction from memory
        let instruction = self.fetch8();

        // Check if program ended
        if instruction == HLT {
            return true;
        }

        // Execute instruction
        self.execute(instruction);

        // Print debug info
        if debug {
            self.debug();
            self.memory_mapper.view_memory(self.get_register("ip"), 32);
            println!("{}", self.get_register("sp"));
            self.memory_mapper.view_memory(self.get_register("sp"), 32);
            println!("");
        }

        false
    }

    // Run the program in memory
    pub fn run(&mut self, debug: bool) {
        let mut halt = false;

        // While program is not ended
        while !halt {
            // Step trough the program
            halt = self.step(debug);
        }

        // Exit program
        std::process::exit(1)
    }

    // Print all registers and values
    pub fn debug(&self) {
        for (_, name) in self.registers_names.iter().enumerate() {
            println!("{}: 0x{:02X}", name, self.get_register(name));
        }
    }
}
