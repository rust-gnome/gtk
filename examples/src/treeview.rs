//! # TreeView Sample
//!
//! This sample demonstrates how to create a TreeView with either a ListStore or TreeStore.

extern crate glib;
extern crate gtk as rgtk;

use rgtk::{Connect, Gtk};
use rgtk::signals::DeleteEvent;
use rgtk::traits::*;
use rgtk::widgets::{BoxBuilder, ListStoreBuilder, TreeIterBuilder, TreeStoreBuilder,
    TreeViewBuilder, TreeViewColumnBuilder, WindowBuilder};

fn append_text_column(gtk: &Gtk, tree: &mut rgtk::TreeView) {
    let column = gtk.tree_view_column().unwrap();
    let cell = rgtk::CellRendererText::new().unwrap();

    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    tree.append_column(&column);
}

fn main() {
    let gtk = Gtk::new();

    let mut window = gtk.window(rgtk::WindowType::TopLevel).unwrap();

    window.set_title("TreeView Sample");
    window.set_window_position(rgtk::WindowPosition::Center);

    Connect::connect(&window, DeleteEvent::new(&mut |_| {
        gtk.main_quit();
        true
    }));

    // test Value

    let hello = String::from("Hello world !");
    let mut value = glib::Value::new();

    value.init(glib::Type::String);
    value.set(&hello);
    println!("gvalue.get example : {}", value.get::<String>());

    // left pane

    let mut left_tree = gtk.tree_view().unwrap();
    let column_types = [glib::Type::String];
    let left_store = gtk.list_store(&column_types).unwrap();
    let left_model = left_store.get_model().unwrap();

    left_tree.set_model(&left_model);
    left_tree.set_headers_visible(false);
    append_text_column(&gtk, &mut left_tree);

    for _ in 0..10 {
        let mut iter = gtk.tree_iter().unwrap();
        left_store.append(&mut iter);
        left_store.set_string(&iter, 0, "I'm in a list");
    }

    // right pane

    let mut right_tree = gtk.tree_view().unwrap();
    let column_types = [glib::Type::String];
    let right_store = gtk.tree_store(&column_types).unwrap();
    let right_model = right_store.get_model().unwrap();

    right_tree.set_model(&right_model);
    right_tree.set_headers_visible(false);
    append_text_column(&gtk, &mut right_tree);

    for _ in 0..10 {
        let mut iter = gtk.tree_iter().unwrap();

        right_store.append(&mut iter, None);
        right_store.set_value(&iter, 0, &value);

        let mut child_iter = gtk.tree_iter().unwrap();

        right_store.append(&mut child_iter, Some(&iter));
        right_store.set_string(&child_iter, 0, "I'm a child node");
    }

    // display the panes

    let mut split_pane = gtk._box(rgtk::Orientation::Horizontal, 10).unwrap();

    split_pane.set_size_request(-1, -1);
    split_pane.add(&left_tree);
    split_pane.add(&right_tree);

    window.add(&split_pane);
    window.show_all();
    gtk.main();
}
