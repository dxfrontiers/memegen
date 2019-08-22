use gtk::prelude::*;

use gdk::{EventScroll,EventButton,EventMotion};
use gtk::{ Fixed, Image, Window };

use gdk::ScrollDirection;
use memegen_lib::{draw_line_at, Line, Orientation, PreviewService};
use std::cell::RefMut;

use std::cmp::max;

use crate::utils::*;


#[derive(Clone)]
pub struct TextArea {
    pub pos_x: i32,
    pub pos_y: i32,
    pub size_x: i32,
    pub size_y: i32,
    pub image: Image,
    pub click_offset_x: i32,
    pub click_offset_y: i32,
    pub position_in_text_field: i32,
    pub line: Line,
}

/**
    Handle motion of the mouse pointer
    This function moves a TextArea, if there is a valid index selected
*/
pub fn handle_motion(
    evt: &EventMotion,
    fixed_container: &Fixed,
    mut lines: RefMut<Vec<TextArea>>,
    background_dimensions: RefMut<(u32, u32)>,
    text_idx: RefMut<i32>,

){
    let (x,y) = evt.get_position();
    let x = x as i32;
    let y = y as i32;

    // if the index is less than one, we expect that there is no textarea selected
    if *text_idx >=0 {
        let mut element = lines.get_mut(*text_idx as usize).unwrap();

        let img_pos_x = x-element.click_offset_x;
        let img_pos_y = y-element.click_offset_y;


        // check if we do no leave the background image boundaries
        // this is split into two separate checks since we want to move along an edge,
        // even if the pointer is moving trough the edge

        if element.size_x + img_pos_x < background_dimensions.0 as i32
            && img_pos_x > 0{
            fixed_container.move_(&element.image, img_pos_x, element.pos_y);
            element.pos_x = img_pos_x;
        }

        if element.size_y + img_pos_y < background_dimensions.1 as i32
            && img_pos_y > 0{
            fixed_container.move_(&element.image,element.pos_x , img_pos_y);
            element.pos_y=img_pos_y;
        }

    }
}




/**
    The not-so-secret sauce of the memegen
    This function handles updates in the text and to a certain degree, supports updates of the text
    without resetting text areas.
*/
pub fn handle_text_update(
    mut last_text: RefMut<String>,
    a: &gtk::TextBuffer,
    fixed_container: &Fixed,
    lines: RefMut<Vec<TextArea>>,
){

    // determine text sections
    let end_iter = a.get_end_iter();
    let start_iter = a.get_start_iter();

    let opt = a.get_text(&start_iter,&end_iter,false);
    let opt = opt.unwrap();
    let s = opt.to_string();

    handle_new_text(&mut last_text,&s,fixed_container,lines);

    *last_text = String::from(s.clone());

}


pub fn handle_new_text(
    last_text: &mut RefMut<String>,
    current_text: &String,
    fixed_container: &Fixed,
    mut lines: RefMut<Vec<TextArea>>,
){
    // split the last seen text
    let last_splits: Vec<&str>  = last_text.split("\n\n").collect();

    let current_splits = current_text.split("\n\n");

    let current_splits = current_splits.enumerate().map(|(pos,block)|  {
        //println!("Split: {}",block);
        let mut old_pos = None;
        for (last_pos,last) in last_splits.iter().enumerate() {
            if *last == block{
                old_pos = Some(last_pos);
                //println!("old {} equals new {}",last,block);
                // exiting the map function early
                return (old_pos,pos,block);
            }
            else {
                // not found yet, check for prefix or suffix
                let prefix_len = common_prefix_len(last,block);
                let suffix_len = common_suffix_len(last,block);
                let max_common_len = max(prefix_len,suffix_len);
                // if a prefix or suffix matches at the same position we consider the text to be updated but not new
                // this might be not entirely correct but a flaw here is a already placed text area
                // this might also lead to an already positioned line to be reset if we insert some text with an existing prefix but this is a prototype
                if max_common_len > 0 && last_pos == pos {
                    old_pos = Some(last_pos);
                    return (old_pos,pos,block);
                }
                // else the line was moved or modified in another way, ignore it
            }
        }
        (old_pos,pos,block)
    });


    let mut new_areas = Vec::new();
    /*
        I would have liked to chain this with another iterator and a foreach function but we
        cannot access `lines` from inside the clsoure in a writeable manner and zipping the two iterators is hard due to possibly different lengths
    */
    for (old_id,_,line_text) in  current_splits{
        match old_id {
            // if we have an old id set (aka not a new line), reuse the old one
            Some(old_id) => {
                // only reuse the text area, if it is a valid id (invalid indexes could happen if the matching algorithm above fails
                if lines.len() > old_id {
                    let ta = lines.get(old_id).unwrap();

                    // clone the elements we want to replace
                    let mut single_line = ta.line.clone();
                    let mut ta = (*ta).clone();

                    // replace the parameters
                    single_line.text= line_text.trim().to_string();
                    ta.line = single_line;

                    // and add the reused text area
                    new_areas.push((line_text,Some(ta)) );
                }
                else { new_areas.push((line_text,None) ); }
            },
            None => {new_areas.push((line_text,None) );}
        };
    }

    // remove all *old* text area images from the gui
    lines.iter().for_each(|i|{
        fixed_container.remove(&i.image);
    });

    // clear the list of old text areas, the new ones are already stored elsewhere
    lines.clear();

    // render all text areas and add them back to the vector
    for (text,area) in  new_areas{
        let img_text = text.trim();

        // skip if we have empty text
        if img_text == ""{
            continue
        }

        // reuse the text areas, if they are contained in the tuple
        let area = match area {
            Some(mut ta) => {
                let img_data = memegen_lib::generate_font_rendering_with_transparency(&mut ta.line);
                let (img,background_dimensions) = add_image_at_target_position(&fixed_container, img_data,(ta.pos_x,ta.pos_y));
                ta.image = img;
                ta.size_x=  background_dimensions.0 as i32;
                ta.size_y=  background_dimensions.1 as i32;
                ta
            }
            None => {
                let mut line = Line{text:img_text.to_string(),..Line::default()};
                let img_data = memegen_lib::generate_font_rendering_with_transparency(&mut line);
                let (img,background_dimensions) = add_image_at_target_position(&fixed_container, img_data,(10,10));
                TextArea{
                    pos_x: 10,
                    pos_y: 10,
                    size_x: background_dimensions.0 as i32,
                    size_y: background_dimensions.1 as i32,
                    image: img,
                    click_offset_x: 0,
                    click_offset_y: 0,
                    line: line,
                    position_in_text_field: 0,
                }
            }
        };

        lines.push(area);

    }
}

