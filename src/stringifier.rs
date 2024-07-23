use image::{DynamicImage, GenericImageView, Rgb};
use std::collections::HashMap;

use crate::board::NailNailPaths;
use crate::{
    board::{Board, Nail},
    image_utils::{dither_image, get_color_masks},
    util::ColorPalette,
};

pub struct Stringifier {
    current_nails: HashMap<Rgb<u8>, Nail>,
    remaining_paths: NailNailPaths,
}

impl Stringifier {
    pub fn new(board: &Board, src_img: &DynamicImage, color_palette: ColorPalette) -> Self {
        let scaled_img = board.scale_image(src_img, None);
        let dithered_img = dither_image(&scaled_img, color_palette);

        let current_nails =
            Stringifier::starting_nails(board.nails(), board.paths(), color_palette, dithered_img);

        Self {
            current_nails,
            remaining_paths: board.paths().clone(),
        }
    }

    fn starting_nails(
        nails: &Vec<Nail>,
        paths: &NailNailPaths,
        color_palette: ColorPalette,
        dithered_img: DynamicImage,
    ) -> HashMap<Rgb<u8>, Nail> {
        let mut starting_nails = HashMap::new();
        let dithered_rgb = dithered_img.to_rgb8();

        for color in color_palette {
            let chosen_path = Stringifier::choose_path(nails, paths, &dithered_rgb, color);

            if let Some(nails) = chosen_path {
                starting_nails.insert(*color, nails.0);
            }
        }
        starting_nails
    }

    pub fn current_nails(&self) -> &HashMap<Rgb<u8>, Nail> {
        &self.current_nails
    }

    fn choose_path(
        nails: &Vec<Nail>,
        paths: &NailNailPaths,
        dithered_rgb: &image::ImageBuffer<Rgb<u8>, Vec<u8>>,
        color: &Rgb<u8>,
    ) -> Option<(Nail, Nail)> {
        let mut max_match = 0;
        let mut chosen_path = None;

        for i in 0..nails.len() {
            for j in i + 1..nails.len() {
                let start = nails[i];
                let end = nails[j];
                let path = paths.get(&start).unwrap().get(&end).unwrap();
                let mut match_count = 0;

                for (x, y) in path {
                    let pixel_color = dithered_rgb.get_pixel(*x, *y);
                    if pixel_color == color {
                        match_count += 1;
                    }
                }

                if match_count > max_match {
                    max_match = match_count;
                    chosen_path = Some((start, end));
                }
            }
        }
        chosen_path
    }
}

#[cfg(test)]
mod tests {
    use image::RgbImage;

    use super::*;

    #[test]
    fn test_choose_path() {
        let nails = vec![Nail(0, 0), Nail(0, 2), Nail(2, 0)];
        let mut paths = HashMap::new();

        let mut from00 = HashMap::new();
        from00.insert(Nail(0, 2), vec![(0, 1)]);
        from00.insert(Nail(2, 0), vec![(1, 0)]);

        let mut from02 = HashMap::new();
        from02.insert(Nail(0, 0), vec![(0, 1)]);
        from02.insert(Nail(2, 0), vec![(1, 1)]);

        let mut from20 = HashMap::new();
        from20.insert(Nail(0, 0), vec![(1, 0)]);
        from20.insert(Nail(0, 2), vec![(1, 1)]);

        paths.insert(Nail(0, 0), from00);
        paths.insert(Nail(0, 2), from02);
        paths.insert(Nail(2, 0), from20);

        let color = Rgb([255, 0, 0]);

        let dithered_rgb = RgbImage::from_fn(3, 3, |x, y| {
            if x == 0 && y == 1 {
                color
            } else {
                Rgb([0, 0, 0])
            }
        });

        let chosen_path = Stringifier::choose_path(&nails, &paths, &dithered_rgb, &color);

        assert_eq!(chosen_path, Some((Nail(0, 0), Nail(0, 2))));
    }
}
// impl ArtAlgo for Stringifier {
//     fn get_current_nails(&self) -> &HashMap<Rgb<u8>, Nail> {
//         &self.current_nails
//     }

//     fn choose_next_nail(&self) -> (Rgb<u8>, Nail) {
//         self.remaining_paths
//     }
// }
