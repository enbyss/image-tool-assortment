use std::{error::Error, vec, fmt::format};

use common_utils::{image::ImageRequest, effectlog::{LogEntry, ExecLog}, palette::generate_n_random_palettes};
use image::DynamicImage;
use image_effects::{
    prelude::*,
    gradient_map, GradientMap, dither::bayer::Bayer,
};
use palette::{Lch, IntoColor, rgb::Rgb, named};
use rand::{rngs::StdRng, SeedableRng, seq::SliceRandom, Rng};

fn main() -> Result<(), Box<dyn Error>> {

    let image = ImageRequest::new("https://media.discordapp.net/attachments/908523233471520828/1143547682384777216/B5F82A24-74DA-4F7B-90A8-3E08D16037AD.jpg".into())
        .image()
        .with_max_dim(720)
        .perform()?
        .into_image()?;
        

    image.save("./basic/data/__SOURCE.png")?;

    generate_images_with_n_random_palettes(image, 500)?;
    // generate_images_with_predetermined_palettes(image, common_utils::palette::palettes())?;

    // let gradient = generate_hue_gradient(230.0);
    // print_gradient_grid(gradient);

    Ok(())
}

fn generate_images_with_n_random_palettes(image: DynamicImage, n: usize) -> Result<(), Box<dyn Error>> {
    let mut rng = StdRng::from_entropy();
    let palettes = generate_n_random_palettes(&mut rng, n);

    for (i, (palette, mut log)) in palettes.into_iter().enumerate() {
        println!("palette {i} / {n}");

        let possible_matrix_sizes = [2, 4, 8, 16];
        let matrix_size = possible_matrix_sizes.choose(&mut rng).unwrap();
        // let matrix_size = &2;

        let mut effects: Vec<Box<dyn Effect<DynamicImage>>> = Vec::new();

        if rng.gen_bool(0.5) {
            let contrast_factor = rng.gen_range(1.0..3.0);    
            log.add_entry(LogEntry::effect("contrast".into(), contrast_factor.to_string()));
            effects.push(Box::new(filters::Contrast(contrast_factor)));
        }
        if rng.gen_bool(0.5) {
            let luma_factor = rng.gen_range(-0.3..0.3);
            log.add_entry(LogEntry::effect("brighten".into(), luma_factor.to_string()));
            effects.push(Box::new(filters::Brighten(luma_factor)));
        }
        if rng.gen_bool(0.5) {
            let hue_shift = rng.gen_range(0.0..360.0); 
            log.add_entry(LogEntry::effect("hue-rotate".into(), hue_shift.to_string()));
            effects.push(Box::new(filters::HueRotate(hue_shift)));
        }
        if rng.gen_bool(0.5) {
            let chroma_factor = rng.gen_range(-0.3..0.3); 
            log.add_entry(LogEntry::effect("saturate".into(), chroma_factor.to_string()));
            effects.push(Box::new(filters::Saturate(chroma_factor)));
        }

        let mut image = image.clone();

        for effect in effects {
            image = effect.affect(image);
        }

        log.add_entry(LogEntry::effect("dither(bayer)".into(), format!("matrix-size({matrix_size})")));

        image.apply(&Bayer::new(*matrix_size as usize, palette))
            .save(format!("./basic/data/gen-output-{i}.png"))?;

        log.write_to(format!("./basic/data/gen-output-{i}.log.txt").as_str())?;
    }

    Ok(())
}

fn generate_images_with_predetermined_palettes(image: DynamicImage, palettes: Vec<(&str, Vec<Rgb>)>) -> Result<(), Box<dyn Error>> {
    // let palettes = common_utils::palette::palettes();

    for (name, palette) in palettes {
        println!("Palette: {}", name);
        image
            .clone()
            // .apply(&Filter::Brighten( 0.1))
            // .apply(&Filter::Contrast( 2.0))
            // .apply(&Filter::GradientMap(&gradient))
            // .apply(&Filter::MultiplyHue(3.0))
            // .apply(&Filter::RotateHue(60.0))
            .apply(&Bayer::new(8, palette))
            // .apply(&Dither::Atkinson(palette))
            // .apply(&Filter::Saturate(0.2))
            // .apply(&Filter::Contrast(2.5))
            // .apply(&Filter::RotateHue(240.0))
            // .apply(&Filter::MultiplyHue(80.0))
            .save(format!("./basic/data/output-{}.png", name))?;
    }

    Ok(())
}