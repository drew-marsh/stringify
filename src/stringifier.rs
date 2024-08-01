use crate::art_algo::{ArtAlgo, StrandPositions};
use crate::board::NailNailPaths;
use crate::util::Dimensions;
use crate::{
    board::{Board, Nail},
    image_utils::dither_image,
    util::ColorPalette,
};
use image::{DynamicImage, GenericImageView, Rgb};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use threadpool::ThreadPool;

pub struct Stringifier {
    initial_nails: HashMap<Rgb<u8>, Nail>,
    paths: HashMap<Nail, HashMap<Nail, Arc<Vec<(u32, u32)>>>>,
    remaining_pixels: Arc<RwLock<HashMap<Xy, Rgb<u8>>>>,
    dimensions: Dimensions,
}

type Xy = (u32, u32);

impl Stringifier {
    pub fn new(board: &Board, src_img: &DynamicImage, color_palette: ColorPalette) -> Self {
        let scaled_img = board.scale_image(src_img, None);
        let dithered_img = dither_image(&scaled_img, color_palette);

        let initial_nails =
            Stringifier::starting_nails(board.nails(), board.paths(), color_palette, &dithered_img);

        let remaining_pixels = Stringifier::image_to_pixel_options(&dithered_img);
        let remaining_pixels = Arc::new(RwLock::new(remaining_pixels));

        let paths = board.paths().clone();
        let paths = convert_to_arc_paths(paths);

        Self {
            initial_nails,
            paths,
            remaining_pixels,
            dimensions: board.dimensions().clone(),
        }
    }

    fn image_to_pixel_options(image: &DynamicImage) -> HashMap<Xy, Rgb<u8>> {
        let rgb_img = image.to_rgb8();
        let mut pixels = HashMap::new();
        let (width, height) = rgb_img.dimensions();

        for x in 0..width {
            for y in 0..height {
                let pixel_color = rgb_img.get_pixel(x, y);
                pixels.insert((x, y), *pixel_color);
            }
        }

        pixels
    }

    fn starting_nails(
        nails: &Vec<Nail>,
        paths: &NailNailPaths,
        color_palette: ColorPalette,
        dithered_img: &DynamicImage,
    ) -> HashMap<Rgb<u8>, Nail> {
        let mut starting_nails = HashMap::new();
        let dithered_rgb = dithered_img.to_rgb8();

        for color in color_palette {
            let chosen_path = Stringifier::choose_path(nails, paths, &dithered_rgb, color);

            if let Some(nails) = chosen_path {
                starting_nails.insert(*color, nails.0);
            }
        }
        starting_nails
    }

    fn choose_path(
        nails: &Vec<Nail>,
        paths: &NailNailPaths,
        dithered_rgb: &image::ImageBuffer<Rgb<u8>, Vec<u8>>,
        color: &Rgb<u8>,
    ) -> Option<(Nail, Nail)> {
        let mut max_match = 0;
        let mut chosen_path = None;

        for i in 0..nails.len() {
            for j in i + 1..nails.len() {
                let start = nails[i];
                let end = nails[j];
                let path = paths.get(&start).unwrap().get(&end).unwrap();
                let mut match_count = 0;

                for (x, y) in path {
                    let pixel_color = dithered_rgb.get_pixel(*x, *y);
                    if pixel_color == color {
                        match_count += 1;
                    }
                }

                if match_count > max_match {
                    max_match = match_count;
                    chosen_path = Some((start, end));
                }
            }
        }
        chosen_path
    }

    fn clear_path(&mut self, from_nail: Nail, to_nail: Nail) {
        let path = self.paths.get(&from_nail).unwrap().get(&to_nail).unwrap();

        path.iter().for_each(|(x, y)| {
            let mut rp = self.remaining_pixels.write().unwrap();
            rp.remove(&(*x, *y));
        });
    }
}

fn convert_to_arc_paths(
    paths: HashMap<Nail, HashMap<Nail, Vec<(u32, u32)>>>,
) -> HashMap<Nail, HashMap<Nail, Arc<Vec<(u32, u32)>>>> {
    let paths = paths
        .iter()
        .map(|(from_nail, paths_from)| {
            let paths_from = paths_from
                .iter()
                .map(|(end, path)| (end.clone(), Arc::new(path.clone())))
                .collect::<HashMap<Nail, Arc<Vec<(u32, u32)>>>>();
            (*from_nail, paths_from)
        })
        .collect::<HashMap<Nail, HashMap<Nail, Arc<Vec<(u32, u32)>>>>>();
    paths
}

impl ArtAlgo for Stringifier {
    fn initial_nails(&self) -> HashMap<Rgb<u8>, Nail> {
        self.initial_nails.clone()
    }

