use crate::util::ArtAlgo;
use image::Rgb;

use crate::board::Nail;

pub struct ArtGenerator {
    algo: Box<dyn ArtAlgo>,
    pattern: Vec<(Rgb<u8>, Nail)>,
}

impl ArtGenerator {
    pub fn new(algo: Box<dyn ArtAlgo>) -> Self {
        let pattern: Vec<(Rgb<u8>, Nail)> = Vec::new();
        Self { algo, pattern }
    }

    pub fn step(&mut self) -> (Rgb<u8>, Nail) {
        let (color, nail) = self.algo.chooseNextNail();
        self.pattern.push((color, nail));
        (color, nail)
    }

    pub fn getPattern(&self) -> &Vec<(Rgb<u8>, Nail)> {
        &self.pattern
    }
}
