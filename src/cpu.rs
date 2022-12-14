// Imports
use crate::device::{Device, DeviceType};
use crate::device_mapper::DeviceMapper;
use std::collections::HashMap;

// Instructions for the CPU

// Move instructions
pub const MOV_LIT_REG: u8 = 0x10;
pub const MOV_REG_REG: u8 = 0x11;
pub const MOV_REG_MEM: u8 = 0x12;
pub const MOV_MEM_REG: u8 = 0x13;
pub const MOV_LIT_MEM: u8 = 0x14;
pub const MOV_REG_PTR_REG: u8 = 0x15;
pub const MOV_LIT_OFF_REG: u8 = 0x16;

// Arithmetic instructions
pub const ADD_REG_REG: u8 = 0x20;
pub const ADD_LIT_REG: u8 = 0x21;
pub const SUB_LIT_REG: u8 = 0x22;
pub const SUB_REG_LIT: u8 = 0x23;
pub const SUB_REG_REG: u8 = 0x24;
pub const INC_REG: u8 = 0x25;
pub const DEC_REG: u8 = 0x26;
pub const MUL_LIT_REG: u8 = 0x27;
pub const MUL_REG_REG: u8 = 0x28;

// Binary manipulation instructions
pub const LSH_REG_LIT: u8 = 0x30;
pub const LSH_REG_REG: u8 = 0x31;
pub const RSH_REG_LIT: u8 = 0x32;
pub const RSH_REG_REG: u8 = 0x33;
pub const AND_REG_LIT: u8 = 0x34;
pub const AND_REG_REG: u8 = 0x35;
pub const OR_REG_LIT: u8 = 0x36;
pub const OR_REG_REG: u8 = 0x37;
pub const XOR_REG_LIT: u8 = 0x38;
pub const XOR_REG_REG: u8 = 0x39;
pub const NOT: u8 = 0x3A;

// Branching instructions
pub const JNE_REG: u8 = 0x40;
pub const JNE_LIT: u8 = 0x41;
pub const JEQ_REG: u8 = 0x42;
pub const JEQ_LIT: u8 = 0x43;
pub const JLT_REG: u8 = 0x44;
pub const JLT_LIT: u8 = 0x45;
pub const JGT_REG: u8 = 0x46;
pub const JGT_LIT: u8 = 0x47;
pub const JLE_REG: u8 = 0x48;
pub const JLE_LIT: u8 = 0x49;
pub const JGE_REG: u8 = 0x4A;
pub const JGE_LIT: u8 = 0x4B;

// Miscellaneous instructions
pub const PSH_LIT: u8 = 0x50;
pub const PSH_REG: u8 = 0x51;
pub const POP: u8 = 0x52;
pub const CAL_LIT: u8 = 0x53;
pub const CAL_REG: u8 = 0x54;
pub const RET: u8 = 0x55;
pub const HLT: u8 = 0x56;

// CPU class
pub struct CPU {
    device_mapper: DeviceMapper,
    registers_names: Vec<String>,
    registers: Device,
    registers_map: HashMap<String, usize>,
    stack_frame_size: u16,
}

// CPU implementation
impl CPU {
    pub fn new(device_mapper: DeviceMapper) -> Self {
        // Create CPU all registers
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
        let mut registers = Device::new(registers_names.len() * 2, DeviceType::Memory);

        // Map the registers names
        let mut registers_map = HashMap::new();
        for (index, name) in registers_names.iter().enumerate() {
            registers_map.insert(name.clone(), index * 2);
        }

        // Set stack pointer and frame pointer to the right address
        let sp_offset = registers_map.get("sp").unwrap();
        let fp_offset = registers_map.get("fp").unwrap();
        let memory_position = ((0xFFFF - 2) as u16).to_be_bytes(); // TODO try to make this dynamic in some way
        let stack_frame_size = 0;
        registers.set_byte(memory_position[0], *sp_offset);
        registers.set_byte(memory_position[0], *fp_offset);
        registers.set_byte(memory_position[1], *sp_offset + 1);
        registers.set_byte(memory_position[1], *fp_offset + 1);

        Self {
            device_mapper,
            registers_names,
            registers,
            registers_map,
            stack_frame_size,
        }
    }

