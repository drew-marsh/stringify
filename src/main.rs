use art_generator::ArtGenerator;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};
use image_utils::kmeans;
use std::{path::Path, rc::Rc, time::Instant};
use stringifier::Stringifier;
mod board;
use board::Board;
mod art_algo;
mod art_generator;
mod image_utils;
mod stringifier;
mod util;

fn main() {
    let nail_spacing_pixels = 3;
    let nail_count = 200;

    // load
    let src_img = load_src_image("pikachu.jpg").expect("Failed to load image");

    // board
    let board = Rc::new(Board::new(nail_spacing_pixels, nail_count));

    // scale
    // let scaled_img = board.scale_image(&src_img, None);
    // let scaled_img = image::open(Path::new("imgout/scaled.png")).expect("Failed to load image");

    // let palette = [
    //     Rgb([137, 111, 78]),
    //     Rgb([131, 159, 104]),
    //     Rgb([113, 121, 137]),
    //     Rgb([255, 255, 255]),
    //     Rgb([76, 82, 75]),
    // ]
    // .to_vec();

    // let palette = kmeans(5, &src_img.to_rgb8());

    // pikachu palette
    let palette = vec![
        Rgb([214, 186, 189]),
        Rgb([107, 96, 122]),
        Rgb([20, 9, 23]),
        Rgb([183, 108, 57]),
        Rgb([71, 45, 45]),
        // cheeks
        Rgb([150, 32, 18]),
    ];

    // let dithered = dither_image(&scaled_img, &palette);
    // save_output_image(&dithered, "dithered.png");

    // masks
    // let color_masks = get_color_masks(&dithered, &palette);
    // save_mask_images(&color_masks, dithered);

    let algo = Stringifier::new(&board, &src_img, &palette);
    let mut generator = ArtGenerator::new(Rc::clone(&board), Box::new(algo));

    let start = Instant::now();

    let mut step = 0;
    while generator.step().is_some() {
        if step % 100 == 0 && step != 0 {
            println!("Step: {}", step);
            // save_output_image(generator.art(), "art.png");
        }
        step += 1;
    }
    println!("Completed after {} steps", step);
    println!("Elapsed time: {:?}", start.elapsed());

    let pattern = generator.pattern();
    let art = generator.art();
    save_output_image(art, "art.png");

    // println!("Pattern: {:?}", pattern);
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

fn load_src_image(filename: &str) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let file_path = Path::new("imgsrc").join(filename);
    let image = image::open(file_path)?;
    Ok(image)
}
