extern crate atk;
extern crate gdk;
extern crate gio;
extern crate glib;
extern crate gtk;
extern crate memegen_lib;
extern crate url;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{ Builder, Button, EventBox, Fixed, TextView, Window, Application };
use std::env::args;

use memegen_lib::{PreviewService};
use std::cell::RefCell;
use std::rc::Rc;


mod handlers;
mod utils;
use crate::handlers::*;
use crate::utils::*;
use crate::handlers::TextArea;


macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}



fn main() {
    let application = gtk::Application::new("dev.amann.test.rust_gk", Default::default())
        .expect("Initialization failed");

    application.connect_activate(move |app| {
        setup_gui(app);
    });

    application.run(&args().collect::<Vec<_>>());

    println!("Memegen terminated");
}

fn setup_gui(app: &Application){
    let glade_src = include_str!("main.glade");

    // this builder provides access to all components of the defined ui
    let builder = Builder::new_from_string(glade_src);

    let window: Window = builder.get_object("wnd_main").expect("Couldn't get window");
    window.set_title("Memegen");
    window.set_application(app);

    let evt_box: EventBox = builder.get_object("evt_box").expect("Couldn't get evt_box");

    let fixed_container = Fixed::new();
    evt_box.add(&fixed_container);

    evt_box.add_events(gdk::EventMask::SCROLL_MASK);

    let background_data = include_bytes!("../morpheus.jpg");

    let fallback_image = image::load_from_memory(background_data).unwrap().to_rgba();
    let fallback_prev = PreviewService::generate_preview(&fallback_image);

    let (img,background_dimensions) = add_image_at_target_position(&fixed_container, fallback_prev,(0,0));

    let text_images: Vec<TextArea> = Vec::new();
    let startpos = Rc::new(RefCell::new((0,0)));
    let text_idx = Rc::new(RefCell::new(-1 as i32));
    let lines = Rc::new(RefCell::new(text_images));

    let background_dimensions = Rc::new(RefCell::new(background_dimensions));
    let background_location = Rc::new(RefCell::new(String::new()));



    // we now wire up the buttons

    // the file button, opens the file chooser dialog
    let btn_file: Button = builder.get_object("btn_load").expect("Couldn't get btn_load");
    btn_file.connect_clicked( clone!(
            window, background_dimensions, background_location => move |_| {
            update_image_from_file(
                &window,
                &img,
                background_dimensions.borrow_mut(),
                background_location.borrow_mut())
        }));


    // the save button, saves the resulting image as output.jpg
    let btn_save: Button = builder.get_object("btn_save").expect("Couldn't get btn_save");
    btn_save.connect_clicked( clone!(
            lines, background_location => move |_| {
                handle_save(background_location.borrow_mut(),lines.borrow_mut())
        }));


    // we register a callback to receive updates about the changes on the text area

    let text_view: TextView = builder.get_object("text_view").expect("Couldn't get text_view");
    let text_buffer = text_view.get_buffer().unwrap();
    let last_text = Rc::new(RefCell::new(String::new()));

    text_buffer.connect_changed(clone!(
              lines, window, fixed_container,last_text => move |a|{
               handle_text_update(
                    last_text.borrow_mut(),
                    a,
                    &fixed_container,
                    lines.borrow_mut());
            window.show_all();
        }));


    // we now connect to mouse events: click, move and scroll

    evt_box.connect_button_press_event( clone!(
              lines, startpos, text_idx =>  move |_,btn| {
            handle_button_press(
                btn,
                startpos.borrow_mut(),
                lines.borrow_mut(),
                text_idx.borrow_mut(),
            );
            Inhibit(false)
        }));

    evt_box.connect_scroll_event( clone!(
              lines, window, fixed_container =>  move |_,evt| {
                handle_scroll_event(
                    evt,
                    lines.borrow_mut(),
                    &fixed_container,
                    &window
                );
                Inhibit(false)
        }));



    evt_box.connect_motion_notify_event(clone!(
      lines, background_dimensions, fixed_container,text_idx =>  move |_,evt| {
        handle_motion(
            evt,
            &fixed_container,
            lines.borrow_mut(),
            background_dimensions.borrow_mut(),
            text_idx.borrow_mut(),

        );
        Inhibit(false)
    }));

    window.show_all();

}
