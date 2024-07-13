use image::{ImageBuffer, Rgb};
use kmeans::kmeans;
use std::path::Path;
mod dither;
mod kmeans;
fn main() {
    let image = load_image("juniper.jpg").expect("Failed to load image");

    // let palette = kmeans(5, &image);

    let palette: Vec<Rgb<u8>> = [
        Rgb([137, 111, 78]),
        Rgb([131, 159, 104]),
        Rgb([113, 121, 137]),
        Rgb([255, 255, 255]),
        Rgb([76, 82, 75]),
    ]
    .to_vec();

    let dithered = dither::dither_image(&image, &palette);

    dithered
        .save("juniper_dithered.png")
        .expect("Failed to save image");
}

fn load_image(file_path: &str) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, image::ImageError> {
    let path = Path::new(file_path);
    let img = image::open(path)?;
    Ok(img.to_rgb8())
}
