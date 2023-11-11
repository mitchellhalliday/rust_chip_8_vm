use crate::cpu::Cpu;
use crate::display::Display;
use crate::ram::Ram;

use std::{
    fs::File,
    io::{BufReader, Read},
};

pub struct Chip8 {
    ram: Ram,
    cpu: Cpu,
    display: Display,
}

const SPRITES: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut ram = Ram::new();

        for (i, byte) in SPRITES.iter().enumerate() {
            ram.write_byte(i as u16, *byte);
        }

        Chip8 {
            ram,
            cpu: Cpu::new(),
            display: Display::new(),
        }
    }

    pub fn load_rom(&mut self, path: &str) {
        let f = BufReader::new(File::open(path).unwrap());

        for (i, byte) in f.bytes().enumerate() {
            self.ram.write_byte((0x0200 + i) as u16, byte.unwrap());
        }
    }

    pub fn run(&mut self) {
        let mut display_buffer: [u8; 2048] = [0; 2048];

        loop {
            self.cpu.run_instruction(&mut self.ram, &mut display_buffer);

            self.cpu.decrement_timers();

            self.display.draw(display_buffer);

            // std::thread::sleep(std::time::Duration::from_micros(1));
        }
    }
}
