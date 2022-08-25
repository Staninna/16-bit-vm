use crate::memory::Memory;

// Region class
pub struct Region {
    device: Memory,
    start: u16,
    end: u16,
    remap: bool,
}

// MemoryMapper class
pub struct MemoryMapper {
    regions: Vec<Region>,
}

// MemoryMapper implementation
impl MemoryMapper {
    pub fn new() -> Self {
        Self {
            regions: Vec::new(),
        }
    }

    // Map a memory to a region
    pub fn map(&mut self, device: Memory, start: u16, end: u16, remap: bool) {
        self.regions.push(Region {
            device,
            start,
            end,
            remap,
        });
    }

    // TODO: Write a function to remove regions from the memory-mapper

    // Write a byte
    pub fn set_byte(&mut self, data: u8, address: u16) {
        // Find address in region
        for region in self.regions.iter_mut() {
            if address >= region.start && address <= region.end {
                // Remap the address if needed
                let final_address = if region.remap {
                    address - region.start
                } else {
                    address
                };

                // Write byte
                region.device.set_byte(data, final_address as usize);
                return;
            }
        }

        // If address out of bounds, panic
        panic!("Index out of bounds");
    }

    // Read a byte
    pub fn get_byte(&self, address: u16) -> u8 {
        // Find address is region
        for region in self.regions.iter() {
            if address >= region.start && address <= region.end {
                // Remap the address if needed
                let final_address = if region.remap {
                    address - region.start
                } else {
                    address
                };

                // Read byte
                return region.device.get_byte(final_address as usize);
            }
        }

        // If address out of bounds, panic
        panic!("Index out of bounds");
    }

    // Find region by address
    pub fn mut_find_region(&mut self, address: u16) -> &mut Region {
        // Find address in region
        for region in self.regions.iter_mut() {
            if address >= region.start && address <= region.end {
                return region;
            }
        }

        // If address out of bounds, panic
        panic!("Address 0x{:04X} not found in any region", address);
    }

    // Find region by address
    pub fn find_region(&self, address: u16) -> &Region {
        // Find address in region
        for region in self.regions.iter() {
            if address >= region.start && address <= region.end {
                return region;
            }
        }

        // If address out of bounds, panic
        panic!("Address 0x{:04X} not found in any region", address);
    }

    // Read bytes
    pub fn get_uint_16(&self, address: u16) -> u16 {
        let region = self.find_region(address);

        // Remap the address if needed
        let final_address = if region.remap {
            address - region.start
        } else {
            address
        };

        // Read bytes
        let bytes = [
            region.device.get_byte(final_address as usize),
            region.device.get_byte((final_address + 1) as usize),
        ];
        u16::from_be_bytes(bytes)
    }

    // Read a byte
    pub fn get_uint_8(&self, address: u16) -> u8 {
        let region = self.find_region(address);

        // Remap the address if needed
        let final_address = if region.remap {
            address - region.start
        } else {
            address
        };

        // Read byte
        region.device.get_byte(final_address as usize)
    }

    // Write bytes
    pub fn set_uint_16(&mut self, address: u16, value: u16) {
        let region = self.mut_find_region(address);

        // Remap the address if needed
        let final_address = if region.remap {
            address - region.start
        } else {
            address
        };

        // Write bytes
        let bytes = value.to_be_bytes();
        region.device.set_byte(bytes[0], final_address as usize);
        region
            .device
            .set_byte(bytes[1], (final_address + 1) as usize);
    }

    // Write a byte
    pub fn set_uint_8(&mut self, address: u16, value: u8) {
        let region = self.mut_find_region(address);

        // Remap the address if needed
        let final_address = if region.remap {
            address - region.start
        } else {
            address
        };

        // Write byte
        region.device.set_byte(value, final_address as usize);
    }

    // Print/read bytes in given address range
    pub fn view_memory(&self, address: u16, size: usize) {
        let region = self.find_region(address);

        // Remap the address if needed
        let final_address = if region.remap {
            address - region.start
        } else {
            address
        };

        // Print and read bytes
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
