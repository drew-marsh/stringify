use image::{DynamicImage, GenericImageView, Rgb};
use std::collections::HashMap;

use crate::art_generator::ArtAlgo;
use crate::board::NailNailPaths;
use crate::{
    board::{Board, Nail},
    image_utils::{dither_image},
    util::ColorPalette,
};

pub struct Stringifier {
    current_nails: HashMap<Rgb<u8>, Nail>,
    remaining_paths: NailNailPaths,
    current_image: image::ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl Stringifier {
    pub fn new(board: &Board, src_img: &DynamicImage, color_palette: ColorPalette) -> Self {
        let scaled_img = board.scale_image(src_img, None);
        let dithered_img = dither_image(&scaled_img, color_palette);

        let current_nails =
            Stringifier::starting_nails(board.nails(), board.paths(), color_palette, &dithered_img);

        Self {
            current_nails,
            remaining_paths: board.paths().clone(),
            current_image: dithered_img.to_rgb8(),
        }
    }

    fn starting_nails(
        nails: &Vec<Nail>,
        paths: &NailNailPaths,
        color_palette: ColorPalette,
        dithered_img: &DynamicImage,
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

impl ArtAlgo for Stringifier {
    fn current_nails(&self) -> &HashMap<Rgb<u8>, Nail> {
        &self.current_nails
    }
    fn choose_next_nail(&self) -> (Rgb<u8>, Nail) {
        let mut best_path: Option<(Rgb<u8>, Nail)> = None;
        let mut max_match = 0;

        for (color, nail) in &self.current_nails {
            let paths = self.remaining_paths.get(nail).unwrap();
            for (next_nail, path) in paths {
                let mut match_count = 0;
                for (x, y) in path {
                    let pixel_color = self.current_image.get_pixel(*x, *y);
                    if pixel_color == color {
                        match_count += 1;
                    }
                }
                if match_count > max_match {
                    max_match = match_count;
                    best_path = Some((*color, *next_nail));
                }
            }
        }

        best_path.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use image::RgbImage;

    use super::*;

    fn create_mock_board() -> (
        Vec<Nail>,
        NailNailPaths,
        image::ImageBuffer<Rgb<u8>, Vec<u8>>,
    ) {
        let nails = vec![Nail(0, 0), Nail(0, 3), Nail(3, 0)];
        let mut paths = HashMap::new();

        let mut from00 = HashMap::new();
        from00.insert(Nail(0, 3), vec![(0, 1), (0, 2)]);
        from00.insert(Nail(3, 0), vec![(1, 0), (2, 0)]);

        let mut from03 = HashMap::new();
        from03.insert(Nail(0, 0), vec![(0, 1), (0, 2)]);
        from03.insert(Nail(3, 0), vec![(1, 1), (2, 2)]);

        let mut from30 = HashMap::new();
        from30.insert(Nail(0, 0), vec![(1, 0), (2, 0)]);
        from30.insert(Nail(0, 3), vec![(1, 1), (2, 2)]);

        paths.insert(Nail(0, 0), from00);
        paths.insert(Nail(0, 3), from03);
        paths.insert(Nail(3, 0), from30);

        let img = RgbImage::from_fn(3, 3, |x, y| {
            if x < 2 {
                Rgb([255, 255, 255])
            } else if y < 2 {
                Rgb([127, 127, 127])
            } else {
                Rgb([0, 0, 0])
            }
        });

        (nails, paths, img)
    }

    #[test]
    fn test_choose_next_nail() {
        let (nails, paths, img) = create_mock_board();
        let color = Rgb([255, 255, 255]);

        let current_nails = HashMap::from([(color, Nail(0, 0))]);

        let stringifier = Stringifier {
            current_nails,
            remaining_paths: paths.clone(),
            current_image: img.clone(),
        };

        let next_nail = stringifier.choose_next_nail();

        assert_eq!(next_nail, (color, Nail(0, 3)));
    }

    #[test]
    fn test_choose_path() {
        let (nails, paths, img) = create_mock_board();
        let color = Rgb([255, 255, 255]);

        let chosen_path = Stringifier::choose_path(&nails, &paths, &img, &color);

        assert_eq!(chosen_path, Some((Nail(0, 0), Nail(0, 3))));
    }
}
