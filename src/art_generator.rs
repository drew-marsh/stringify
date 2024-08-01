use std::{collections::HashMap, rc::Rc};

use image::{GenericImage, GenericImageView, Pixel, Rgb, Rgba};

use crate::{
    art_algo::ArtAlgo,
    board::{Board, Nail},
};

type NailPattern = Vec<(Rgb<u8>, Nail)>;

pub struct ArtGenerator {
    board: Rc<Board>,
    algo: Box<dyn ArtAlgo>,
    current_nails: HashMap<Rgb<u8>, Nail>,
    pattern: NailPattern,
    art: image::DynamicImage,
}

impl ArtGenerator {
    pub fn new(board: Rc<Board>, algo: Box<dyn ArtAlgo>) -> Self {
        let nails = algo.initial_nails();

        let pattern: NailPattern = nails.iter().map(|(color, nail)| (*color, *nail)).collect();

        let art =
            image::DynamicImage::new_rgba8(board.dimensions().width(), board.dimensions().height());

        Self {
            board,
            algo,
            current_nails: nails,
            pattern,
            art,
        }
    }

    pub fn step(&mut self) -> Option<(Rgb<u8>, Nail)> {
        // TODO get rid of this clone
        let nail_choice = self.algo.next_nail(&self.current_nails);

        if nail_choice.is_none() {
            return None;
        }

        let (color, next_nail) = nail_choice.unwrap();
        let last_nail = *self.current_nails.get(&color).unwrap();

        self.paint_path(last_nail, next_nail, color);

        self.pattern.push((color, next_nail));
        self.current_nails.insert(color, next_nail);

        nail_choice
    }

    fn paint_path(&mut self, last_nail: Nail, next_nail: Nail, color: Rgb<u8>) {
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
