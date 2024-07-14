use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};
use kmeans::kmeans;
use std::path::Path;
mod board;
use board::Board;
mod dither;
mod kmeans;

use image::imageops::FilterType;
use std::time::Instant;

fn main() {
    let nail_spacing_pixels = 20;
    let nail_count = 4;

    // load
    // let src_img = load_src_image().expect("Failed to load image");

    // board
    let board = Board::new(nail_spacing_pixels, nail_count);

    // scale
    // let scaled_img = board.scale_image(&src_img, None);
    let scaled_img = image::open(Path::new("imgout/scaled.png")).expect("Failed to load image");

    // dither
    let palette = [
        Rgb([137, 111, 78]),
        Rgb([131, 159, 104]),
        Rgb([113, 121, 137]),
        Rgb([255, 255, 255]),
        Rgb([76, 82, 75]),
    ]
    .to_vec();

    let dithered = dither::dither_image(&scaled_img, &palette);
    save_output_image(&dithered, "dithered.png");

    // let palette = kmeans(5, &image);
}

fn save_output_image(image: &DynamicImage, name: &str) {
    let path = format!("imgout/{}", name);
    image.save(path).expect("Failed to save image");
}

fn load_src_image() -> Result<DynamicImage, image::ImageError> {
    let path = Path::new("imgsrc").read_dir()?.next().unwrap()?.path();
    let img = image::open(path)?;
    Ok(img)
}