    // Read a register
    fn get_register(&self, name: &str) -> u16 {
        // Check if register exists
        if !self.registers_map.contains_key(name) {
            panic!("Register {} not found", name);
        }

        // Read register
        let offset = self.registers_map.get(name).unwrap();
        let memory = [
            self.registers.get_byte(*offset),
            self.registers.get_byte(*offset + 1),
        ];
        u16::from_be_bytes(memory)
    }

    // Write to a register
    fn set_register(&mut self, name: &str, value: u16) {
        // Check if register exists
        if !self.registers_map.contains_key(name) {
            panic!("Register {} not found", name);
        }

        // Write to register
        let offset = self.registers_map.get(name).unwrap();
        let bytes = value.to_be_bytes();
        self.registers.set_byte(bytes[0], *offset);
        self.registers.set_byte(bytes[1], *offset + 1);
    }

    // Read byte from memory
    fn fetch8(&mut self) -> u8 {
        let ip = self.get_register("ip");
        let byte = self.device_mapper.get_byte(ip);
        self.set_register("ip", ip + 1);
        byte
    }

    // Read bytes from memory
    fn fetch16(&mut self) -> u16 {
        let ip = self.get_register("ip");
        let bytes = [
            self.device_mapper.get_byte(ip),
            self.device_mapper.get_byte(ip + 1),
        ];
        self.set_register("ip", ip + 2);
        u16::from_be_bytes(bytes)
    }

    // Push bytes on the stack
    fn push(&mut self, bytes: [u8; 2]) {
        // Read stack pointer
        let sp_address = self.get_register("sp");

        // Write stack
        self.device_mapper.set_byte(bytes[0], sp_address);
        self.device_mapper.set_byte(bytes[1], sp_address + 1);

        // Move stack pointer
        self.set_register("sp", sp_address - 2);
        self.stack_frame_size += 2;
    }

    // Pop bytes from the stack
    fn pop(&mut self) -> u16 {
        // Move stack pointer
        let next_sp_address = self.get_register("sp") + 2;
        self.set_register("sp", next_sp_address);
        self.stack_frame_size -= 2;

        // Read stack
        u16::from_be_bytes([
            self.device_mapper.get_byte(next_sp_address),
            self.device_mapper.get_byte(next_sp_address + 1),
        ])
    }

    // Push CPU state
    fn push_state(&mut self) {
        // Push registers
        self.push(self.get_register("r1").to_be_bytes());
        self.push(self.get_register("r2").to_be_bytes());
        self.push(self.get_register("r3").to_be_bytes());
        self.push(self.get_register("r4").to_be_bytes());
        self.push(self.get_register("r5").to_be_bytes());
        self.push(self.get_register("r6").to_be_bytes());
        self.push(self.get_register("r7").to_be_bytes());
        self.push(self.get_register("r8").to_be_bytes());
        self.push(self.get_register("ip").to_be_bytes());

        // Push frame size
        self.push((self.stack_frame_size + 2).to_be_bytes());
        self.stack_frame_size = 0;

        // Write new frame pointer
        self.set_register("fp", self.get_register("sp"));
    }

    // Pop CPU state
    fn pop_state(&mut self) {
        // Read frame pointer
        let frame_pointer_address = self.get_register("fp");

        // Write new stack pointer
        self.set_register("sp", frame_pointer_address);

        // Pop stack frame size
        self.stack_frame_size = self.pop();
        let stack_frame_size = self.stack_frame_size;

        // Pop registers
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

        // Remove arguments frm CAL
        let cal_args = self.pop();
        for _ in 0..cal_args {
            self.pop();
        }

        // Reset frame pointer
        self.set_register("fp", frame_pointer_address + stack_frame_size);
    }

    // Get register offset
    fn fetch_register_index(&mut self) -> usize {
        (self.fetch8() as usize % self.registers_names.len()) * 2
    }

