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
    // Create memory devices
    let memory = Memory::new(0xFF00); // 65280 bytes
    let stack = Memory::new(0x00FF); // 255 bytes

    // Create memory mapper
    let mut mm = MemoryMapper::new();

    // Map memory devices to memory mapper
    mm.map(memory, 0x0000, 0xFF00, true);
    mm.map(stack, 0xFF00, 0xFFFF, true);

    // Load program to memory
    hardcode(&mut mm);

    // Create virtual machine
    let mut cpu = CPU::new(mm);

    // Run the program
    cpu.run(DEBUG);
}

// Hardcode a program to memory
fn hardcode(mm: &mut MemoryMapper) {
    let mut program_address = 0;

    mm.set_byte(MOV_LIT_REG, program_address + 0);
    mm.set_byte(0x00, program_address + 1);
    mm.set_byte(0x01, program_address + 2); // 0x0001
    mm.set_byte(R1, program_address + 3);

    mm.set_byte(ADD_REG_REG, program_address + 4);
    mm.set_byte(R1, program_address + 5);
    mm.set_byte(R2, program_address + 6);

    mm.set_byte(JMP_NOT_EQ, program_address + 7);
    mm.set_byte(0x00, program_address + 8);
    mm.set_byte(0x14, program_address + 9); // 0x0014 / 20
    mm.set_byte(0x00, program_address + 10);
    mm.set_byte(0xFF, program_address + 11); // 0x00FF / 256

    mm.set_byte(PSH_LIT, program_address + 12);
    mm.set_byte(0xFF, program_address + 13);
    mm.set_byte(0xFF, program_address + 14); // 0xFFFF

    mm.set_byte(PSH_LIT, program_address + 15);
    mm.set_byte(0xEE, program_address + 16);
    mm.set_byte(0xEE, program_address + 17); // 0xEEEE

    mm.set_byte(PSH_LIT, program_address + 18);
    mm.set_byte(0xDD, program_address + 19);
    mm.set_byte(0xDD, program_address + 20); // 0xDDDD

    mm.set_byte(PSH_LIT, program_address + 21);
    mm.set_byte(0xCC, program_address + 22);
    mm.set_byte(0xCC, program_address + 23); // 0xCCCC

    mm.set_byte(PSH_LIT, program_address + 24);
    mm.set_byte(0xBB, program_address + 25);
    mm.set_byte(0xBB, program_address + 26); // 0xBBBB

    mm.set_byte(PSH_LIT, program_address + 27);
    mm.set_byte(0xAA, program_address + 28);
    mm.set_byte(0xAA, program_address + 29); // 0xAAAA

    mm.set_byte(PSH_LIT, program_address + 30);
    mm.set_byte(0x00, program_address + 31);
    mm.set_byte(0x00, program_address + 32); // 0x0000

    mm.set_byte(CAL_LIT, program_address + 33);
    mm.set_byte(0xBA, program_address + 34);
    mm.set_byte(0xAA, program_address + 35); // 0xBAAA

    mm.set_byte(PSH_LIT, program_address + 36);
    mm.set_byte(0xAB, program_address + 37);
    mm.set_byte(0xCD, program_address + 38); // 0xABCD

    mm.set_byte(HLT, program_address + 39);

    program_address = 0x00FF;

    mm.set_byte(MOV_REG_REG, program_address + 0);
    mm.set_byte(ACC, program_address + 1);
    mm.set_byte(R2, program_address + 2);

    mm.set_byte(JMP_NOT_EQ, program_address + 3);
    mm.set_byte(0xFF, program_address + 4);
    mm.set_byte(0xFF, program_address + 5); // 0xFFFF
    mm.set_byte(0x00, program_address + 6);
    mm.set_byte(0x00, program_address + 7); // 0xFFFF

    program_address = 0xBAAA;

    mm.set_byte(MOV_LIT_REG, program_address + 0);
    mm.set_byte(0xAD, program_address + 1);
    mm.set_byte(0xAD, program_address + 2); // 0xADAD
    mm.set_byte(R3, program_address + 3);

    mm.set_byte(MOV_LIT_REG, program_address + 4);
    mm.set_byte(0xAE, program_address + 5);
    mm.set_byte(0xAE, program_address + 6); // 0xAEAE
    mm.set_byte(R4, program_address + 7);

    mm.set_byte(MOV_LIT_REG, program_address + 8);
    mm.set_byte(0xAF, program_address + 9);
    mm.set_byte(0xAF, program_address + 10); // 0xAFAF
    mm.set_byte(R5, program_address + 11);

    mm.set_byte(PSH_LIT, program_address + 12);
    mm.set_byte(0xAA, program_address + 13);
    mm.set_byte(0xAA, program_address + 14); // 0xAAAA

    mm.set_byte(PSH_LIT, program_address + 15);
    mm.set_byte(0xBB, program_address + 16);
    mm.set_byte(0xBB, program_address + 17); // 0xBBBB

    mm.set_byte(PSH_LIT, program_address + 18);
    mm.set_byte(0xCC, program_address + 19);
    mm.set_byte(0xCC, program_address + 20); // 0xCCCC

    mm.set_byte(POP, program_address + 21);
    mm.set_byte(R5, program_address + 22);

    mm.set_byte(RET, program_address + 23);
}
