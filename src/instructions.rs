#![allow(dead_code)]
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
