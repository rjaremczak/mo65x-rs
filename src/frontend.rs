use crate::mos6510::memory::Memory;
use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};

pub struct Frontend {
    next_update: Instant,
    framebuf_addr: u16,
    framebuf: Vec<u32>,
    window: Window,
}

impl Frontend {
    pub const WIDTH: usize = 32;
    pub const HEIGHT: usize = 32;
    pub const FB_LEN: usize = Self::WIDTH * Self::HEIGHT;
    pub const UPDATE_PERIOD: Duration = Duration::from_millis(20);

    pub fn new() -> Self {
        let mut frontend = Self {
            next_update: Instant::now(),
            framebuf_addr: 0x200,
            framebuf: vec![0; Self::FB_LEN],
            window: Window::new(
                "Frame Buffer",
                Self::WIDTH,
                Self::HEIGHT,
                WindowOptions {
                    scale: minifb::Scale::X8,
                    scale_mode: minifb::ScaleMode::AspectRatioStretch,
                    title: true,
                    borderless: false,
                    resize: false,
                    topmost: false,
                    transparency: false,
                },
            )
            .unwrap(),
        };
        frontend.framebuf.iter_mut().enumerate().for_each(|(i, x)| *x = i as u32);
        // frontend.window.limit_update_rate(None);
        frontend
    }

    pub fn quit(&self) -> bool {
        !self.is_window_open() || self.is_key_down(Key::Escape)
    }

    pub fn update(&mut self, memory: &Memory) {
        while Instant::now() < self.next_update {}
        self.next_update = Instant::now() + Self::UPDATE_PERIOD;
        let vmem = memory.view(self.framebuf_addr, Self::FB_LEN);
        for i in 0..Self::FB_LEN {
            self.framebuf[i] = C64_PALETTE[vmem[i] as usize & 0x0f];
        }
        self.window
            .update_with_buffer(&mut self.framebuf, Self::WIDTH, Self::HEIGHT)
            .unwrap();
    }

    #[inline]
    fn is_window_open(&self) -> bool {
        self.window.is_open()
    }

    #[inline]
    fn is_key_down(&self, key: Key) -> bool {
        self.window.is_key_down(key)
    }
}

const C64_PALETTE: [u32; 16] = [
    0xFF000000, 0xFFFFFFFF, 0xFF880000, 0xFFAAFFEE, 0xFFCC44CC, 0xFF00CC55, 0xFF0000AA, 0xFFEEEE77, 0xFFDD8855, 0xFF664400, 0xFFFF7777,
    0xFF333333, 0xFF777777, 0xFFAAFF66, 0xFF0088FF, 0xFFBBBBBB,
];
