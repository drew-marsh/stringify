use bresenham::Bresenham;
use image::{imageops::FilterType, DynamicImage};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Board {
    width: u32,
    height: u32,
    nails: Vec<(u32, u32)>,
    paths: HashMap<((u32, u32), (u32, u32)), Vec<(u32, u32)>>,
}

impl Board {
    pub fn new(nail_spacing_pixels: u32, nail_count: u32) -> Self {
        assert!(nail_spacing_pixels > 0);
        assert!(nail_count > 1);

        let circumference = nail_spacing_pixels * nail_count;
        let diameter = (circumference as f64 / std::f64::consts::PI).round() as u32;

        let nails = place_nails(diameter, nail_count);
        let paths = precompute_paths(&nails);

        Self {
            width: diameter,
            height: diameter,
            nails,
            paths,
        }
    }

    pub fn scale_image(&self, img: &DynamicImage, filter: Option<FilterType>) -> DynamicImage {
        let filter = if (filter.is_some()) {
            filter.unwrap()
        } else {
            FilterType::Lanczos3
        };

        img.resize_to_fill(self.width, self.height, filter)
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

fn place_nails(diameter: u32, nail_count: u32) -> Vec<(u32, u32)> {
    let origin = (diameter / 2, diameter / 2);
    let spacing = 2.0 * std::f64::consts::PI / nail_count as f64;

    let nails = (0..nail_count)
        .map(|i| {
            let x = (origin.0 as f64 + (diameter as f64 / 2.0) * (spacing * i as f64).cos()).round()
                as u32;
            let y = (origin.1 as f64 + (diameter as f64 / 2.0) * (spacing * i as f64).sin()).round()
                as u32;
            (x, y)
        })
        .collect::<Vec<_>>();

    nails
}

fn precompute_paths(nails: &[(u32, u32)]) -> HashMap<((u32, u32), (u32, u32)), Vec<(u32, u32)>> {
    let mut paths = HashMap::new();

    for i in 0..nails.len() {
        for j in (i + 1)..nails.len() {
            let start = nails[i];
            let end = nails[j];

            let bresenham = Bresenham::new(
                (start.0 as isize, start.1 as isize),
                (end.0 as isize, end.1 as isize),
            );

            let path = bresenham
                .map(|t| (t.0 as u32, t.1 as u32))
                .collect::<Vec<_>>();

            paths.insert((end, start), path.clone());
            paths.insert((start, end), path);
        }
    }

    paths
}

// // not used yet
// fn place_nails_rectangle(width: u32, height: u32, nail_spacing_pixels: u32) -> Vec<(u32, u32)> {
//     let mut nails: Vec<(u32, u32)> = Vec::new();

//     for x in (0..=width - nail_spacing_pixels).step_by(nail_spacing_pixels as usize) {
//         nails.push((x, 0));
//     }
//     for y in (0..=height - nail_spacing_pixels).step_by(nail_spacing_pixels as usize) {
//         nails.push((width, y));
//     }
//     for x in (0..=width - nail_spacing_pixels)
//         .rev()
//         .step_by(nail_spacing_pixels as usize)
//     {
//         nails.push((x, height));
//     }
//     for y in (0..=height - nail_spacing_pixels)
//         .rev()
//         .step_by(nail_spacing_pixels as usize)
//     {
//         nails.push((0, y));
//     }
//     nails
// }
// // not used yet
// fn new_rectangle(
//     src_img: &ImageBuffer<Rgb<u8>, Vec<u8>>,
//     nail_spacing_pixels: u32,
//     nail_count: u32,
// ) -> Self {
//     assert!(nail_spacing_pixels > 0);
//     assert!(nail_count > 1);
//     assert!(nail_count % 2 == 0);

//     let width_ratio = src_img.width() as f64 / (src_img.width() + src_img.height()) as f64;
//     let nails_wide = (nail_count as f64 * width_ratio / 2.0).ceil() as u32;
//     let nails_tall = (nail_count - nails_wide * 2) / 2;

//     let width = nails_wide * nail_spacing_pixels;
//     let height = nails_tall * nail_spacing_pixels;

//     let nails = place_nails_rectangle(width, height, nail_spacing_pixels);

//     Self {
//         width,
//         height,
//         nails,
//     }
// }
