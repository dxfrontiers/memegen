
pub mod memegen {}

mod layout;
mod rendering;
mod service;

pub use layout::draw_line;
pub use layout::draw_line_at;
pub use layout::draw_lines_top_bottom;
pub use layout::generate_font_rendering_with_transparency;
pub use layout::Line;
pub use layout::Orientation;
pub use service::PositionedLine;
pub use service::PreviewService;
pub use service::UpdateRequest;

#[cfg(test)]
mod tests {
    use crate::service::PreviewService;
    use crate::{draw_line_at, draw_lines_top_bottom, Line};

    #[test]
    fn test_rendering_exact_position() {
        let text1 = vec!["Toptext".to_string()];
        let text1 = vec!["Toptext".to_string()];

        let mut image = image::open("res/images/puffin.jpg").unwrap().to_rgba();
        let mut line = Line {
            text: "TXßg".to_string(),
            ..Line::default()
        };
        draw_line_at(&mut line, &mut image, 50.0, 50.0);
        image
            .save("test_output/test_rendering_exact_position.png")
            .unwrap();
    }

    #[test]
    fn test_rendering_empty() {
        let texts_top = vec!["".to_string()];
        let texts_bottom = vec!["".to_string()];
        let mut image = image::open("res/images/puffin.jpg").unwrap().to_rgba();
        draw_lines_top_bottom(texts_top, texts_bottom, &mut image);
        image.save("test_output/test_rendering_empty.jpg").unwrap();
    }

    #[test]
    fn test_rendering_multiline() {
        let texts_top = vec![
            "Toptext hoes here".to_string(),
            "Second line top".to_string(),
        ];
        let texts_bottom = vec![
            "First lower".to_string(),
            "second lower".to_string(),
            "some text is so tiny you can barely read it if you do not\
             move closer to your monitor but that is unhealthy for your eyes."
                .to_string(),
        ];

        let mut image = image::open("res/images/puffin.jpg").unwrap().to_rgba();
        draw_lines_top_bottom(texts_top, texts_bottom, &mut image);

        image
            .save("test_output/test_rendering_multiline.jpg")
            .unwrap();
    }

    #[test]
    fn test_rendering_with_long_texts() {
        let texts_top = vec![
            "Toptext hoes here".to_string(),
            "Second line top".to_string(),
        ];
        let texts_bottom = vec!["First lower".to_string(),
                                "second lower".to_string(),
                                "some text is so tiny you can barely read it if you do not\
                                 move closer to your monitor but that is unhealthy for your eyes.\
                                  Some say this might be trolling peaople with bad eyes but for\
                                  this test I think this might be ok. I wonder if we can get the \
                                  font so tiny that not even font size one will be small enough, \
                                  so that the program is stuck in an infinite loop.\
                                  In metal typesetting, a font was a particular size, weight and \
                                  style of a typeface. Each font was a matched set of type,\
                                   one piece (called a sort) for each glyph, and a typeface
                                   consisting of a range of fonts that shared an overall design.
                                   In modern usage, with the advent of digital typography, font\
                                    is frequently synonymous with typeface. Each style is in a\
                                     separate font file—for instance, the typeface Bulmer may \
                                     include the fonts Bulmer roman, Bulmer italic, Bulmer bold and B\
                                     ulmer extended—but the term font might be applied either to one \
                                     of these alone or to the whole typeface.".to_string(),
                                "
                                     In metal typesetting, a font was a particular size, weight and \
                                  style of a typeface. Each font was a matched set of type,\
                                   one piece (called a sort) for each glyph, and a typeface
                                   consisting of a range of fonts that shared an overall design.
                                   In modern usage, with the advent of digital typography, font\
                                    is frequently synonymous with typeface. Each style is in a\
                                     separate font file—for instance, the typeface Bulmer may \
                                     include the fonts Bulmer roman, Bulmer italic, Bulmer bold and B\
                                     ulmer extended—but the term font might be applied either to one \
                                     of these alone or to the whole typeface.".to_string(),
                                "
                                     In metal typesetting, a font was a particular size, weight and \
                                  style of a typeface. Each font was a matched set of type,\
                                   one piece (called a sort) for each glyph, and a typeface
                                   consisting of a range of fonts that shared an overall design.
                                   In modern usage, with the advent of digital typography, font\
                                    is frequently synonymous with typeface. Each style is in a\
                                     separate font file—for instance, the typeface Bulmer may \
                                     include the fonts Bulmer roman, Bulmer italic, Bulmer bold and B\
                                     ulmer extended—but the term font might be applied either to one \
                                     of these alone or to the whole typeface.".to_string()];

        let mut image = image::open("res/images/puffin.jpg").unwrap().to_rgba();
        draw_lines_top_bottom(texts_top, texts_bottom, &mut image);

        image
            .save("test_output/test_rendering_with_long_texts.jpg")
            .unwrap();
    }

    #[test]
    fn test_rendering_with_one_insanely_long_text() {
        let texts_top = vec![
            "Toptext hoes here".to_string(),
            "Second line top".to_string(),
        ];
        let texts_bottom = vec!["First lower".to_string(),
                                "second lower".to_string(),
                                "some text is so tiny you can barely read it if you do not\
                                 move closer to your monitor but that is unhealthy for your eyes.\
                                  Some say this might be trolling peaople with bad eyes but for\
                                  this test I think this might be ok. I wonder if we can get the \
                                  font so tiny that not even font size one will be small enough, \
                                  so that the program is stuck in an infinite loop.\
                                  In metal typesetting, a font was a particular size, weight and \
                                  style of a typeface. Each font was a matched set of type,\
                                   one piece (called a sort) for each glyph, and a typeface
                                   consisting of a range of fonts that shared an overall design.
                                   In modern usage, with the advent of digital typography, font\
                                    is frequently synonymous with typeface. Each style is in a\
                                     separate font file—for instance, the typeface Bulmer may \
                                     include the fonts Bulmer roman, Bulmer italic, Bulmer bold and B\
                                     ulmer extended—but the term font might be applied either to one \
                                     of these alone or to the whole typeface.\
                                  In metal typesetting, a font was a particular size, weight and \
                                  style of a typeface. Each font was a matched set of type,\
                                   one piece (called a sort) for each glyph, and a typeface
                                   consisting of a range of fonts that shared an overall design.
                                   In modern usage, with the advent of digital typography, font\
                                    is frequently synonymous with typeface. Each style is in a\
                                     separate font file—for instance, the typeface Bulmer may \
                                     include the fonts Bulmer roman, Bulmer italic, Bulmer bold and B\
                                     ulmer extended—but the term font might be applied either to one \
                                     of these alone or to the whole typeface.\
                                     In metal typesetting, a font was a particular size, weight and \
                                  style of a typeface. Each font was a matched set of type,\
                                   one piece (called a sort) for each glyph, and a typeface
                                   consisting of a range of fonts that shared an overall design.
                                   In modern usage, with the advent of digital typography, font\
                                    is frequently synonymous with typeface. Each style is in a\
                                     separate font file—for instance, the typeface Bulmer may \
                                     include the fonts Bulmer roman, Bulmer italic, Bulmer bold and B\
                                     ulmer extended—but the term font might be applied either to one \
                                     of these alone or to the whole typeface.".to_string()];

        let mut image = image::open("res/images/puffin.jpg").unwrap().to_rgba();
        draw_lines_top_bottom(texts_top, texts_bottom, &mut image);

        image
            .save("test_output/test_rendering_with_one_insanely_long_text.jpg")
            .unwrap();
    }
}