    fn next_nail(&mut self, nails: &StrandPositions) -> Option<(Rgb<u8>, Nail)> {
        let pool = ThreadPool::new(num_cpus::get());

        let worst_possible_score = -(self.dimensions.width() as i32);
        let best_move = None;
        let best_score = worst_possible_score;
        let best_move: Arc<Mutex<Option<(Rgb<u8>, Nail)>>> = Arc::new(Mutex::new(best_move));
        let best_score = Arc::new(Mutex::new(best_score));

        let handles: Vec<_> = nails
            .iter()
            .flat_map(|(color, nail)| {
                let paths_from_nail = self.paths.get(nail).unwrap();

                paths_from_nail
                    .iter()
                    .map(|(next_nail, path)| {
                        try_move(
                            &pool,
                            Arc::clone(&self.remaining_pixels),
                            Arc::clone(path),
                            *color,
                            Arc::clone(&best_move),
                            Arc::clone(&best_score),
                            *next_nail,
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        pool.join();

        let best_move = best_move.lock().unwrap().clone();

        if let Some((color, next_nail)) = best_move {
            self.clear_path(nails[&color], next_nail);
        }

        best_move
    }
}

fn try_move(
    pool: &ThreadPool,
    remaining_pixels: Arc<RwLock<HashMap<(u32, u32), Rgb<u8>>>>,
    path: Arc<Vec<(u32, u32)>>,
    color: Rgb<u8>,
    best_move: Arc<Mutex<Option<(Rgb<u8>, Nail)>>>,
    best_score: Arc<Mutex<i32>>,
    next_nail: Nail,
) {
    pool.execute(move || {
        let remaining_pixels = remaining_pixels.read().unwrap();

        let (match_count, score) = path_score(path, remaining_pixels, color);

        let mut best_move = best_move.lock().unwrap();
        let mut best_score = best_score.lock().unwrap();

        if match_count > 0 && score > *best_score {
            *best_score = score;
            *best_move = Some((color, next_nail));
        }
    })
}

fn path_score(
    path: Arc<Vec<(u32, u32)>>,
    remaining_pixels: std::sync::RwLockReadGuard<HashMap<(u32, u32), Rgb<u8>>>,
    color: Rgb<u8>,
) -> (i32, i32) {
    let mut match_count = 0;
    let mut mismatch_count = 0;

    path.iter().for_each(|(x, y)| {
        let pixel_color = remaining_pixels.get(&(*x, *y));

        match pixel_color {
            Some(pixel_color) if *pixel_color == color => {
                match_count += 1;
            }
            Some(_) => {
                mismatch_count += 1;
            }
            None => (),
        }
    });

    let score = match_count - mismatch_count;
    (match_count, score)
}

#[cfg(test)]
mod tests {
    use image::DynamicImage;
    use image::RgbImage;
    use std::sync::{Arc, Mutex};
    use std::thread;

    use super::*;

    // NWWWN
    // W  W
    // G B
    // GG
    // N
    fn create_mock_board() -> (Vec<Nail>, NailNailPaths, DynamicImage) {
        let nails = vec![Nail(0, 0), Nail(0, 4), Nail(4, 0)];
        let mut paths = HashMap::new();

        let mut from00 = HashMap::new();
        from00.insert(Nail(0, 4), vec![(0, 1), (0, 2), (0, 3)]);
        from00.insert(Nail(4, 0), vec![(1, 0), (2, 0), (3, 0)]);

        let mut from04 = HashMap::new();
        from04.insert(Nail(0, 0), vec![(0, 1), (0, 2), (0, 3)]);
        from04.insert(Nail(4, 0), vec![(1, 3), (2, 2), (3, 1)]);

        let mut from40 = HashMap::new();
        from40.insert(Nail(0, 0), vec![(1, 0), (2, 0), (3, 0)]);
        from40.insert(Nail(0, 4), vec![(1, 3), (2, 2), (3, 1)]);

        paths.insert(Nail(0, 0), from00);
        paths.insert(Nail(0, 4), from04);
        paths.insert(Nail(4, 0), from40);

        let img: DynamicImage = DynamicImage::ImageRgb8(RgbImage::from_fn(5, 5, |x, y| {
            if y < 2 {
                return Rgb([255, 255, 255]);
            }
            if x < 2 {
                return Rgb([127, 127, 127]);
            }
            Rgb([0, 0, 0])
        }));

        (nails, paths, img)
    }

    #[test]
    fn test_choose_next_nail() {
        let (_nails, paths, img) = create_mock_board();
        let color = Rgb([255, 255, 255]);

        let current_nails = HashMap::from([(color, Nail(0, 0))]);

        let mut stringifier = Stringifier {
            initial_nails: current_nails.clone(),
            paths: convert_to_arc_paths(paths),
            remaining_pixels: Arc::new(RwLock::new(Stringifier::image_to_pixel_options(&img))),
            dimensions: Dimensions::new(5, 5),
        };

        let next_nail = stringifier
            .next_nail(&current_nails)
            .expect("No next nail found");

        assert_eq!(next_nail, (color, Nail(4, 0)));
    }

    #[test]
    fn choose_two_nails() {
        let (_nails, paths, img) = create_mock_board();

        let w = Rgb([255, 255, 255]);
        let g = Rgb([127, 127, 127]);
        let b = Rgb([0, 0, 0]);

        let mut current_nails = HashMap::from([(w, Nail(0, 0)), (g, Nail(0, 0)), (b, Nail(0, 0))]);

        let mut stringifier = Stringifier {
            initial_nails: current_nails.clone(),
            paths: convert_to_arc_paths(paths),
            remaining_pixels: Arc::new(RwLock::new(Stringifier::image_to_pixel_options(&img))),
            dimensions: Dimensions::new(5, 5),
        };

        let next_nail = stringifier
            .next_nail(&current_nails)
            .expect("no next nail found");
        assert_eq!(next_nail, (w, Nail(4, 0)));

        current_nails.insert(next_nail.0, next_nail.1);

        let next_nail = stringifier
            .next_nail(&current_nails)
            .expect("no next nail found");
        assert_eq!(next_nail, (g, Nail(0, 4)));
    }

    #[test]
    fn test_choose_path() {
        let (nails, paths, img) = create_mock_board();
        let color = Rgb([255, 255, 255]);

        let chosen_path = Stringifier::choose_path(&nails, &paths, &img.to_rgb8(), &color);

        assert_eq!(chosen_path, Some((Nail(0, 0), Nail(4, 0))));
    }
}
