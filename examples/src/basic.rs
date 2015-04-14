//! # Basic Sample
//!
//! This sample demonstrates how to create a toplevel `window`, set its title, size and position, how to add a `button` to this `window` and how to connect signals with actions.

#![crate_type = "bin"]

extern crate gtk as rgtk;

use rgtk::{Connect, Gtk};
use rgtk::traits::*;
use rgtk::signals::{DeleteEvent};
use rgtk::widgets::{ButtonBuilder, WindowBuilder};

fn main() {
    let gtk = Gtk::new();

    let mut window = gtk.window(rgtk::WindowType::TopLevel).unwrap();

    window.set_title("First GTK+ Program");
    window.set_border_width(10);
    window.set_window_position(rgtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    Connect::connect(&window, DeleteEvent::new(&mut |_| {
        gtk.main_quit();
        true
    }));

    let button = gtk.button_with_label("Click me!").unwrap();

    window.add(&button);

    window.show_all();
    gtk.main();
}

