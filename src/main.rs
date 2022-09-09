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

const DEBUG: bool = true;

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
    let program_address = 0;

    // Move 0xFFFF in register 1
    mm.set_byte(MOV_LIT_REG, program_address + 0);
    mm.set_byte(0xFF, program_address + 1);
    mm.set_byte(0xFF, program_address + 2); // 0xFFFF
    mm.set_byte(R1, program_address + 3);

    // Move 0xEEEE in register 2
    mm.set_byte(MOV_LIT_REG, program_address + 4);
    mm.set_byte(0xEE, program_address + 5);
    mm.set_byte(0xEE, program_address + 6); // 0xEEEE
    mm.set_byte(R2, program_address + 7);

    // Move 0xDDDD in memory address 0x0F00
    mm.set_byte(MOV_LIT_MEM, program_address + 8);
    mm.set_byte(0xDD, program_address + 9);
    mm.set_byte(0xDD, program_address + 10); //0xDDDD
    mm.set_byte(0x0F, program_address + 11);
    mm.set_byte(0x00, program_address + 12); // 0x0F00

    // Move 0xCCCC in memory address 0x0F02
    mm.set_byte(MOV_LIT_MEM, program_address + 13);
    mm.set_byte(0xCC, program_address + 14);
    mm.set_byte(0xCC, program_address + 15); //0xCCCC
    mm.set_byte(0x0F, program_address + 16);
    mm.set_byte(0x02, program_address + 17); // 0x0F02

    // Move 0x0002 in register 3
    mm.set_byte(MOV_LIT_REG, program_address + 18);
    mm.set_byte(0x00, program_address + 19);
    mm.set_byte(0x02, program_address + 20); // 0x0002
    mm.set_byte(R3, program_address + 21);

    // Move 0x0F00 with offset from register 3 memory address to register 4
    mm.set_byte(MOV_LIT_OFF_REG, program_address + 22);
    mm.set_byte(0x0F, program_address + 23);
    mm.set_byte(0x00, program_address + 24);
    mm.set_byte(R3, program_address + 25);
    mm.set_byte(R4, program_address + 26);

    // Move 0x0000 with offset from register 3 memory address to register 7
    mm.set_byte(MOV_LIT_OFF_REG, program_address + 27);
    mm.set_byte(0x00, program_address + 28);
    mm.set_byte(0x00, program_address + 29);
    mm.set_byte(R3, program_address + 30);
    mm.set_byte(R7, program_address + 31);

    // Move register 1 in register 5
    mm.set_byte(MOV_REG_REG, program_address + 32);
    mm.set_byte(R1, program_address + 33);
    mm.set_byte(R5, program_address + 34);

    // Move register 2 in register 6
    mm.set_byte(MOV_REG_REG, program_address + 35);
    mm.set_byte(R2, program_address + 36);
    mm.set_byte(R6, program_address + 37);

    // Move register 1 in memory address 0x0F04
    mm.set_byte(MOV_REG_MEM, program_address + 38);
    mm.set_byte(R1, program_address + 39);
    mm.set_byte(0x0F, program_address + 40);
    mm.set_byte(0x04, program_address + 41);

    // Move register 2 in memory address 0x0F06
    mm.set_byte(MOV_REG_MEM, program_address + 42);
    mm.set_byte(R2, program_address + 43);
    mm.set_byte(0x0F, program_address + 44);
    mm.set_byte(0x06, program_address + 45);

    // Move value on memory address in register 4 in register 6
    mm.set_byte(MOV_REG_PTR_REG, program_address + 46);
    mm.set_byte(R4, program_address + 47);
    mm.set_byte(R6, program_address + 48);

    // Move value on memory address in register 3 in register 5
    mm.set_byte(MOV_REG_PTR_REG, program_address + 49);
    mm.set_byte(R3, program_address + 50);
    mm.set_byte(R5, program_address + 51);

    // Move memory address 0x0F00 in register 8
    mm.set_byte(MOV_MEM_REG, program_address + 52);
    mm.set_byte(0x0F, program_address + 53);
    mm.set_byte(0x00, program_address + 54);
    mm.set_byte(R8, program_address + 55);

    // Move memory address 0x0F01 in register 7
    mm.set_byte(MOV_MEM_REG, program_address + 56);
    mm.set_byte(0x0F, program_address + 57);
    mm.set_byte(0x01, program_address + 58);
    mm.set_byte(R7, program_address + 59);

    // Reset registers by MOV_LIT_REG 0x0000 in RX
    mm.set_byte(MOV_LIT_REG, program_address + 60);
    mm.set_byte(0x00, program_address + 61);
    mm.set_byte(0x00, program_address + 62);
    mm.set_byte(R1, program_address + 63);

    mm.set_byte(MOV_LIT_REG, program_address + 64);
    mm.set_byte(0x00, program_address + 65);
    mm.set_byte(0x00, program_address + 66);
    mm.set_byte(R2, program_address + 67);

    mm.set_byte(MOV_LIT_REG, program_address + 68);
    mm.set_byte(0x00, program_address + 69);
    mm.set_byte(0x00, program_address + 70);
    mm.set_byte(R3, program_address + 71);

    mm.set_byte(MOV_LIT_REG, program_address + 72);
    mm.set_byte(0x00, program_address + 73);
    mm.set_byte(0x00, program_address + 74);
    mm.set_byte(R4, program_address + 75);

    mm.set_byte(MOV_LIT_REG, program_address + 76);
    mm.set_byte(0x00, program_address + 77);
    mm.set_byte(0x00, program_address + 78);
    mm.set_byte(R5, program_address + 79);

    mm.set_byte(MOV_LIT_REG, program_address + 80);
    mm.set_byte(0x00, program_address + 81);
    mm.set_byte(0x00, program_address + 82);
    mm.set_byte(R6, program_address + 83);

    mm.set_byte(MOV_LIT_REG, program_address + 84);
    mm.set_byte(0x00, program_address + 85);
    mm.set_byte(0x00, program_address + 86);
    mm.set_byte(R7, program_address + 87);

    mm.set_byte(MOV_LIT_REG, program_address + 88);
    mm.set_byte(0x00, program_address + 89);
    mm.set_byte(0x00, program_address + 90);
    mm.set_byte(R8, program_address + 91);

    // Move 0x0008 in register 1
    mm.set_byte(MOV_LIT_REG, program_address + 92);
    mm.set_byte(0x00, program_address + 93);
    mm.set_byte(0x08, program_address + 94);
    mm.set_byte(R1, program_address + 95);

    // Move 0x0008 in register 2
    mm.set_byte(MOV_LIT_REG, program_address + 96);
    mm.set_byte(0x00, program_address + 97);
    mm.set_byte(0x08, program_address + 98);
    mm.set_byte(R2, program_address + 99);

    // Add register 1 to register 2
    mm.set_byte(ADD_REG_REG, program_address + 100);
    mm.set_byte(R1, program_address + 101);
    mm.set_byte(R2, program_address + 102);

    // Add register 0x00FF to register 2
    mm.set_byte(ADD_LIT_REG, program_address + 103);
    mm.set_byte(0x00, program_address + 104);
    mm.set_byte(0xFF, program_address + 105);
    mm.set_byte(R2, program_address + 106);

    // Subtract 0x0001 from register 2
    mm.set_byte(SUB_LIT_REG, program_address + 107);
    mm.set_byte(0x00, program_address + 108);
    mm.set_byte(0x01, program_address + 109);
    mm.set_byte(R2, program_address + 110);

    // Subtract register 1 from register 2
    mm.set_byte(SUB_REG_REG, program_address + 111);
    mm.set_byte(R1, program_address + 112);
    mm.set_byte(R2, program_address + 113);

    // Subtract Register 2 from 0x0005
    mm.set_byte(SUB_LIT_REG, program_address + 114);
    mm.set_byte(0x00, program_address + 115);
    mm.set_byte(0x05, program_address + 116);
    mm.set_byte(R2, program_address + 117);

    // Multiply 0x0002 with register 2
    mm.set_byte(MUL_LIT_REG, program_address + 118);
    mm.set_byte(0x00, program_address + 119);
    mm.set_byte(0x02, program_address + 120);
    mm.set_byte(R2, program_address + 121);

    // Multiply register 2 by register 1
    mm.set_byte(MUL_REG_REG, program_address + 122);
    mm.set_byte(R2, program_address + 123);
    mm.set_byte(R1, program_address + 124);

    // Increment register 2
    mm.set_byte(INC_REG, program_address + 125);
    mm.set_byte(R2, program_address + 126);

    // Decrement register 2
    mm.set_byte(DEC_REG, program_address + 127);
    mm.set_byte(R2, program_address + 128);

    // Left shift register 2 by 1
    mm.set_byte(LSH_REG_LIT, program_address + 129);
    mm.set_byte(R2, program_address + 130);
    mm.set_byte(0x00, program_address + 131);
    mm.set_byte(0x01, program_address + 132);

    // Left shift register 2 by 1
    mm.set_byte(LSH_REG_LIT, program_address + 133);
    mm.set_byte(R2, program_address + 134);
    mm.set_byte(0x00, program_address + 135);
    mm.set_byte(0x01, program_address + 136);

    // Move 0x0002 in register 3
    mm.set_byte(MOV_LIT_REG, program_address + 137);
    mm.set_byte(0x00, program_address + 138);
    mm.set_byte(0x02, program_address + 139);
    mm.set_byte(R3, program_address + 140);

    // Right shift register 2 by 2
    mm.set_byte(RSH_REG_REG, program_address + 141);
    mm.set_byte(R2, program_address + 142);
    mm.set_byte(R3, program_address + 143);

    // Halt
    mm.set_byte(HLT, program_address + 144);
}
