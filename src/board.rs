use image::{ImageBuffer, Rgb};

#[derive(Debug)]
pub struct Board {
    width: u32,
    height: u32,
    nails: Vec<(u32, u32)>,
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

        let width = nails_wide * nail_spacing_pixels;
        let height = nails_tall * nail_spacing_pixels;

        let nails = place_nails(width, height, nail_spacing_pixels);

        Self {
            width,
            height,
            nails,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

fn place_nails(width: u32, height: u32, nail_spacing_pixels: u32) -> Vec<(u32, u32)> {
    let mut nails: Vec<(u32, u32)> = Vec::new();

    for x in (0..=width - nail_spacing_pixels).step_by(nail_spacing_pixels as usize) {
        nails.push((x, 0));
    }
    for y in (0..=height - nail_spacing_pixels).step_by(nail_spacing_pixels as usize) {
        nails.push((width, y));
    }
    for x in (0..=width - nail_spacing_pixels)
        .rev()
        .step_by(nail_spacing_pixels as usize)
    {
        nails.push((x, height));
    }
    for y in (0..=height - nail_spacing_pixels)
        .rev()
        .step_by(nail_spacing_pixels as usize)
    {
        nails.push((0, y));
    }
    nails
}
