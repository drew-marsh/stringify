use image::Rgb;

pub type ColorPalette<'a> = &'a [Rgb<u8>];

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Dimensions {
    width: u32,
    height: u32,
}

impl Dimensions {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
