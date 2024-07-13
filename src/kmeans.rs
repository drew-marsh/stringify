use image::{ImageBuffer, Rgb};
use rand::Rng;

pub fn kmeans(k: usize, image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<Rgb<u8>> {
    // Step 1: Initialize centroids randomly
    let mut centroids: Vec<Rgb<u8>> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..k {
        let random_pixel = image.get_pixel(
            rng.gen_range(0..image.width()),
            rng.gen_range(0..image.height()),
        );
        centroids.push(*random_pixel);
    }

    // Step 4: Repeat steps 2 and 3 until convergence or maximum iterations
    let max_iterations = 100;
    let mut iteration = 0;
    let mut converged = false;

    while !converged && iteration < max_iterations {
        let prev_centroids = centroids.clone();

        let assignments = assign_pixels_to_centroids(&centroids, &image);

        update_centroids(&mut centroids, &assignments, &image);

        converged = centroids
            .iter()
            .zip(prev_centroids.iter())
            .all(|(c1, c2)| c1 == c2);

        iteration += 1;
    }

    centroids
}

fn assign_pixels_to_centroids(
    centroids: &[Rgb<u8>],
    image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
) -> Vec<usize> {
    let mut assignments = Vec::<usize>::new();
    for (i, pixel) in image.pixels().enumerate() {
        let mut min_distance = u32::MAX;
        let mut closest_centroid = 0;
        for (j, centroid) in centroids.iter().enumerate() {
            let distance = calculate_distance(pixel, centroid);
            if distance < min_distance {
                min_distance = distance;
                closest_centroid = j;
            }
        }
        assignments.push(closest_centroid);
    }

    assignments
}

fn update_centroids(
    centroids: &mut Vec<Rgb<u8>>,
    assignments: &[usize],
    image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
) {
    for (i, centroid) in centroids.iter_mut().enumerate() {
        let mut sum_red = 0;
        let mut sum_green = 0;
        let mut sum_blue = 0;
        let mut count = 0;
        for (j, assignment) in assignments.iter().enumerate() {
            if *assignment == i {
                let pixel = image.get_pixel(j as u32 % image.width(), j as u32 / image.width());
                sum_red += pixel[0] as u32;
                sum_green += pixel[1] as u32;
                sum_blue += pixel[2] as u32;
                count += 1;
            }
        }
        if count > 0 {
            centroid[0] = (sum_red / count) as u8;
            centroid[1] = (sum_green / count) as u8;
            centroid[2] = (sum_blue / count) as u8;
        }
    }
}

fn calculate_distance(pixel1: &Rgb<u8>, pixel2: &Rgb<u8>) -> u32 {
    let red_diff = pixel1[0] as i32 - pixel2[0] as i32;
    let green_diff = pixel1[1] as i32 - pixel2[1] as i32;
    let blue_diff = pixel1[2] as i32 - pixel2[2] as i32;
    (red_diff * red_diff + green_diff * green_diff + blue_diff * blue_diff) as u32
}
