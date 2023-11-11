use minifb::{Window, WindowOptions};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    window: Window,
}

impl Display {
    pub fn new() -> Display {
        let window_options = WindowOptions {
            borderless: false,
            title: true,
            resize: false,
            topmost: false,
            none: false,
            scale: minifb::Scale::X8,
            scale_mode: minifb::ScaleMode::Stretch,
            transparency: false,
        };

        let window = Window::new("Test - ESC to exit", WIDTH, HEIGHT, window_options)
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });

        // Limit to max ~60 fps update rate
        // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        Display { window }
    }

    pub fn draw(&mut self, buffer: [u8; 2048]) {
        let mut pixel_buffer: [u32; 2048] = [0; 2048];

        for (i, v) in buffer.iter().enumerate() {
            if *v == 0x0 {
                pixel_buffer[i] = 0x000000;
            } else {
                pixel_buffer[i] = 0xFFFFFF;
            }
        }

        self.window
            .update_with_buffer(&pixel_buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
