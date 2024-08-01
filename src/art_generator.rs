use std::rc::Rc;

use image::{GenericImage, GenericImageView, Pixel, Rgb, Rgba};

use crate::{
    art_algo::ArtAlgo,
    board::{Board, Nail},
};

type NailPattern = Vec<(Rgb<u8>, Nail)>;

pub struct ArtGenerator {
    board: Rc<Board>,
    algo: Box<dyn ArtAlgo>,
    pattern: NailPattern,
    art: image::DynamicImage,
}

impl ArtGenerator {
    pub fn new(board: Rc<Board>, algo: Box<dyn ArtAlgo>) -> Self {
        let pattern: NailPattern = algo
            .current_nails()
            .iter()
            .map(|(color, nail)| (*color, *nail))
            .collect();

        let art =
            image::DynamicImage::new_rgba8(board.dimensions().width(), board.dimensions().height());

        Self {
            board,
            algo,
            pattern,
            art,
        }
    }

    pub fn step(&mut self) -> Option<(Rgb<u8>, Nail)> {
        // TODO get rid of this clone
        let current_nails = self.algo.current_nails().clone();
        let nail_choice = self.algo.choose_next_nail();

        if nail_choice.is_none() {
            return None;
        }

        let (color, next_nail) = nail_choice.unwrap();

        self.pattern.push((color, next_nail));

        let last_nail = *current_nails.get(&color).unwrap();
        let path = self
            .board
            .paths()
            .get(&last_nail)
            .unwrap()
            .get(&next_nail)
            .unwrap();

        for (x, y) in path {
            if self.art.get_pixel(*x, *y) == Rgba([0, 0, 0, 0]) {
                self.art.put_pixel(*x, *y, color.to_rgba());
            }
        }

        nail_choice
    }

    pub fn pattern(&self) -> &NailPattern {
        &self.pattern
    }

    pub fn art(&self) -> &image::DynamicImage {
        &self.art
    }

    pub fn board(&self) -> &Board {
        &self.board
    }
}
