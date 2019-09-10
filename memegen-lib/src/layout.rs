use image::{DynamicImage, ImageBuffer, Rgba};
use rusttype::{point, PositionedGlyph, Scale, VMetrics};

use crate::rendering::{draw_glyphs_with_outline, Fontspec};

pub type RgbaImage = ImageBuffer<Rgba<u8>, Vec<u8>>;

/**
A single line of a caption.
The Line owns all of its attributes.

*/
#[derive(Clone)]
pub struct Line {
    /**
        The string that is represented by this line.
        This string should consist of only printable characters,
        no error handling regarding unprintable chars is in place.
    */
    pub text: String,
    pub orientation: Orientation,
    pub fontspec: Fontspec,
    pub number_from_layout_anchor: u32,
}

impl Line {
    pub fn default() -> Line {
        Line {
            text: "Empty".to_string(),
            orientation: Orientation::Top,
            fontspec: Fontspec::impact(),
            number_from_layout_anchor: 0,
        }
    }
    pub fn get_v_metrics(&self) -> VMetrics {
        self.fontspec.font.v_metrics(self.fontspec.scale)
    }
}

#[derive(Clone)]
pub enum Orientation {
    Top,
    Bottom,
}

pub fn draw_lines_top_bottom(
    texts_top: Vec<String>,
    texts_bottom: Vec<String>,
    image: &mut RgbaImage,
) {
    let mut orientation = &Orientation::Top;
    for (ln, text) in texts_top.iter().enumerate() {
        let number_from_layout_anchor = ln as u32;
        let mut line = Line {
            text: text.clone(),
            orientation: orientation.clone(),
            number_from_layout_anchor,
            ..Line::default()
        };
        draw_line(&mut line, image);
    }

    orientation = &Orientation::Bottom;
    for (ln, text) in texts_bottom.iter().rev().enumerate() {
        let number_from_layout_anchor = ln as u32;
        let mut line = Line {
            text: text.clone(),
            orientation: orientation.clone(),
            number_from_layout_anchor,
            ..Line::default()
        };
        draw_line(&mut line, image);
    }
}

pub fn draw_line_at(line: &mut Line, image: &mut RgbaImage, x_pos: f32, y_pos: f32) {
    let v_metrics = line.get_v_metrics();
    let glyphs: Vec<_> = line
        .fontspec
        .font
        .layout(
            &line.text,
            line.fontspec.scale,
            point(x_pos, y_pos + v_metrics.ascent),
        )
        .collect();
    draw_glyphs_with_outline(&line.fontspec, &glyphs, image);
}

pub fn draw_line(line: &mut Line, image: &mut RgbaImage) {
    let (x_pos, y_pos) = autolayout_line(line, image);
    draw_line_at(line, image, x_pos, y_pos)
}

pub fn generate_font_rendering_with_transparency(line: &mut Line) -> RgbaImage {

    let padding = 6.0;
    let splits = line.text.split("\n");

    let mut total_height = 0.0;
    let mut max_width = 0.0;
    

    for split in splits {
        let pre_glyphs: Vec<_> = line
            .fontspec
            .font
            .layout(split, line.fontspec.scale, point(0.0, 0.0))
            .collect();
        let glyphs_width = get_glyph_width(&pre_glyphs) as f32;
        let v_metrics = line.fontspec.font.v_metrics(line.fontspec.scale);
        let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil();

        total_height += glyphs_height + padding;
        if max_width < glyphs_width  {
            max_width = glyphs_width
        }
    }   

    let mut preview =
        DynamicImage::new_rgba8((max_width + padding) as u32, (total_height + padding) as u32)
            .to_rgba();

    for (i,split) in line.text.split("\n").enumerate() {

        let spacing = line.fontspec.font.v_metrics(line.fontspec.scale).ascent + padding;
        let offset = spacing * i as f32;
        let mut single_line = Line{
            text: split.to_string(),
            fontspec: line.fontspec.clone(),
            orientation: Orientation::Top,  
            number_from_layout_anchor: 0,
        };
        draw_line_at(&mut single_line, &mut preview, 2 as f32, 2.0 + offset);
    }

    preview
}