    // Execute an instruction
    fn execute(&mut self, instruction: u8) {
        match instruction {
            // Move instructions

            // Move literal to register
            MOV_LIT_REG => {
                // Read instruction
                let literal = self.fetch16();
                let register = self.fetch_register_index();
                let value: [u8; 2] = literal.to_be_bytes();

                // Write to register
                self.registers.set_byte(value[0], register);
                self.registers.set_byte(value[1], register + 1);
            }

            // Move literal to memory
            MOV_LIT_MEM => {
                // Read instruction
                let literal = self.fetch16();
                let address = self.fetch16();
                let value: [u8; 2] = literal.to_be_bytes();

                // Write to register
                self.device_mapper.set_byte(value[0], address);
                self.device_mapper.set_byte(value[1], address + 1);
            }

            // Move register to register with offset
            MOV_LIT_OFF_REG => {
                // Read instruction
                let base_address = self.fetch16();
                let register_from = self.fetch_register_index();
                let register_to = self.fetch_register_index();

                // Read offset
                let register_memory = [
                    self.registers.get_byte(register_from),
                    self.registers.get_byte(register_from + 1),
                ];
                let offset = u16::from_be_bytes(register_memory);

                // Read value
                let value = [
                    self.device_mapper.get_byte(base_address + offset),
                    self.device_mapper.get_byte(base_address + offset),
                ];

                // Write to register
                self.registers.set_byte(value[0], register_to);
                self.registers.set_byte(value[1], register_to + 1);
            }

            // Move register to register
            MOV_REG_REG => {
                // Read instruction
                let register_from = self.fetch_register_index();
                let register_to = self.fetch_register_index();

                // Read from_register
                let value = [
                    self.registers.get_byte(register_from),
                    self.registers.get_byte(register_from + 1),
                ];

                // Write to_register
                self.registers.set_byte(value[0], register_to);
                self.registers.set_byte(value[1], register_to + 1);
            }

            // Move register to memory
            MOV_REG_MEM => {
                // Read instruction
                let register_from = self.fetch_register_index();
                let address = self.fetch16();

                // Read from_register
                let value = [
                    self.registers.get_byte(register_from),
                    self.registers.get_byte(register_from + 1),
                ];

                // Write memory
                self.device_mapper.set_byte(value[0], address);
                self.device_mapper.set_byte(value[1], address + 1);
            }

            // Move register pointer to register
            MOV_REG_PTR_REG => {
                // Read instruction
                let register_from = self.fetch_register_index();
                let register_to = self.fetch_register_index();

                // Read register
                let register_memory = [
                    self.registers.get_byte(register_from),
                    self.registers.get_byte(register_from + 1),
                ];
                let pointer = u16::from_be_bytes(register_memory);

                // Read pointer value
                let value = [
                    self.device_mapper.get_byte(pointer),
                    self.device_mapper.get_byte(pointer + 1),
                ];

                // Write to_register
                self.registers.set_byte(value[0], register_to);
                self.registers.set_byte(value[1], register_to + 1);
            }

            // Move memory to register
            MOV_MEM_REG => {
                // Read instruction
                let address = self.fetch16();
                let register_to = self.fetch_register_index();

                // Read from memory
                let value = [
                    self.device_mapper.get_byte(address),
                    self.device_mapper.get_byte(address + 1),
                ];

                // Write register
                self.registers.set_byte(value[0], register_to);
                self.registers.set_byte(value[1], register_to + 1);
            }

            // Algorithmic instructions

            // Add register to register
            ADD_REG_REG => {
                // Read instruction
                let register1 = self.fetch_register_index();
                let register2 = self.fetch_register_index();

                // Read register 1
                let register1_memory = [
                    self.registers.get_byte(register1),
                    self.registers.get_byte(register1 + 1),
                ];
                let value_register1 = u16::from_be_bytes(register1_memory);

                // Read register 2
                let register2_value = [
                    self.registers.get_byte(register2),
                    self.registers.get_byte(register2 + 1),
                ];
                let value_register2 = u16::from_be_bytes(register2_value);

                // Add values
                self.set_register("acc", value_register1 + value_register2);
            }

            // Add literal to register
            ADD_LIT_REG => {
                // Read instruction
                let literal = self.fetch16();
                let register = self.fetch_register_index();

                // Read register
                let register_memory = [
                    self.registers.get_byte(register),
                    self.registers.get_byte(register + 1),
                ];
                let value_register = u16::from_be_bytes(register_memory);

                // Add values
                self.set_register("acc", value_register + literal);
            }

            // Subtract literal from register
            SUB_LIT_REG => {
                // Read instruction
                let literal = self.fetch16();
                let register = self.fetch_register_index();

                // Read register
                let register_memory = [
                    self.registers.get_byte(register),
                    self.registers.get_byte(register + 1),
                ];
                let value_register = u16::from_be_bytes(register_memory);

                // Subtract values
                self.set_register("acc", value_register - literal);
            }

            // Subtract register from literal
            SUB_REG_LIT => {
                // Read instruction
                let register = self.fetch_register_index();
                let literal = self.fetch16();
                let register_memory = [
                    self.registers.get_byte(register),
                    self.registers.get_byte(register + 1),
                ];
                let value_register = u16::from_be_bytes(register_memory);

                // Subtract values
                self.set_register("acc", value_register - literal);
            }

            // Subtract register from register
            SUB_REG_REG => {
                // Read instruction
                let register1 = self.fetch_register_index();
                let register2 = self.fetch_register_index();

                // Read register 1
                let register1_memory = [
                    self.registers.get_byte(register1),
                    self.registers.get_byte(register1 + 1),
                ];
                let value_register1 = u16::from_be_bytes(register1_memory);

                // Read register 2
                let register2_value = [
                    self.registers.get_byte(register2),
                    self.registers.get_byte(register2 + 1),
                ];
                let value_register2 = u16::from_be_bytes(register2_value);

                // Subtract values
                self.set_register("acc", value_register1 - value_register2);
            }

            // Multiply literal by register
            MUL_LIT_REG => {
                // Read instruction
                let literal = self.fetch16();
                let register = self.fetch_register_index();

                // Read register
                let register_memory = [
                    self.registers.get_byte(register),
                    self.registers.get_byte(register + 1),
                ];
                let value_register = u16::from_be_bytes(register_memory);

                // Multiply values
                self.set_register("acc", value_register * literal);
            }

            // Multiply register by register
            MUL_REG_REG => {
                // Read instruction
                let register1 = self.fetch_register_index();
                let register2 = self.fetch_register_index();

                // Read register 1
                let register1_memory = [
                    self.registers.get_byte(register1),
                    self.registers.get_byte(register1 + 1),
                ];
                let value_register1 = u16::from_be_bytes(register1_memory);

                // Read register 2
                let register2_value = [
                    self.registers.get_byte(register2),
                    self.registers.get_byte(register2 + 1),
                ];
                let value_register2 = u16::from_be_bytes(register2_value);

                // Multiply values
                self.set_register("acc", value_register1 * value_register2);
            }

            // Increment register
            INC_REG => {
                // Read instruction
                let register = self.fetch_register_index();

                // Read register
                let register_memory = [
                    self.registers.get_byte(register),
                    self.registers.get_byte(register + 1),
                ];

                // Increment value
                let old_value = u16::from_be_bytes(register_memory);
                let new_value = (old_value + 1).to_be_bytes();

                // Write register
                self.registers.set_byte(new_value[0], register);
                self.registers.set_byte(new_value[1], register + 1);
            }

            // Decrement register
            DEC_REG => {
                // Read instruction
                let register = self.fetch_register_index();

                // Read register
                let register_memory = [
                    self.registers.get_byte(register),
                    self.registers.get_byte(register + 1),
                ];

                // Decrement value
                let old_value = u16::from_be_bytes(register_memory);
                let new_value = (old_value - 1).to_be_bytes();

                // Write register
                self.registers.set_byte(new_value[0], register);
                self.registers.set_byte(new_value[1], register + 1);
            }

            // Binary manipulation instructions

            // Left shift register by literal
            LSH_REG_LIT => {
                // Read instruction
                let register = self.fetch_register_index();
                let literal = self.fetch16();

                // Read register
                let register_memory = [
                    self.registers.get_byte(register),
                    self.registers.get_byte(register + 1),
                ];
                let value_register = u16::from_be_bytes(register_memory);

                // Shift value
                let new_value = (value_register << literal).to_be_bytes();

                // Left shift value
                self.registers.set_byte(new_value[0], register);
                self.registers.set_byte(new_value[1], register + 1);
            }

            // Left shift register by register
            LSH_REG_REG => {
                // Read instruction
                let register1 = self.fetch_register_index();
                let register2 = self.fetch_register_index();

                // Read register 1
                let register1_memory = [
                    self.registers.get_byte(register1),
                    self.registers.get_byte(register1 + 1),
                ];
                let value_register1 = u16::from_be_bytes(register1_memory);

                // Read register 2
                let register2_memory = [
                    self.registers.get_byte(register2),
                    self.registers.get_byte(register2 + 1),
                ];
                let shift_by = u16::from_be_bytes(register2_memory);

                // Shift value
                let new_value = (value_register1 << shift_by).to_be_bytes();

                // Write register 1
                self.registers.set_byte(new_value[0], register1);
                self.registers.set_byte(new_value[1], register1 + 1);
            }

            // Right shift register by literal
            RSH_REG_LIT => {
                // Read instruction
                let register = self.fetch_register_index();
                let literal = self.fetch16();

                // Read register
                let register_memory = [
                    self.registers.get_byte(register),
                    self.registers.get_byte(register + 1),
                ];
                let value_register = u16::from_be_bytes(register_memory);

                // Shift value
                let new_value = (value_register >> literal).to_be_bytes();

                // Left shift value
                self.registers.set_byte(new_value[0], register);
                self.registers.set_byte(new_value[1], register + 1);
            }

            // Right shift register by register
            RSH_REG_REG => {
                // Read instruction
                let register1 = self.fetch_register_index();
                let register2 = self.fetch_register_index();

                // Read register 1
                let register1_memory = [
                    self.registers.get_byte(register1),
                    self.registers.get_byte(register1 + 1),
                ];
                let value_register1 = u16::from_be_bytes(register1_memory);

                // Read register 2
                let register2_memory = [
                    self.registers.get_byte(register2),
                    self.registers.get_byte(register2 + 1),
                ];
                let shift_by = u16::from_be_bytes(register2_memory);

                // Shift value
                let new_value = (value_register1 >> shift_by).to_be_bytes();

                // Write register 1
                self.registers.set_byte(new_value[0], register1);
                self.registers.set_byte(new_value[1], register1 + 1);
            }

            // And register with literal
            AND_REG_LIT => {
                // Read instruction
                let register = self.fetch_register_index();
                let literal = self.fetch16();

                // Read register
                let register_memory = [
                    self.registers.get_byte(register),
                    self.registers.get_byte(register + 1),
                ];
                let value_register = u16::from_be_bytes(register_memory);

                // And values
                self.set_register("acc", value_register & literal)
            }

            // And register with register
            AND_REG_REG => {
                // Read instruction
                let register1 = self.fetch_register_index();
                let register2 = self.fetch_register_index();

                // Read register 1
                let register1_memory = [
                    self.registers.get_byte(register1),
                    self.registers.get_byte(register1 + 1),
                ];
                let value_register1 = u16::from_be_bytes(register1_memory);

                // Read register 1
                let register2_memory = [
                    self.registers.get_byte(register2),
                    self.registers.get_byte(register2 + 1),
                ];
                let value_register2 = u16::from_be_bytes(register2_memory);

                // And values
                self.set_register("acc", value_register1 & value_register2)
            }

            // Or register with literal
            OR_REG_LIT => {
                // Read instruction
                let register = self.fetch_register_index();
                let literal = self.fetch16();

                // Read register
                let register_memory = [
                    self.registers.get_byte(register),
                    self.registers.get_byte(register + 1),
                ];
                let value_register = u16::from_be_bytes(register_memory);

                // And values
                self.set_register("acc", value_register | literal)
            }

            // Or register with register
            OR_REG_REG => {
                // Read instruction
                let register1 = self.fetch_register_index();
                let register2 = self.fetch_register_index();

                // Read register 1
                let register1_memory = [
                    self.registers.get_byte(register1),
                    self.registers.get_byte(register1 + 1),
                ];
                let value_register1 = u16::from_be_bytes(register1_memory);

                // Read register 1
                let register2_memory = [
                    self.registers.get_byte(register2),
                    self.registers.get_byte(register2 + 1),
                ];
                let value_register2 = u16::from_be_bytes(register2_memory);

                // And values
                self.set_register("acc", value_register1 | value_register2)
            }

            // Not register
            NOT => {
                // Read instruction
                let register = self.fetch_register_index();

                // Read register
                let register_memory = [
                    self.registers.get_byte(register),
                    self.registers.get_byte(register + 1),
                ];
                let value_register = u16::from_be_bytes(register_memory);

                // Not value
                self.set_register("acc", !value_register)
            }

            // Branching instructions

            // Jump if register not equal
            JNE_REG => {
                // Read instruction
                let register = self.fetch_register_index();
                let address = self.fetch16();

                // Read register
                let register_memory = [
                    self.registers.get_byte(register),
                    self.registers.get_byte(register + 1),
                ];
                let value_register = u16::from_be_bytes(register_memory);

                // Move instruction pointer
                if value_register != self.get_register("acc") {
                    self.set_register("ip", address);
                }
            }

            // Jump if literal not equal
            JNE_LIT => {
                // Read instruction
                let value = self.fetch16();
                let address = self.fetch16();

                // Move instruction pointer
                if value != self.get_register("acc") {
                    self.set_register("ip", address);
                }
            }

            // Jump if register equal
            JEQ_REG => {
                // Read instruction
                let register = self.fetch_register_index();
                let address = self.fetch16();

                // Read register
                let register_memory = [
                    self.registers.get_byte(register),
                    self.registers.get_byte(register + 1),
                ];
                let value_register = u16::from_be_bytes(register_memory);

                // Move instruction pointer
                if value_register == self.get_register("acc") {
                    self.set_register("ip", address);
                }
            }

            // Jump if literal equal
            JEQ_LIT => {
                // Read instruction
                let value = self.fetch16();
                let address = self.fetch16();

                // Move instruction pointer
                if value == self.get_register("acc") {
                    self.set_register("ip", address);
                }
            }

            // Jump if register less then
            JLT_REG => {
                // Read instruction
                let register = self.fetch_register_index();
                let address = self.fetch16();

                // Read register
                let register_memory = [
                    self.registers.get_byte(register),
                    self.registers.get_byte(register + 1),
                ];
                let value_register = u16::from_be_bytes(register_memory);

                // Move instruction pointer
                if value_register < self.get_register("acc") {
                    self.set_register("ip", address);
                }
            }

            // Jump if literal less than
            JLT_LIT => {
                // Read instruction
                let value = self.fetch16();
                let address = self.fetch16();

                // Move instruction pointer
                if value < self.get_register("acc") {
                    self.set_register("ip", address);
                }
            }

            // Jump if register greater then
            JGT_REG => {
                // Read instruction
                let register = self.fetch_register_index();
                let address = self.fetch16();

                // Read register
                let register_memory = [
                    self.registers.get_byte(register),
                    self.registers.get_byte(register + 1),
                ];
                let value_register = u16::from_be_bytes(register_memory);

                // Move instruction pointer
                if value_register > self.get_register("acc") {
                    self.set_register("ip", address);
                }
            }

            // Jump if literal greater than
            JGT_LIT => {
                // Read instruction
                let value = self.fetch16();
                let address = self.fetch16();

                // Move instruction pointer
                if value < self.get_register("acc") {
                    self.set_register("ip", address);
                }
            }

            // Jump if register greater then
            JLE_REG => {
                // Read instruction
                let register = self.fetch_register_index();
                let address = self.fetch16();

                // Read register
                let register_memory = [
                    self.registers.get_byte(register),
                    self.registers.get_byte(register + 1),
                ];
                let value_register = u16::from_be_bytes(register_memory);

                // Move instruction pointer
                if value_register <= self.get_register("acc") {
                    self.set_register("ip", address);
                }
            }

            // Jump if literal less or equal than
            JLE_LIT => {
                // Read instruction
                let value = self.fetch16();
                let address = self.fetch16();

                // Move instruction pointer
                if value <= self.get_register("acc") {
                    self.set_register("ip", address);
                }
            }

            // Jump if register greater then
            JGE_REG => {
                // Read instruction
                let register = self.fetch_register_index();
                let address = self.fetch16();

                // Read register
                let register_memory = [
                    self.registers.get_byte(register),
                    self.registers.get_byte(register + 1),
                ];
                let value_register = u16::from_be_bytes(register_memory);

                // Move instruction pointer
                if value_register >= self.get_register("acc") {
                    self.set_register("ip", address);
                }
            }

            // Jump if literal greater or equal than
            JGE_LIT => {
                // Read instruction
                let value = self.fetch16();
                let address = self.fetch16();

                // Move instruction pointer
                if value >= self.get_register("acc") {
                    self.set_register("ip", address);
                }
            }

            // Miscellaneous instructions

            // Push literal
            PSH_LIT => {
                // Read instruction
                let literal = self.fetch16().to_be_bytes();

                // Push literal
                self.push(literal);
            }

            // Push register
            PSH_REG => {
                // Read instruction
                let register = self.fetch_register_index();

                // Read register
                let value = [
                    self.registers.get_byte(register),
                    self.registers.get_byte(register + 1),
                ];

                // Push register
                self.push(value);
            }

            // Pop
            POP => {
                // Read instruction
                let register = self.fetch_register_index();

                // Pop value
                let value = self.pop();
                let bytes = value.to_be_bytes();

                // Write register
                self.registers.set_byte(bytes[0], register);
                self.registers.set_byte(bytes[1], register + 1);
            }

            // Call subroutine from literal
            CAL_LIT => {
                // Read instruction
                let literal = self.fetch16();

                // Push state
                self.push_state();

                // Move instruction pointer
                self.set_register("ip", literal);
            }

            // Call subroutine from register
            CAL_REG => {
                // Read instruction
                let register = self.fetch_register_index();

                // Read register
                let address = [
                    self.registers.get_byte(register),
                    self.registers.get_byte(register + 1),
                ];

                // Push state
                self.push_state();

                // Move instruction pointer
                self.set_register("ip", u16::from_be_bytes(address));
            }

            // Return from CAL
            RET => {
                // Restore state from the stack
                self.pop_state();
            }

            _ => {
                panic!("Instruction: 0x{:02X} not found", instruction);
            }
        }
    }

