use std::time;

use minifb::{Key, Window, WindowOptions};

pub struct Gui {
    buffer: Vec<u32>,
    window: Window,
}

impl Gui {
    pub const WIDTH: usize = 32;
    pub const HEIGHT: usize = 32;

    pub fn new() -> Self {
        Self {
            buffer: vec![0; Self::WIDTH * Self::HEIGHT],
            window: Window::new(
                "Frame Buffer",
                Self::WIDTH,
                Self::HEIGHT,
                WindowOptions {
                    scale: minifb::Scale::X8,
                    scale_mode: minifb::ScaleMode::AspectRatioStretch,
                    title: false,
                    borderless: false,
                    resize: false,
                    topmost: false,
                    transparency: false,
                },
            )
            .unwrap(),
        }
    }

    pub fn init(&mut self) {
        self.buffer.iter_mut().enumerate().for_each(|(i, x)| *x = i as u32);
        self.window.update_with_buffer(&mut self.buffer, Self::WIDTH, Self::HEIGHT);
    }

    pub fn update_fb(&mut self, vmem: &[u8]) {
        self.window.update_with_buffer(&mut self.buffer, Self::WIDTH, Self::HEIGHT);
    }

    #[inline]
    pub fn is_window_open(&self) -> bool {
        self.window.is_open()
    }

    #[inline]
    pub fn is_key_down(&self, key: Key) -> bool {
        self.window.is_key_down(key)
    }
}
