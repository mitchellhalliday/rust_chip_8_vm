mod chip8;
mod cpu;
mod display;
mod ram;

fn main() {
    let mut chip8 = chip8::Chip8::new();

    chip8.load_rom("./roms/pong");

    chip8.run();
}
