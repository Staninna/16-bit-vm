#![allow(dead_code)]
mod cpu;
use cpu::{create_memory::create_memory, instructions::*, *};

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

fn main() {
    let mut memory = create_memory(0xFFFF);

    // Load program to memory
    let mut program_address = 0;

    memory.buffer[program_address + 0] = MOV_LIT_REG;
    memory.buffer[program_address + 1] = 0x00;
    memory.buffer[program_address + 2] = 0x01; // 0x0001
    memory.buffer[program_address + 3] = R1;

    memory.buffer[program_address + 4] = ADD_REG_REG;
    memory.buffer[program_address + 5] = R1;
    memory.buffer[program_address + 6] = R2;

    memory.buffer[program_address + 7] = JMP_NOT_EQ;
    memory.buffer[program_address + 8] = 0x00;
    memory.buffer[program_address + 9] = 0x64; // 0x0064 / 100
    memory.buffer[program_address + 10] = 0x00;
    memory.buffer[program_address + 11] = 0x18; // 0x0018 / 24

    memory.buffer[program_address + 12] = HLT;

    program_address = 0x0018;

    memory.buffer[program_address + 0] = MOV_REG_REG;
    memory.buffer[program_address + 1] = ACC;
    memory.buffer[program_address + 2] = R2;

    memory.buffer[program_address + 3] = JMP_NOT_EQ;
    memory.buffer[program_address + 4] = 0xFF;
    memory.buffer[program_address + 5] = 0xFF; // 0xFFFF
    memory.buffer[program_address + 6] = 0x00;
    memory.buffer[program_address + 7] = 0x00; // 0xFFFF

    // Create virtual machine
    let mut cpu = CPU::new(memory);

    // Run the program
    cpu.run();
}
