use crate::memory::Memory;

// Region class for memory-mapper
pub struct Region {
    device: Memory,
    start: u16,
    end: u16,
    remap: bool,
}

// memory-mapper class for io management
pub struct MemoryMapper {
    regions: Vec<Region>,
}

// Logic for the memory-mapper
impl MemoryMapper {
    // Create a new memory-mapper
    pub fn new() -> Self {
        Self {
            regions: Vec::new(),
        }
    }

    // Add a region to the memory-mapper
    pub fn map(&mut self, device: Memory, start: u16, end: u16, remap: bool) {
        self.regions.push(Region {
            device,
            start,
            end,
            remap,
        });
    }

    // Write a function to remove regions from the memory-mapper
    // TODO: Implement this function

    // Write a byte to the memory-mapper
    pub fn set_byte(&mut self, data: u8, address: u16) {
        // Check if the address is in a region
        for region in self.regions.iter_mut() {
            if address >= region.start && address <= region.end {
                let final_address = if region.remap {
                    address - region.start
                } else {
                    address
                };
                region.device.set_byte(data, final_address as usize);
                return;
            }
        }

        // If the index is not in a region, panic
        panic!("Index out of bounds");
    }

    // Read a byte from the memory-mapper
    pub fn get_byte(&self, address: u16) -> u8 {
        // Check if the index is in a region
        for region in self.regions.iter() {
            if address >= region.start && address <= region.end {
                let final_address = if region.remap {
                    address - region.start
                } else {
                    address
                };
                return region.device.get_byte(final_address as usize);
            }
        }
        panic!("Index out of bounds");
    }

    // Find region by address
    pub fn mut_find_region(&mut self, address: u16) -> &mut Region {
        for region in self.regions.iter_mut() {
            if address >= region.start && address <= region.end {
                return region;
            }
        }

        panic!("Address 0x{:04X} not found in any region", address);
    }

    // Find region by address
    pub fn find_region(&self, address: u16) -> &Region {
        for region in self.regions.iter() {
            if address >= region.start && address <= region.end {
                return region;
            }
        }

        panic!("Address 0x{:04X} not found in any region", address);
    }

    // Read a bytes from the memory-mapper
    pub fn get_uint_16(&self, address: u16) -> u16 {
        let region = self.find_region(address);
        let final_address = if region.remap {
            address - region.start
        } else {
            address
        };

        let bytes = [
            region.device.get_byte(final_address as usize),
            region.device.get_byte((final_address + 1) as usize),
        ];

        u16::from_be_bytes(bytes)
    }

    // Read a byte from the memory-mapper
    pub fn get_uint_8(&self, address: u16) -> u8 {
        let region = self.find_region(address);
        let final_address = if region.remap {
            address - region.start
        } else {
            address
        };

        region.device.get_byte(final_address as usize)
    }

    // Write a bytes to the memory-mapper
    pub fn set_uint_16(&mut self, address: u16, value: u16) {
        let region = self.mut_find_region(address);
        let final_address = if region.remap {
            address - region.start
        } else {
            address
        };

        let bytes = value.to_be_bytes();
        region.device.set_byte(bytes[0], final_address as usize);
        region
            .device
            .set_byte(bytes[1], (final_address + 1) as usize);
    }

    // Write a byte to the memory-mapper
    pub fn set_uint_8(&mut self, address: u16, value: u8) {
        let region = self.mut_find_region(address);
        let final_address = if region.remap {
            address - region.start
        } else {
            address
        };

        region.device.set_byte(value, final_address as usize);
    }

    // Print all content of memory in given address range
    pub fn view_memory(&self, address: u16, size: usize) {
        let region = self.find_region(address);
        let final_address = if region.remap {
            address - region.start
        } else {
            address
        };

        print!("0x{:04X}: ", address);
        for i in 0..size {
            print!(
                "0x{:02X} ",
                region.device.get_byte(final_address as usize + i)
            );
        }
        println!("");
    }
}