fn autolayout_line(mut line: &mut Line, image: &mut RgbaImage) -> (f32, f32) {
    let ratio = 1.0;
    let border_padding = 20.0;
    let mut x_pos = get_x_pos(&line, image);
    println!("xpos: {}", x_pos);
    while x_pos < border_padding {
        line.fontspec.scale = Scale::uniform(line.fontspec.scale.x - 1.0);
        x_pos = get_x_pos(&line, image);
    }
    let v_metrics = line.fontspec.font.v_metrics(line.fontspec.scale);
    let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as f32;
    let pos_y = match line.orientation {
        Orientation::Top => border_padding + get_line_offset(&mut line, ratio, glyphs_height),
        Orientation::Bottom => {
            image.height() as f32
                - border_padding
                - glyphs_height as f32
                - get_line_offset(&mut line, ratio, glyphs_height)
        }
    };
    (x_pos, pos_y)
}

fn get_line_offset(line: &mut Line, ratio: f32, glyphs_height: f32) -> f32 {
    line.number_from_layout_anchor as f32 * (glyphs_height * ratio)
}

fn get_x_pos(line: &Line, image: &mut RgbaImage) -> f32 {
    if line.text.len() < 1 {
        return (image.width() / 2) as f32;
    }
    let pre_glyphs: Vec<_> = line
        .fontspec
        .font
        .layout(&line.text, line.fontspec.scale, point(0.0, 0.0))
        .collect();
    let glyphs_width = get_glyph_width(&pre_glyphs) as f32;
    let image_width = image.width() as f32;
    let x_pos = (image_width - glyphs_width) / 2.0;
    x_pos
}

fn get_glyph_width(glyphs: &Vec<PositionedGlyph>) -> u32 {
    let min_x = glyphs
        .first()
        .map(|g| g.pixel_bounding_box().unwrap().min.x)
        .unwrap();
    let max_x = glyphs
        .last()
        .map(|g| g.pixel_bounding_box().unwrap().max.x)
        .unwrap();
    (max_x - min_x) as u32
}

/*
These tests are in this file so they can directly test the private functions of the module
*/
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_x_pos_valid() {
        let image = DynamicImage::new_rgb8(100, 100);
        let line = Line {
            text: "a".to_string(),
            ..Line::default()
        };
        let x_pos = get_x_pos(&line, &mut image.to_rgba());
        // (100-24)/2 = 38
        assert_eq!(x_pos, 38.0);
    }
    #[test]
    fn test_get_x_pos_single_char_tiny_image() {
        let image = DynamicImage::new_rgb8(22, 22);
        let line = Line {
            text: "a".to_string(),
            ..Line::default()
        };
        let x_pos = get_x_pos(&line, &mut image.to_rgba());
        // the glyph cannot fit, so we get a negative position
        assert!(x_pos < 0.0);
        // (22-24)/2 = -1
        assert_eq!(x_pos, -1.0);
    }
    #[test]
    fn test_get_x_pos_long_text() {
        let image = DynamicImage::new_rgb8(100, 100);
        let line = Line {
            text: "aaaaaaaa".to_string(),
            ..Line::default()
        };
        let x_pos = get_x_pos(&line, &mut image.to_rgba());
        // chars: 8*24 = 192
        // spaces: 7*2 ~ 7*3 = 14 ~ 21
        // line-total: 206 ~ 213
        // xpos: -57 ~ -53.5
        assert!(x_pos > -57.0 && x_pos < -53.5);
    }

    #[test]
    fn test_font_img() {
        let test_img_data = generate_font_rendering_with_transparency(&mut Line {
            text: "Test".to_string(),
            ..Line::default()
        });
        test_img_data.save("test_output/test_font_img.png").unwrap();
    }
}
