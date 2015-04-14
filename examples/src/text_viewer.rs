//! # Toolbar, Scrollable Text View and File Chooser
//!
//! A simple text file viewer

#![feature(core)]

extern crate gtk as rgtk;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::num::FromPrimitive;

use rgtk::{Connect, Gtk};
use rgtk::traits::*;
use rgtk::signals::{Clicked, DeleteEvent};
use rgtk::widgets::{BoxBuilder, ImageBuilder, ScrolledWindowBuilder, TextViewBuilder,
    ToolbarBuilder, ToolButtonBuilder, WindowBuilder};

fn main() {
    let gtk = Gtk::new();

    let mut window = gtk.window(rgtk::WindowType::TopLevel).unwrap();
    window.set_title("Text File Viewer");
    window.set_window_position(rgtk::WindowPosition::Center);
    window.set_default_size(400, 300);

    let mut toolbar = gtk.toolbar().unwrap();

    let open_icon = gtk.image_from_icon_name("document-open", rgtk::IconSize::SmallToolbar).unwrap();
    let text_view = gtk.text_view().unwrap();

    let mut open_button = gtk.tool_button::<rgtk::Image>(Some(&open_icon), Some("Open")).unwrap();
    open_button.set_is_important(true);
    Connect::connect(&open_button, Clicked::new(&mut || {
        // TODO move this to a impl?
        let file_chooser = rgtk::FileChooserDialog::new(
            "Open File", None, rgtk::FileChooserAction::Open,
            [("Open", rgtk::ResponseType::Ok), ("Cancel", rgtk::ResponseType::Cancel)]);
        let response: Option<rgtk::ResponseType> = FromPrimitive::from_i32(file_chooser.run());

        match response {
            Some(rgtk::ResponseType::Ok) => {
                let filename = file_chooser.get_filename().unwrap();
                let file = File::open(&filename).unwrap();

                let mut reader = BufReader::new(file);
                let mut contents = String::new();
                let _ = reader.read_to_string(&mut contents);

                text_view.get_buffer().unwrap().set_text(&contents);

            },
            _ => {}
        };

        file_chooser.destroy();
    }));

    toolbar.add(&open_button);

    let mut scroll = gtk.scrolled_window(None, None).unwrap();
    scroll.set_policy(rgtk::PolicyType::Automatic, rgtk::PolicyType::Automatic);
    scroll.add(&text_view);

    let mut vbox = gtk._box(rgtk::Orientation::Vertical, 0).unwrap();
    vbox.pack_start(&toolbar, false, true, 0);
    vbox.pack_start(&scroll, true, true, 0);

    window.add(&vbox);

    Connect::connect(&window, DeleteEvent::new(&mut |_| {
        gtk.main_quit();
        true
    }));

    window.show_all();
    gtk.main();
}

