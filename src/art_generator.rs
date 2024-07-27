use image::Rgb;

use crate::{
    art_algo::ArtAlgo,
    board::{Board, Nail},
};

type NailPattern = Vec<(Rgb<u8>, Nail)>;

pub struct ArtGenerator {
    board: Board,
    algo: Box<dyn ArtAlgo>,
    pattern: NailPattern,
}

impl ArtGenerator {
    pub fn new(board: Board, algo: Box<dyn ArtAlgo>) -> Self {
        let pattern: NailPattern = algo
            .current_nails()
            .iter()
            .map(|(color, nail)| (*color, *nail))
            .collect();

        Self {
            board,
            algo,
            pattern,
        }
    }

    pub fn step(&mut self) -> (Rgb<u8>, Nail) {
        let (color, nail) = self.algo.choose_next_nail();
        self.pattern.push((color, nail));
        (color, nail)
    }

    pub fn get_pattern(&self) -> &NailPattern {
        &self.pattern
    }
}