    // Run one instruction
    fn step(&mut self, debug: bool) -> bool {
        // Read instruction
        let instruction = self.fetch8();

        // Check if program ended
        if instruction == HLT {
            // Return true if ended
            return true;
        }

        // Execute instruction
        self.execute(instruction);

        // Print debug info
        if debug {
            self.debug();
            self.device_mapper.view_memory(self.get_register("ip"), 16);
            self.device_mapper.view_memory(0xFFFF - 16, 16);
            self.device_mapper.view_memory(0x0F00, 16);
            println!("");

            // Wait for input
            std::io::stdin().read_line(&mut String::new()).unwrap();
        }

        // Return false if not ended
        false
    }

    // Run program
    pub fn run(&mut self, debug: bool) {
        // Set halt to false
        let mut halt = false;

        // While running program
        while !halt {
            // Run instruction
            halt = self.step(debug);
        }

        // Print debug info
        if debug {
            self.debug();
            self.device_mapper.view_memory(self.get_register("ip"), 16);
            self.device_mapper.view_memory(0xFFFF - 16, 16);
            self.device_mapper.view_memory(0x0F00, 16);
            println!("");
        }

        // Exit program
        std::process::exit(1)
    }

    // Print registers
    pub fn debug(&self) {
        for (_, name) in self.registers_names.iter().enumerate() {
            println!("{}: 0x{:04X}", name, self.get_register(name));
        }
    }
}
