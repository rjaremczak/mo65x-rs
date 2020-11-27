use minifb::{Window, WindowOptions};

pub struct Video {
    buffer: Vec<u32>,
    window: Window,
}

impl Video {
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

    pub fn update_fb(&mut self, vmem: &[u8; Self::WIDTH * Self::HEIGHT]) {}
}
