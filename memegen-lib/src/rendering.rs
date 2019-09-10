use crate::layout::RgbaImage;
use image::Rgba;
use rusttype::{Font, PositionedGlyph, Scale};

const IMPACT_FONT_DATA: FontData = FontData(include_bytes!("../res/fonts/impact.ttf"));
const ROBOTO_FONT_DATA: FontData = FontData(include_bytes!("../res/fonts/roboto.ttf"));

const WOBBLE_OFFSET: [(i32, i32); 4] = [(2, 2), (2, -2), (-2, 2), (-2, -2)];

struct FontData(&'static [u8]);

pub enum FontDataSelector {
    ImpactFontData,
    RobotoFontData,
}

#[derive(Clone)]
pub struct Fontspec {
    pub font: Font<'static>,
    pub scale: Scale,
    pub colour_outline: (u8, u8, u8),
    pub colour_main: (u8, u8, u8),
}

impl Fontspec {
    pub fn impact() -> Fontspec {
        Fontspec {
            font: get_font(FontDataSelector::ImpactFontData),
            scale: Scale::uniform(64.0),
            colour_main: (255, 255, 255),
            colour_outline: (0, 0, 0),
        }
    }
    pub fn roboto() -> Fontspec {
        Fontspec {
            font: get_font(FontDataSelector::RobotoFontData),
            scale: Scale::uniform(64.0),
            colour_main: (255, 255, 255),
            colour_outline: (0, 0, 0),
        }
    }
}

pub fn get_font(selector: FontDataSelector) -> Font<'static> {
    let res = match selector {
        FontDataSelector::ImpactFontData => Font::from_bytes(IMPACT_FONT_DATA.0),
        FontDataSelector::RobotoFontData => Font::from_bytes(ROBOTO_FONT_DATA.0),
    };
    res.expect("Baked fonts are broken")
}

pub fn draw_glyphs_with_outline(
    fontspec: &Fontspec,
    glyphs: &Vec<PositionedGlyph>,
    mut image: &mut RgbaImage,
) {
    // Draw offset glyphs
    for offset in WOBBLE_OFFSET.iter() {
        draw_glyphs(fontspec.colour_outline, &glyphs, &mut image, offset);
    }
    // Draw centered glyphs
    draw_glyphs(fontspec.colour_main, &glyphs, &mut image, &(0, 0));
}

fn draw_glyphs(
    colour: (u8, u8, u8),
    glyphs: &Vec<PositionedGlyph>,
    image: &mut RgbaImage,
    offset: &(i32, i32),
) {
    let (max_x,max_y) = image.dimensions();
    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            // Draw the glyph into the image per-pixel by using the draw closure
            glyph.draw(|x, y, v| {
                // Offset the position by the glyph bounding box
                let px = x as i32 + bounding_box.min.x as i32 + offset.0;
                let py = y as i32 + bounding_box.min.y as i32 + offset.1;
                if px < 0 || px < 0{
                    return
                }
                // we can now safely cast them down
                let px = px as u32;
                let py = py as u32;

                if v > 0.5 && px < max_x && py < max_y {
                    image.put_pixel(
                        px,
                        py,
                        // Turn the coverage into an alpha value
                        Rgba {
                            data: [colour.0, colour.1, colour.2, (v * 255.0) as u8],
                        },
                    )
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rendering::{getFont, FontData, FontDataSelector};
    #[test]
    fn test_font_loaded_name() {
        let font = getFont(FontDataSelector::ImpactFontData);
        let font_name_string: String = font
            .font_name_strings()
            .map(|(a, b, c)| String::from_utf8_lossy(a))
            .collect();
        //rintln!("Str: {}",font_name_string);
        assert!(font_name_string.contains("Impact"));
    }
}
