use image::{DynamicImage, Rgb};
use std::collections::HashMap;

use crate::{
    board::{Board, Nail},
    image_utils::{dither_image, get_color_masks},
    util::{ArtAlgo, ColorPalette},
};

struct ColorState {
    nail: Option<Nail>,
    mask: Vec<Vec<bool>>,
}

pub struct Stringifier {
    color_states: HashMap<Rgb<u8>, ColorState>,
    remaining_paths: HashMap<(Nail, Nail), Vec<(u32, u32)>>,
}

impl Stringifier {
    pub fn new(board: &Board, src_img: &DynamicImage, color_palette: ColorPalette) -> Self {
        let scaled_img = board.scale_image(src_img, None);
        let dithered_img = dither_image(&scaled_img, color_palette);
        let color_masks = get_color_masks(&dithered_img, color_palette);

        let color_states = color_palette.iter().fold(HashMap::new(), |mut acc, color| {
            let mask = color_masks.get(color).unwrap();
            acc.insert(
                *color,
                ColorState {
                    nail: None,
                    mask: mask.clone(),
                },
            );
            acc
        });

        Self {
            color_states,
            remaining_paths: board.getPaths().clone(),
        }
    }
}