/**
    This function handles scroll events on text areas to resize fint elements
*/
pub fn handle_scroll_event(
    evt: &EventScroll,
    mut lines: RefMut<Vec<TextArea>>,
    fixed_container: &Fixed,
    window: &Window,
){
    let (x,y) = evt.get_position();
    let x = x as i32;
    let y = y as i32;

    let dir = evt.get_direction();
    let scale_modifier = match dir{
        ScrollDirection::Down => -2,
        ScrollDirection::Up => 2,
        _ => 0
    };

    let res = get_idx_from_position(&mut lines,x,y);

    if let Some(i) = res{
        if scale_modifier != 0{
            //let mut lines = lines.borrow_mut();
            let mut element = lines.get_mut(i as usize).unwrap();
            if element.line.fontspec.scale.x > 2.0 {
                element.line.fontspec.scale.x += scale_modifier as f32;
                element.line.fontspec.scale.y += scale_modifier as f32;

                fixed_container.remove(&element.image);

                let img_data = memegen_lib::generate_font_rendering_with_transparency(&mut element.line);
                let (img,background_dimensions) = add_image_at_target_position(&fixed_container, img_data,(element.pos_x,element.pos_y));
                element.image = img;
                element.size_x = background_dimensions.0 as i32;
                element.size_y = background_dimensions.1 as i32;

                window.show_all();
            }
        }
    }
}

/**
    Handles activations of the primary mouse button
    This is used to select the index of the text area, that will be moved
*/
pub fn handle_button_press(
    evt: &EventButton,
    mut startpos: RefMut<(i32,i32)>,
    mut lines: RefMut<Vec<TextArea>>,
    mut text_idx: RefMut<i32>,
){
    let (x,y) = evt.get_position();
    let x = x as i32;
    let y = y as i32;

    *startpos=(x , y);

    let res = get_idx_from_position(&mut lines,x,y);
    if let Some(i) = res{
        *text_idx=i as i32;
    }
    else{
        *text_idx=-1;
    }
}

/**
    Handle activation of the save button
*/
pub fn handle_save(
    background_location: RefMut<String>,
    mut lines: RefMut<Vec<TextArea>>,
){
    let mut image = image::load_from_memory(include_bytes!("../morpheus.jpg")).unwrap().to_rgba();
    if *background_location != ""{
        image = image::open(&*background_location).unwrap().to_rgba();
    }

    let mut image = PreviewService::generate_preview(&image);
    for area in lines.iter_mut() {
        let line = &area.line;
        for (i,split) in line.text.split("\n").enumerate() {

            let spacing = line.fontspec.font.v_metrics(line.fontspec.scale).ascent + 6.0;
            let offset = spacing * i as f32;
            let mut single_line = Line{
                text: split.to_string(),
                fontspec: line.fontspec.clone(),
                orientation: Orientation::Top,
                number_from_layout_anchor: 0,
            };
            draw_line_at(&mut single_line, &mut image, area.pos_x as f32,area.pos_y as f32 + offset);
        }
    }
    image.save("./output.jpg").unwrap();
}

