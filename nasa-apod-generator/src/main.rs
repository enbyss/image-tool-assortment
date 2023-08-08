use image_effects::utils::ImageFilterResult;
use rand::prelude::SliceRandom;
use dotenv::dotenv;

pub mod nasa;
pub mod palettes;
mod utils;

use crate::{nasa::dither_random_apod_image, palettes::palettes};

const ITERATIONS: usize = 25;

fn main() -> ImageFilterResult<()> {
    dotenv().ok();
    let palettes = palettes();
    let api_key = std::env::var("NASA_API_KEY").expect("NASA_API_KEY must be set in the environment/.env.");
    const USE_HD: bool = true;

    let mut rng = rand::thread_rng();
    for i in 0..ITERATIONS {
        let palette = palettes.choose(&mut rng).unwrap();
        println!("generating image {i} using palette [{}]...", palette.0);
        if let Err(error) = dither_random_apod_image(&mut rng, &api_key, palette, USE_HD) {
            println!("error when dithering apod: {:?}", error);
        };
    }

    Ok(())
}
