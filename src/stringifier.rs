use image::{DynamicImage, GenericImageView, Rgb};
use std::collections::HashMap;

use crate::art_generator::ArtAlgo;
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

        let current_nails = Stringifier::starting_nails(board, color_palette, dithered_img);

        Self {
            current_nails,
            remaining_paths: board.paths().clone(),
        }
    }

    fn starting_nails(
        board: &Board,
        color_palette: ColorPalette,
        dithered_img: DynamicImage,
    ) -> HashMap<Rgb<u8>, Nail> {
        let mut starting_nails = HashMap::new();
        let dithered_rgb = dithered_img.to_rgb8();

        for color in color_palette {
            let nails = board.nails();
            let paths = board.paths();
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

            if let Some(nails) = chosen_path {
                starting_nails.insert(*color, nails.0);
            }
        }
        starting_nails
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
