use std::fs::File;
use frame::GifFrame;
use image::{codecs::gif::{GifDecoder, GifEncoder}, AnimationDecoder};
use image_effects::prelude::{Filter, Affectable, Dither};
use image_effects::{prelude::{SrgbColour as RGB}};
use palettes::palettes;

mod frame;
mod palettes;

fn main() {
    let palettes = palettes();

    // let palette = vec![RGB::BLACK, RGB::WHITE, RGB::CYAN, RGB::MAGENTA, RGB::YELLOW];

    let file = File::open("./gif-effect/data/sunrise-fortpierce.gif").unwrap();
    let decoder = GifDecoder::new(file).unwrap();
    let frames = decoder.into_frames();
    let frames = frames.collect_frames().expect("Error decoding gif");

    for (name, palette) in palettes.iter() {
        println!("working for palette: {name}");
        let frames = frames.clone().into_iter()
            .map(|frame| GifFrame(frame)
                // .apply(&Filter::Contrast(2.0))
                .apply(&Dither::Bayer(2, &palette))
                .0)
            .collect::<Vec<_>>();

        let file_out = File::create(format!("./gif-effect/data/output-{name}.gif")).unwrap();
        let mut encoder = GifEncoder::new(file_out);
        encoder.encode_frames(frames.into_iter()).unwrap();
    }
}   
