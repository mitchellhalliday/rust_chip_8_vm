use crate::ram::Ram;

use rand;

pub struct Cpu {
    vx: [u8; 16],
    stack: [u16; 16],
    sp: u16,
    pc: u16,
    i: u16,
    vf: u8,
    delay_timer: u8,
    sound_timer: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            vx: [0; 16],
            stack: [0; 16],
            sp: 0,
            pc: 0x0200,
            i: 0,
            vf: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn run_instruction(&mut self, ram: &mut Ram, display_buffer: &mut [u8; 2048]) {
        let hi = ram.read_byte(self.pc) as u16;
        let lo = ram.read_byte(self.pc + 1) as u16;

        let instruction = (hi << 8) | lo;

        if instruction == 0 {
            panic!("Instruction is 0");
        }

        let nnn = instruction & 0x0FFF;
        let nn = instruction & 0x00FF;
        let n = instruction & 0x000F;
        let x = (instruction & 0x0F00) >> 8;
        let y = (instruction & 0x00F0) >> 4;

        println!("Instruction {:x}", instruction);

        match instruction & 0xF000 {
            0x0000 => match nn {
                0x00E0 => {
                    panic!("Clear Display not implimented: {:X}", instruction);
                }
                0x00EE => {
                    self.pc = self.stack[self.sp as usize] + 2;
                    self.sp -= 1;
                }
                _ => {
                    panic!("Unknown Instruction: {:X}", instruction);
                }
            },
            0x1000 => {
                self.pc = nnn;
            }
            0x2000 => {
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                self.pc = nnn;
            }
            0x3000 => {
                if self.vx[x as usize] == nn as u8 {
                    self.pc += 2;
                }

                self.pc += 2;
            }
            0x4000 => {
                if self.vx[x as usize] != nn as u8 {
                    self.pc += 2;
                }

                self.pc += 2;
            }
            0x5000 => {
                if self.vx[x as usize] == self.vx[(y >> 8) as usize] {
                    self.pc += 2;
                }

                self.pc += 2;
            }
            0x6000 => {
                self.vx[x as usize] = nn as u8;
                self.pc += 2
            }
            0x7000 => {
                let sum = self.vx[x as usize] as u16 + nn as u16;

                if sum > 255 {
                    self.vx[x as usize] = (sum - 256) as u8;
                } else {
                    self.vx[x as usize] = sum as u8;
                }

                self.pc += 2
            }
            0x8000 => {
                match n {
                    0x0 => {
                        self.vx[x as usize] = self.vx[y as usize];
                    }
                    0x1 => {
                        self.vx[x as usize] = self.vx[x as usize] | self.vx[y as usize];
                    }
                    0x2 => {
                        self.vx[x as usize] = self.vx[x as usize] & self.vx[y as usize];
                    }
                    0x3 => {
                        self.vx[x as usize] = self.vx[x as usize] ^ self.vx[y as usize];
                    }
                    0x4 => {
                        let sum = self.vx[x as usize] as u16 + self.vx[y as usize] as u16;

                        if sum > 255 {
                            self.vx[x as usize] = (sum - 256) as u8;
                            self.vf = 1;
                        } else {
                            self.vx[x as usize] = sum as u8;
                            self.vf = 0;
                        }
                    }
                    0x5 => {
                        if self.vx[x as usize] > self.vx[y as usize] {
                            self.vf = 1;
                            self.vx[x as usize] -= self.vx[y as usize];
                        } else {
                            self.vf = 0;
                            self.vx[x as usize] =
                                (0xFF - (self.vx[y as usize] - self.vx[x as usize])) + 1;
                        }
                    }
                    0x6 => {
                        self.vf = self.vx[x as usize] & 0x1;

                        self.vx[x as usize] >>= 1;
                    }
                    0x7 => {
                        let int = self.vx[y as usize] as u16 - self.vx[x as usize] as u16;

                        self.vx[x as usize] = int as u8;

                        self.vf = if self.vx[y as usize] > self.vx[x as usize] {
                            1
                        } else {
                            0
                        };
                    }
                    0xe => {
                        self.vf = self.vx[x as usize] >> 7;

                        self.vx[x as usize] <<= 1;
                    }
                    _ => {
                        panic!("Unknown Instruction: {:X}", instruction);
                    }
                }

                self.pc += 2
            }
            0x9000 => {
                if self.vx[x as usize] != self.vx[y as usize] {
                    self.pc += 2;
                }

                self.pc += 2;
            }
            0xA000 => {
                self.i = nnn;
                self.pc += 2;
            }
            0xB000 => {
                self.pc = nnn + self.vx[0] as u16;
            }
            0xC000 => {
                self.vx[x as usize] = rand::random::<u8>() & nn as u8;
                self.pc += 2;
            }
            0xD000 => {
                for i in 0..n {
                    let byte = ram.read_byte(self.i + i);

                    for j in 0..8 {
                        let sprite_pixel = (byte >> (7 - j)) & 0x1;

                        let x = (self.vx[x as usize] as u16 + j) % 64;
                        let y = (self.vx[y as usize] as u16 + i) % 32;

                        let index = (y * 64 + x) as usize;

                        let screen_pixel = display_buffer[index];

                        display_buffer[index] = sprite_pixel ^ screen_pixel;

                        self.vf = if (sprite_pixel == 1) && (screen_pixel == 1) {
                            1
                        } else {
                            0
                        }
                    }
                }

                self.pc += 2;
            }
            0xE000 => match nn {
                0x9E => {
                    self.pc += 2;
                }
                0xA1 => {
                    self.pc += 4;
                }
                _ => {
                    panic!("Unknown Instruction: {:X}", instruction);
                }
            },
            0xF000 => match nn {
                0x07 => {
                    self.vx[x as usize] = self.delay_timer;

                    self.pc += 2;
                }
                0x0A => {
                    panic!("Not Implimented: {:X}", instruction);
                }
                0x15 => {
                    self.delay_timer = self.vx[x as usize];

                    self.pc += 2;
                }
                0x18 => {
                    self.sound_timer = self.vx[x as usize];

                    self.pc += 2;
                }
                0x1E => {
                    self.i += self.vx[x as usize] as u16;

                    self.pc += 2;
                }
                0x29 => {
                    self.i = self.vx[x as usize] as u16 * 5;

                    self.pc += 2;
                }
                0x33 => {
                    let vx = self.vx[x as usize];

                    ram.write_byte(self.i, vx / 100);
                    ram.write_byte(self.i + 1, (vx / 10) % 10);
                    ram.write_byte(self.i + 2, (vx % 100) % 10);

                    self.pc += 2;
                }
                0x55 => {
                    let x = x as usize;

                    for i in 0..(x + 1) {
                        ram.write_byte(self.i + i as u16, self.vx[i]);
                    }

                    self.pc += 2;
                }
                0x65 => {
                    let x = x as usize;

                    for i in 0..(x + 1) {
                        self.vx[i] = ram.read_byte(self.i + i as u16);
                    }

                    self.pc += 2;
                }
                _ => {
                    panic!("Unknown Instruction: {:X}", instruction);
                }
            },
            _ => {
                panic!("Unknown Instruction: {:X}", instruction);
            }
        }
    }

    pub fn decrement_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }
}
