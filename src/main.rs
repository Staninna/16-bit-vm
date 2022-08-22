#![allow(dead_code)]
mod cpu;
mod create_memory;
mod instructions;
use cpu::*;
use create_memory::*;
use instructions::*;

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

    memory.set_byte(MOV_LIT_REG, program_address + 0);
    memory.set_byte(0x00, program_address + 1);
    memory.set_byte(0x01, program_address + 2); // 0x0001
    memory.set_byte(R1, program_address + 3);

    memory.set_byte(ADD_REG_REG, program_address + 4);
    memory.set_byte(R1, program_address + 5);
    memory.set_byte(R2, program_address + 6);

    memory.set_byte(JMP_NOT_EQ, program_address + 7);
    memory.set_byte(0x00, program_address + 8);
    memory.set_byte(0x64, program_address + 9); // 0x0064 / 100
    memory.set_byte(0x00, program_address + 10);
    memory.set_byte(0x18, program_address + 11); // 0x0018 / 24

    memory.set_byte(HLT, program_address + 12);

    program_address = 0x0018;

    memory.set_byte(MOV_REG_REG, program_address + 0);
    memory.set_byte(ACC, program_address + 1);
    memory.set_byte(R2, program_address + 2);

    memory.set_byte(JMP_NOT_EQ, program_address + 3);
    memory.set_byte(0xFF, program_address + 4);
    memory.set_byte(0xFF, program_address + 5); // 0xFFFF
    memory.set_byte(0x00, program_address + 6);
    memory.set_byte(0x00, program_address + 7); // 0xFFFF

    // Create virtual machine
    let mut cpu = CPU::new(memory);

    // Run the program
    cpu.run();
}
