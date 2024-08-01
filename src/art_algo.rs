use std::collections::HashMap;

use image::Rgb;

use crate::board::Nail;

pub type StrandPositions = HashMap<Rgb<u8>, Nail>;

pub(crate) trait ArtAlgo {
    fn initial_nails(&self) -> StrandPositions;
    fn next_nail(&mut self, nails: &StrandPositions) -> Option<(Rgb<u8>, Nail)>;
}
