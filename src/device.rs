// Device types
pub enum DeviceType {
    Memory,
    Stdout,
}

// Stdout codes
pub const STDOUT_CLEAR: u8 = 0xFF;
pub const STDOUT_BOLD: u8 = 0x01;
pub const STDOUT_REGULAR: u8 = 0xF2;

// Memory class
pub struct Device {
    buffer: Vec<u8>,
    pub length: u16,
    device_type: DeviceType,
}

// Memory implementation
impl Device {
    pub fn new(length: usize, device_type: DeviceType) -> Self {
        Self {
            buffer: vec![0x00; length],
            length: length as u16,
            device_type,
        }
    }

    // Write a byte to device
    pub fn set_byte(&mut self, data: u8, address: usize) {
        match self.device_type {
            DeviceType::Memory => {
                self.buffer[address] = data;
            }
            DeviceType::Stdout => {
                match data {
                    // Clear screen
                    STDOUT_CLEAR => {
                        print!("\x1B[2J");
                        return;
                    }
                    // Bold text
                    STDOUT_BOLD => {
                        print!("\x1B[1m");
                        return;
                    }
                    // Regular text
                    STDOUT_REGULAR => {
                        print!("\x1B[0m");
                        return;
                    }
                    _ => (),
                }

                let character_raw = data & 0xFF;

                let x = ((address % 16) + 1) * 2;
                let y = address / 16;
                self.move_to(x, y);

                let character = String::from_utf16(&vec![character_raw as u16]).unwrap();
                print!("{}", character);
            }
        }
    }

    // Read a byte from device
    pub fn get_byte(&self, address: usize) -> u8 {
        match self.device_type {
            DeviceType::Memory => self.buffer[address],
            DeviceType::Stdout => 0x00,
        }
    }

    // Move cursor to x, y on stdout
    pub fn move_to(&self, x: usize, y: usize) {
        match self.device_type {
            DeviceType::Stdout => {
                print!("\x1B[{};{}H", y, x);
            }
            _ => {}
        }
    }
}
