use std::{
    fs::File,
    io::{BufReader, Read},
};

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Chip8 {
    ram: Ram,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 { ram: Ram::new() }
    }

    pub fn load_rom(&mut self, path: &str) {
        let f = BufReader::new(File::open(path).unwrap());

        for (i, byte) in f.bytes().enumerate() {
            self.ram.write_byte((0x0200 + i) as u16, byte.unwrap());
        }
    }
}
