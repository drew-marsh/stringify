use image::{ImageBuffer, Rgb};

#[derive(Debug)]
pub struct Board {
    width: u32,
    height: u32,
}

impl Board {
    pub fn new(
        src_img: &ImageBuffer<Rgb<u8>, Vec<u8>>,
        nail_spacing_pixels: u32,
        nail_count: u32,
    ) -> Self {
        assert!(nail_spacing_pixels > 0);
        assert!(nail_count > 0);
        assert!(nail_count % 2 == 0);

        let width_ratio = src_img.width() as f64 / (src_img.width() + src_img.height()) as f64;
        let nails_wide = (nail_count as f64 * width_ratio / 2.0).ceil() as u32;
        let nails_tall = (nail_count - nails_wide * 2) / 2;

        Self {
            width: nails_wide * nail_spacing_pixels,
            height: nails_tall * nail_spacing_pixels,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
