#![allow(dead_code)]
mod cpu;
mod memory;
mod memory_mapper;

use cpu::*;
use memory::Memory;
use memory_mapper::MemoryMapper;

const IP: u8 = 0;
const ACC: u8 = 1;
const R1: u8 = 2;
const R2: u8 = 3;
const R3: u8 = 4;
const R4: u8 = 5;
const R5: u8 = 6;
const R6: u8 = 7;
const R7: u8 = 8;
const R8: u8 = 9;
const SP: u8 = 10;
const FP: u8 = 11;

const DEBUG: bool = true;

fn main() {
    let memory = Memory::new(0xFFFF);
    let mut memory_mapper = MemoryMapper::new(memory);

    memory_mapper.map(0x0000, 0xFFFF, true);

    // Load program to memory
    let mut program_address = 0;

    memory_mapper.set_byte(MOV_LIT_REG, program_address + 0);
    memory_mapper.set_byte(0x00, program_address + 1);
    memory_mapper.set_byte(0x01, program_address + 2); // 0x0001
    memory_mapper.set_byte(R1, program_address + 3);

    memory_mapper.set_byte(ADD_REG_REG, program_address + 4);
    memory_mapper.set_byte(R1, program_address + 5);
    memory_mapper.set_byte(R2, program_address + 6);

    memory_mapper.set_byte(JMP_NOT_EQ, program_address + 7);
    memory_mapper.set_byte(0x00, program_address + 8);
    memory_mapper.set_byte(0x64, program_address + 9); // 0x0064 / 100
    memory_mapper.set_byte(0x00, program_address + 10);
    memory_mapper.set_byte(0x18, program_address + 11); // 0x0018 / 24

    memory_mapper.set_byte(HLT, program_address + 12);

    program_address = 0x0018;

    memory_mapper.set_byte(MOV_REG_REG, program_address + 0);
    memory_mapper.set_byte(ACC, program_address + 1);
    memory_mapper.set_byte(R2, program_address + 2);

    memory_mapper.set_byte(JMP_NOT_EQ, program_address + 3);
    memory_mapper.set_byte(0xFF, program_address + 4);
    memory_mapper.set_byte(0xFF, program_address + 5); // 0xFFFF
    memory_mapper.set_byte(0x00, program_address + 6);
    memory_mapper.set_byte(0x00, program_address + 7); // 0xFFFF

    // Create virtual machine
    let mut cpu = CPU::new(memory_mapper);

    // Run the program
    cpu.run(DEBUG);
}
