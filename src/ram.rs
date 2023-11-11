pub struct Ram {
    memory: [u8; 4096],
}

impl Ram {
    pub fn new() -> Ram {
        Ram { memory: [0; 4096] }
    }

    pub fn write_byte(&mut self, address: u16, byte: u8) {
        self.memory[address as usize] = byte;
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}
