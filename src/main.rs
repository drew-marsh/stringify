use image::{ImageBuffer, Rgb};
use kmeans::kmeans;
use std::path::Path;
mod board;
use board::Board;
mod dither;
mod kmeans;

fn main() {
    let src_img = load_src_image().expect("Failed to load image");

    let board = Board::new(&src_img, 20, 300);

    dbg!(board);

    // let board_circumference = nail_count * nail_spacing_strixels;

    // let customPalette = [
    //     Rgb([137, 111, 78]),
    //     Rgb([131, 159, 104]),
    //     Rgb([113, 121, 137]),
    //     Rgb([255, 255, 255]),
    //     Rgb([76, 82, 75]),
    // ]
    // .to_vec();

    // let palette = kmeans(5, &image);

    // process_src_image(&customPalette);
}

fn process_src_image(palette: &[Rgb<u8>]) {
    let image = load_src_image().expect("Failed to load image");
    let dithered = dither::dither_image(&image, palette);
    save_image(&dithered, "dithered.png");
}

fn save_image(image: &ImageBuffer<Rgb<u8>, Vec<u8>>, name: &str) {
    let path = format!("imgout/{}", name);
    image.save(path).expect("Failed to save image");
}

fn load_src_image() -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, image::ImageError> {
    let path = Path::new("imgsrc").read_dir()?.next().unwrap()?.path();
    let img = image::open(path)?;
    Ok(img.to_rgb8())
}
