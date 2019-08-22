use gtk::prelude::*;
use std::path::Path;

use gdk_pixbuf::{Colorspace, Pixbuf};
use gtk::{ FileChooserAction, FileChooserDialog, FileFilter, Fixed, Image, ResponseType, Window};
use image::{ImageBuffer, Rgba};

use memegen_lib::PreviewService;
use std::cell::RefMut;

use crate::handlers::TextArea;

pub fn get_idx_from_position(lines: &mut Vec<TextArea>, x: i32, y: i32) -> Option<u32> {
    let mut ctr = 0;
    for img in &mut *lines {
        // check if click is in bounds
        if img.pos_x < x                        // first check lower x bound
            && img.size_x + img.pos_x > x       // then upper (right) x bound (size + position)
            && img.pos_y < y                    // then lower y bound
            && img.size_y + img.pos_y > y
        // and upper y bound
        {
            // the user clicked into the text area
            // update the click offset for moving the image relative to the pointer position
            img.click_offset_x = x - img.pos_x;
            img.click_offset_y = y - img.pos_y;

            return Some(ctr);
        }
        // else not in bounds
        ctr += 1;
    }
    return None;
}

pub fn update_image_from_file(
    window: &Window,
    img: &Image,
    mut background_dimensions: RefMut<(u32, u32)>,
    mut background_location: RefMut<String>,
) {
    let dialog =
        FileChooserDialog::new(Some("Choose a file"), Some(window), FileChooserAction::Open);
    dialog.add_buttons(&[
        ("Cancel", ResponseType::Cancel.into()),
        ("Open", ResponseType::Ok.into()),
    ]);

    dialog.set_current_folder(Path::new("."));

    let filter = FileFilter::new();
    filter.add_pattern("*.jpg");
    filter.add_pattern("*.jpeg");
    filter.add_pattern("*.JPG");
    dialog.set_filter(&filter);
    dialog.set_select_multiple(true);
    // this shows the dialog and blocks execution
    dialog.run();

    // at this point the user has closed the dialog and we take the selected files
    let filename = dialog.get_filename();
    dialog.destroy();
    if let Some(file) = filename {
        let image_file = image::open(&file);
        if let Ok(image_file) = image_file {
            let filename_str = file.to_str().unwrap();
            println!("Loading: {}", filename_str);

            let background = image_file.to_rgba();
            let background = PreviewService::generate_preview(&background);
            let (w, h) = background.dimensions();
            *background_dimensions = background.dimensions();
            *background_location = String::from(filename_str);

            let data = background.into_raw();
            let pixbuf = Pixbuf::new_from_mut_slice(
                data,
                Colorspace::Rgb,
                true,
                8,
                w as i32,
                h as i32,
                (w * 4) as i32,
            );
            img.set_from_pixbuf(&pixbuf);
        }
    }
}

pub fn add_image_at_target_position(
    fixed_container: &Fixed,
    source: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pos: (i32, i32),
) -> (Image, (u32, u32)) {
    let (w, h) = source.dimensions();
    let source_dimensions = source.dimensions();
    let data = source.into_raw();
    let img = Image::new_from_pixbuf(&Pixbuf::new_from_mut_slice(
        data,
        Colorspace::Rgb,
        true,
        8,
        w as i32,
        h as i32,
        (w * 4) as i32,
    ));
    fixed_container.put(&img, pos.0, pos.1);
    (img, source_dimensions)
}

pub fn common_prefix_len(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).take_while(|(x, y)| x == y).count()
}
pub fn common_suffix_len(a: &str, b: &str) -> usize {
    // reversing the char iters individually, since the slices might be of different size
    a.chars()
        .rev()
        .zip(b.chars().rev())
        .take_while(|(x, y)| x == y)
        .count()
}