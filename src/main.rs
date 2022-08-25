#![allow(dead_code)]
mod cpu;
mod device;
mod device_mapper;

use cpu::*;
use device::*;
use device_mapper::DeviceMapper;

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

const DEBUG: bool = false;

fn main() {
    // Create memory devices
    let memory = Device::new(0xFFFF, DeviceType::Memory);
    let stack = Device::new(0x00FF, DeviceType::Memory);

    // Create screen device
    let screen = Device::new(0x00FF, DeviceType::Stdout);

    // Create memory mapper
    let mut mm = DeviceMapper::new();

    // Map memory devices to memory mapper
    mm.map(memory, 0x0000, 0xFF00, true);
    mm.map(stack, 0xFF00, 0xFFFF, true);
    mm.map(screen, 0x3000, 0x30FF, true);

    // Load program to memory
    hardcode(&mut mm);

    // Create virtual machine
    let mut cpu = CPU::new(mm);

    // Run the program
    cpu.run(DEBUG);
}

// Hardcode a program to memory
fn hardcode(mm: &mut DeviceMapper) {
    let mut program_address = 0;

    // Clear screen
    program_address = print(mm, 0, program_address, " ", STDOUT_CLEAR);
    for i in 0x00..0xFF {
        if i % 2 == 0 {
            program_address = print(mm, i as u8, program_address, "$", STDOUT_BOLD);
        } else {
            program_address = print(mm, i as u8, program_address, "%", STDOUT_REGULAR);
        }
    }

    mm.set_byte(HLT, program_address);
}

fn print(mm: &mut DeviceMapper, pos: u8, index: u16, character: &str, command: u8) -> u16 {
    mm.set_byte(MOV_LIT_REG, index);
    mm.set_byte(command, index + 1);
    mm.set_byte(character.as_bytes()[0], index + 2);
    mm.set_byte(R1, index + 3);

    mm.set_byte(MOV_REG_MEM, index + 4);
    mm.set_byte(R1, index + 5);
    mm.set_byte(0x30, index + 6);
    mm.set_byte(pos, index + 7);

    index + 8
}
