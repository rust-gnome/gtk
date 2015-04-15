//! # Synchronizing Widgets
//!
//! You can use signals in order to synchronize the values of widgets. In this example a spin button and a horizontal scale will get interlocked.

extern crate gtk as rgtk;

use rgtk::{Connect, Gtk};
use rgtk::traits::*;
use rgtk::signals::{ValueChanged, DeleteEvent};
use rgtk::widgets::*;

fn main() {
    let gtk = Gtk::new();

    let mut window = gtk.window(rgtk::WindowType::TopLevel).unwrap();

    window.set_title("Enter your age");
    window.set_window_position(rgtk::WindowPosition::Center);
    window.set_default_size(300, 20);

    let mut spin_button = gtk.spin_button_with_range(0.0, 130.0, 1.0).unwrap();
    let slider = gtk.scale_with_range(rgtk::Orientation::Horizontal, 0.0, 130.0, 1.0).unwrap();

    Connect::connect(&spin_button, ValueChanged::new(&mut || {
        let mut adjustment = slider.get_adjustment();

        adjustment.set_value(spin_button.get_value());
    }));

    Connect::connect(&slider, ValueChanged::new(&mut || {
        let adjustment = slider.get_adjustment();

        spin_button.set_value(adjustment.get_value());
    }));

    let mut hbox = gtk._box(rgtk::Orientation::Horizontal, 5).unwrap();

    hbox.set_homogeneouse(true);
    hbox.add(&spin_button);
    hbox.add(&slider);

    Connect::connect(&window, DeleteEvent::new(&mut |_| {
        gtk.main_quit();
        true
    }));

    window.add(&hbox);
    window.show_all();
    gtk.main();
}
