use image::DynamicImage;
use image::ImageBuffer;
use image::Rgb;

pub(crate) fn dither_image(image: &DynamicImage, palette: &[Rgb<u8>]) -> DynamicImage {
    let mut cloned_image = image.clone().to_rgb8();
    let width = cloned_image.width();
    let height = cloned_image.height();

    for y in 0..height {
        for x in 0..width {
            let pixel = *cloned_image.get_pixel(x, y);
            let closest_color = find_closest_color(pixel, palette);
            let quant_error = calculate_quantization_error(pixel, closest_color);
            cloned_image.put_pixel(x, y, closest_color);

            distribute_error(&mut cloned_image, x, y, quant_error);
        }
    }

    DynamicImage::ImageRgb8(cloned_image)
}

fn find_closest_color(pixel: Rgb<u8>, palette: &[Rgb<u8>]) -> Rgb<u8> {
    let mut closest_color = palette[0];
    let mut min_distance = calculate_color_distance(pixel, closest_color);

    for color in palette {
        let distance = calculate_color_distance(pixel, *color);
        if distance < min_distance {
            closest_color = *color;
            min_distance = distance;
        }
    }

    closest_color
}

fn calculate_color_distance(color1: Rgb<u8>, color2: Rgb<u8>) -> u32 {
    let r_diff = color1[0] as i32 - color2[0] as i32;
    let g_diff = color1[1] as i32 - color2[1] as i32;
    let b_diff = color1[2] as i32 - color2[2] as i32;

    (r_diff * r_diff + g_diff * g_diff + b_diff * b_diff) as u32
}

fn calculate_quantization_error(pixel: Rgb<u8>, closest_color: Rgb<u8>) -> Rgb<i32> {
    let r_error = pixel[0] as i32 - closest_color[0] as i32;
    let g_error = pixel[1] as i32 - closest_color[1] as i32;
    let b_error = pixel[2] as i32 - closest_color[2] as i32;

    Rgb([r_error, g_error, b_error])
}

fn distribute_error(
    image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    x: u32,
    y: u32,
    quant_error: Rgb<i32>,
) {
    let width = image.width();
    let height = image.height();

    if x + 1 < width {
        distribute_error_to_pixel(image, x + 1, y, quant_error, 7.0 / 16.0);
    }

    if x > 0 && y + 1 < height {
        distribute_error_to_pixel(image, x - 1, y + 1, quant_error, 3.0 / 16.0);
    }

    if y + 1 < height {
        distribute_error_to_pixel(image, x, y + 1, quant_error, 5.0 / 16.0);
    }

    if x + 1 < width && y + 1 < height {
        distribute_error_to_pixel(image, x + 1, y + 1, quant_error, 1.0 / 16.0);
    }
}

fn distribute_error_to_pixel(
    image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    x: u32,
    y: u32,
    quant_error: Rgb<i32>,
    factor: f32,
) {
    let pixel = image.get_pixel(x, y);
    let r = pixel[0] as i32 + (quant_error[0] as f32 * factor) as i32;
    let g = pixel[1] as i32 + (quant_error[1] as f32 * factor) as i32;
    let b = pixel[2] as i32 + (quant_error[2] as f32 * factor) as i32;

    let clamped_r = r.max(0).min(255) as u8;
    let clamped_g = g.max(0).min(255) as u8;
    let clamped_b = b.max(0).min(255) as u8;

    image.put_pixel(x, y, Rgb([clamped_r, clamped_g, clamped_b]));
}
