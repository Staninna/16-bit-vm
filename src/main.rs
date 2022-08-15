mod cpu;
use cpu::{create_memory::create_memory, instructions::*, *};

// TODO DEBUG
fn wait() {
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
fn main() {
    let mut memory = create_memory(256 * 256);

    // Load program to memory

    // Move memory address 0x0100 to register 1
    memory.buffer[0] = MOV_MEM_REG;
    memory.buffer[1] = 0x01;
    memory.buffer[2] = 0x00; // address: 0x0100
    memory.buffer[3] = 0x02; // register: r1

    // Move value to register 2
    memory.buffer[4] = MOV_LIT_REG;
    memory.buffer[5] = 0x00;
    memory.buffer[6] = 0x01; // value: 0x0001
    memory.buffer[7] = 0x03; // register: r2

    // Add register 1 to register 2
    memory.buffer[8] = ADD_REG_REG;
    memory.buffer[9] = 0x02; // register: r1
    memory.buffer[10] = 0x03; // register: r2

    // Move result to memory at address 0x0100
    memory.buffer[11] = MOV_REG_MEM;
    memory.buffer[12] = 0x01; // register: acc
    memory.buffer[13] = 0x01;
    memory.buffer[14] = 0x00; // address: 0x0100

    // If result is not equal to 0x0005 then jump to 0x0000
    memory.buffer[15] = JMP_NOT_EQ;
    memory.buffer[16] = 0x00;
    memory.buffer[17] = 0x05; // value: 0x0005
    memory.buffer[18] = 0x00;
    memory.buffer[19] = 0x00; // address: 0x0000

    // Create virtual machine
    let mut cpu = CPU::new(memory);

    // Run virtual machine
    loop {
        cpu.view_memory(cpu.get_register("ip"), 8);
        cpu.view_memory(0x0100, 8);
        cpu.debug();
        cpu.step();
        wait();
    }
}
