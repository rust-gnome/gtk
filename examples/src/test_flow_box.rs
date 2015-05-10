extern crate gtk;

use gtk::{IconSize, Orientation, ReliefStyle};
use gtk::signal::Inhibit;
use gtk::traits::*;

fn main() {
    gtk::init();

  let window = gtk::Window::new(gtk::WindowType::TopLevel).unwrap();
  window.set_title("Flow box");
  window.set_window_position(gtk::WindowPosition::Center);
  window.set_default_size(400, 400);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(true)
    });

    let flow_box = gtk::FlowBox::new().unwrap();
    let flow_box_child = gtk::FlowBoxChild::new().unwrap();
    flow_box.connect_select_all(move |flow_box| {
        println!("select all");
    });

    window.add(&flow_box);
    for x in 1..10
      {
        let icon = gtk::Image::new_from_icon_name("folder", gtk::IconSize::Button).unwrap();
        flow_box.add(&icon);
      }
    window.show_all();
    gtk::main();
}
