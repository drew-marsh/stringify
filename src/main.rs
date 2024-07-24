use art_generator::ArtGenerator;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};
use std::path::Path;
use stringifier::Stringifier;
mod board;
use board::Board;
mod art_generator;
mod image_utils;
mod stringifier;
mod util;
use image_utils::dither_image;
use image_utils::get_color_masks;

fn main() {
    let nail_spacing_pixels = 20;
    let nail_count = 10;

    // load
    let src_img = load_src_image().expect("Failed to load image");

    // board
    let board = Board::new(nail_spacing_pixels, nail_count);

    // scale
    // let scaled_img = board.scale_image(&src_img, None);
    // let scaled_img = image::open(Path::new("imgout/scaled.png")).expect("Failed to load image");

    // dither
    let palette = [
        Rgb([137, 111, 78]),
        Rgb([131, 159, 104]),
        Rgb([113, 121, 137]),
        Rgb([255, 255, 255]),
        Rgb([76, 82, 75]),
    ]
    .to_vec();

    // let dithered = dither_image(&scaled_img, &palette);
    // save_output_image(&dithered, "dithered.png");

    // masks
    // let color_masks = get_color_masks(&dithered, &palette);
    // save_mask_images(&color_masks, dithered);

    let algo = Stringifier::new(&board, &src_img, &palette);
    let mut generator = ArtGenerator::new(Box::new(algo));
    generator.step();
    let pattern = generator.get_pattern();

    println!("Pattern: {:?}", pattern);

    // let palette = kmeans(5, &image);
}

fn save_mask_images(
    color_masks: &std::collections::HashMap<Rgb<u8>, Vec<Vec<bool>>>,
    dithered: DynamicImage,
) {
    color_masks.iter().for_each(|(color, mask)| {
        let mut img = ImageBuffer::new(dithered.width(), dithered.height());
        for y in 0..dithered.height() {
            for x in 0..dithered.width() {
                let pixel_color = if mask[x as usize][y as usize] {
                    *color
                } else if *color == Rgb([255, 255, 255]) {
                    Rgb([0, 0, 0])
                } else {
                    Rgb([255, 255, 255])
                };
                img.put_pixel(x, y, pixel_color);
            }
        }
        save_output_image(
            &DynamicImage::ImageRgb8(img),
            &format!("mask_{}{}{}.png", color.0[0], color.0[1], color.0[2]),
        );
    });
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
