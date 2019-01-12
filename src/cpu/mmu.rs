pub struct Mmu {
    memory: [u8; Mmu::TOTAL_MEMORY_SIZE],
}

impl Mmu {
    const TOTAL_MEMORY_SIZE: usize = 0xFFFF;

    pub fn new() -> Mmu {
        Mmu {
            memory: [0; Mmu::TOTAL_MEMORY_SIZE],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}
