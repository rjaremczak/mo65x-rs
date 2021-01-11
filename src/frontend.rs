use crate::{error::Result, mos6510::memory::Memory};
use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};

pub struct Frontend {
    next_update: Instant,
    framebuf: Vec<u32>,
    window: Window,
}

const WIDTH: usize = 32;
const HEIGHT: usize = 32;
const FB_ADDR: u16 = 0x200;
const FB_LEN: usize = WIDTH * HEIGHT;
const UPDATE_PERIOD: Duration = Duration::from_millis(20);

impl Frontend {
    pub fn new() -> Self {
        let mut frontend = Self {
            next_update: Instant::now(),
            framebuf: vec![0; FB_LEN],
            window: Window::new(
                &format!("{:04X} {:.1} fps", FB_ADDR, 1.0 / UPDATE_PERIOD.as_secs_f64()),
                WIDTH,
                HEIGHT,
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
        // frontend.window.limit_update_rate(Some(Duration::from_millis(20)));
        frontend
    }

    pub fn quit(&self) -> bool {
        !self.is_window_open() || self.is_key_down(Key::Escape)
    }

    pub fn vsync(&mut self) {
        while Instant::now() < self.next_update {}
        self.next_update = Instant::now() + UPDATE_PERIOD;
    }

    pub fn update(&mut self, memory: &Memory) -> Result<()> {
        let vmem = memory.view(FB_ADDR, FB_LEN);
        for i in 0..FB_LEN {
            self.framebuf[i] = C64_PALETTE[vmem[i] as usize & 0x0f];
        }
        self.window.update_with_buffer(&mut self.framebuf, WIDTH, HEIGHT)?;
        Ok(())
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
