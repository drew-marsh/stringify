use crate::board::Nail;
use image::Rgb;

pub type ColorPalette<'a> = &'a [Rgb<u8>];

pub trait ArtAlgo {
    fn chooseNextNail(&self) -> (Rgb<u8>, Nail);
}
