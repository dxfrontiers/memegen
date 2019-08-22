extern crate memegen_lib;
use image::{DynamicImage, Rgba, ImageBuffer,GenericImage};
use rusttype::{point, Font, Scale, PositionedGlyph, VMetrics};

use memegen_lib::{Line, Orientation, draw_lines_top_bottom};
use memegen_lib::draw_line;

fn main() {
    let texts_top = vec!["One does not simply".to_string()];
    let texts_bottom = vec!["create a meme generator".to_string(),];

    let mut image = image::open("simply.jpg").unwrap().to_rgba();
    draw_lines_top_bottom(texts_top,texts_bottom, &mut image);

    image.save("test_output/test.jpg").unwrap();
}




