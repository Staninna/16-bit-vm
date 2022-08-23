use crate::memory::Memory;

// Region class for io-mapper
#[derive(PartialEq)]
pub struct Region {
    start: u16,
    end: u16,
    remap: bool,
}

// Io-mapper class for io management
pub struct MemoryMapper {
    device: Memory,
    regions: Vec<Region>,
}

// Logic for the Io-mapper
impl MemoryMapper {
    // Create a new Io-mapper
    pub fn new(device: Memory) -> Self {
        Self {
            device,
            regions: Vec::new(),
        }
    }

    // Add a region to the Io-mapper
    pub fn map(&mut self, start: u16, end: u16, remap: bool) {
        // Check if device is long enough to hold the region using start and end
        if end - start > self.device.length as u16 {
            panic!("Device is not long enough to hold the region");
        }

        // Add the region to the io-mapper
        self.regions.push(Region { start, end, remap });
    }

    // Remove mapped region from memory
    pub fn un_map(&mut self, region: &Region) {
        self.regions
            .remove(self.regions.iter().position(|r| r == region).unwrap());
    }

    // Write a byte to the io-mapper
    pub fn set_byte(&mut self, data: u8, index: u16) {
        // Check if the index is in a region
        for region in self.regions.iter() {
            if index >= region.start && index <= region.end {
                self.device.set_byte(data, index as usize);
                return;
            }
        }

        // If the index is not in a region, panic
        panic!("Index out of bounds");
    }

    // Read a byte from the io-mapper
    pub fn get_byte(&self, address: u16) -> u8 {
        // Check if the index is in a region
        for region in self.regions.iter() {
            if address >= region.start && address <= region.end {
                return self.device.get_byte(address as usize);
            }
        }
        panic!("Index out of bounds");
    }

    // Find region by address
    pub fn find_region(&self, address: u16) -> &Region {
        for region in self.regions.iter() {
            if address >= region.start && address <= region.end {
                return region;
            }
        }

        panic!("Address not found in any region");
    }

    pub fn get_uint_16(&self, address: u16) -> u16 {
        let region = self.find_region(address);
        let final_address = if region.remap {
            address - region.start
        } else {
            address
        };

        let bytes = [
            self.device.get_byte(final_address as usize),
            self.device.get_byte((final_address + 1) as usize),
        ];

        u16::from_be_bytes(bytes)
    }

    pub fn get_uint_8(&self, address: u16) -> u8 {
        let region = self.find_region(address);
        let final_address = if region.remap {
            address - region.start
        } else {
            address
        };

        self.device.get_byte(final_address as usize)
    }

    pub fn set_uint_16(&mut self, address: u16, value: u16) {
        let region = self.find_region(address);
        let final_address = if region.remap {
            address - region.start
        } else {
            address
        };

        let bytes = value.to_be_bytes();
        self.device.set_byte(bytes[0], final_address as usize);
        self.device.set_byte(bytes[1], (final_address + 1) as usize);
    }

    pub fn set_uint_8(&mut self, address: u16, value: u8) {
        let region = self.find_region(address);
        let final_address = if region.remap {
            address - region.start
        } else {
            address
        };

        self.device.set_byte(value, final_address as usize);
    }
}
