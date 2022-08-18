#![allow(dead_code)]
mod cpu;
use cpu::{create_memory::create_memory, instructions::*, *};

fn wait() {
    std::io::stdin().read_line(&mut String::new()).unwrap();
}

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
    let mut memory = create_memory(0x01FF);

    // Load program to memory

    memory.buffer[0] = PSH_LIT;
    memory.buffer[1] = 0x33;
    memory.buffer[2] = 0x33; // 0x3333

    memory.buffer[3] = PSH_LIT;
    memory.buffer[4] = 0x22;
    memory.buffer[5] = 0x22; // 0x2222

    memory.buffer[6] = PSH_LIT;
    memory.buffer[7] = 0x11;
    memory.buffer[8] = 0x11; // 0x1111

    memory.buffer[9] = MOV_LIT_REG;
    memory.buffer[10] = 0x12;
    memory.buffer[11] = 0x34; // 0x1234
    memory.buffer[12] = R1;

    memory.buffer[13] = MOV_LIT_REG;
    memory.buffer[14] = 0x56;
    memory.buffer[15] = 0x78; // 0x5678
    memory.buffer[16] = R4;

    memory.buffer[17] = PSH_LIT;
    memory.buffer[18] = 0x00;
    memory.buffer[19] = 0x00; // 0x0000
    memory.buffer[20] = CALL_LIT;
    memory.buffer[21] = 0x00;
    memory.buffer[22] = 0xAA; // 0x3000

    // Subroutine
    let mut subroutine_address: u16 = 0x00AA;

    memory.buffer[subroutine_address as usize + 0] = PSH_LIT;
    memory.buffer[subroutine_address as usize + 1] = 0x01;
    memory.buffer[subroutine_address as usize + 2] = 0x02; // 0x0102

    memory.buffer[subroutine_address as usize + 3] = PSH_LIT;
    memory.buffer[subroutine_address as usize + 4] = 0x03;
    memory.buffer[subroutine_address as usize + 5] = 0x04; // 0x0304

    memory.buffer[subroutine_address as usize + 6] = PSH_LIT;
    memory.buffer[subroutine_address as usize + 7] = 0x05;
    memory.buffer[subroutine_address as usize + 8] = 0x06; // 0x0506

    memory.buffer[subroutine_address as usize + 9] = MOV_LIT_REG;
    memory.buffer[subroutine_address as usize + 10] = 0x07;
    memory.buffer[subroutine_address as usize + 11] = 0x08; // 0x0708
    memory.buffer[subroutine_address as usize + 12] = R1;

    memory.buffer[subroutine_address as usize + 13] = MOV_LIT_REG;
    memory.buffer[subroutine_address as usize + 14] = 0x09;
    memory.buffer[subroutine_address as usize + 15] = 0x0a; // 0x090a
    memory.buffer[subroutine_address as usize + 16] = R8;

    memory.buffer[subroutine_address as usize + 17] = PSH_LIT;
    memory.buffer[subroutine_address as usize + 18] = 0x00;
    memory.buffer[subroutine_address as usize + 19] = 0x00; // 0x0000
    memory.buffer[subroutine_address as usize + 20] = CALL_LIT;
    memory.buffer[subroutine_address as usize + 21] = 0x00;
    memory.buffer[subroutine_address as usize + 22] = 0xFF; // 0x7000

    memory.buffer[subroutine_address as usize + 23] = RET;

    // Subroutine
    subroutine_address = 0x00FF;
    memory.buffer[subroutine_address as usize + 0] = PSH_LIT;
    memory.buffer[subroutine_address as usize + 1] = 0x99;
    memory.buffer[subroutine_address as usize + 2] = 0x99; // 0x9999

    memory.buffer[subroutine_address as usize + 3] = PSH_LIT;
    memory.buffer[subroutine_address as usize + 4] = 0x88;
    memory.buffer[subroutine_address as usize + 5] = 0x88; // 0x8888

    memory.buffer[subroutine_address as usize + 6] = PSH_LIT;
    memory.buffer[subroutine_address as usize + 7] = 0x77;
    memory.buffer[subroutine_address as usize + 8] = 0x77; // 0x7777

    memory.buffer[subroutine_address as usize + 9] = MOV_LIT_REG;
    memory.buffer[subroutine_address as usize + 10] = 0x78;
    memory.buffer[subroutine_address as usize + 11] = 0x79; // 0x7879
    memory.buffer[subroutine_address as usize + 12] = R6;

    memory.buffer[subroutine_address as usize + 13] = MOV_LIT_REG;
    memory.buffer[subroutine_address as usize + 14] = 0x7a;
    memory.buffer[subroutine_address as usize + 15] = 0x7b; // 0x7a7b
    memory.buffer[subroutine_address as usize + 16] = R7;

    memory.buffer[subroutine_address as usize + 17] = RET;

    memory.buffer[22 + 1] = PSH_LIT;
    memory.buffer[23 + 1] = 0x44;
    memory.buffer[24 + 1] = 0x44; // 0x4444

    // Create virtual machine
    let mut cpu = CPU::new(memory);

    // Variables
    let stack_debug_size = 64;

    // Run virtual machine
    cpu.view_memory(cpu.get_register("ip"), 16);
    cpu.view_memory(
        (cpu.memory.buffer.len() - stack_debug_size) as u16,
        stack_debug_size,
    );
    cpu.debug();
    loop {
        wait();
        cpu.step();
        cpu.view_memory(cpu.get_register("ip"), 16);
        cpu.view_memory(
            (cpu.memory.buffer.len() - stack_debug_size) as u16,
            stack_debug_size,
        );
        cpu.debug();
    }
}
