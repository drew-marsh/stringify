use std::collections::HashMap;

use image::Rgb;

use crate::board::Nail;

type NailPattern = Vec<(Rgb<u8>, Nail)>;

pub(crate) trait ArtAlgo {
    fn current_nails(&self) -> &HashMap<Rgb<u8>, Nail>;
    fn choose_next_nail(&mut self) -> (Rgb<u8>, Nail);
}

pub struct ArtGenerator {
    algo: Box<dyn ArtAlgo>,
    pattern: NailPattern,
}

impl ArtGenerator {
    pub fn new(algo: Box<dyn ArtAlgo>) -> Self {
        let pattern: NailPattern = algo
            .current_nails()
            .iter()
            .map(|(color, nail)| (*color, *nail))
            .collect();
        Self { algo, pattern }
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
