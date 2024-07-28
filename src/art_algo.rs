use std::collections::HashMap;

use image::Rgb;

use crate::board::Nail;

pub(crate) trait ArtAlgo {
    fn current_nails(&self) -> &HashMap<Rgb<u8>, Nail>;
    fn choose_next_nail(&mut self) -> Option<(Rgb<u8>, Nail)>;
}
