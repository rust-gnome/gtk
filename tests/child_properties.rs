extern crate gtk;

use gtk::{BoxExt, Button, ContainerExt, PackType};
use gtk::Orientation::Vertical;

#[test]
fn child_properties() {
    gtk::init().unwrap();

    let vbox = gtk::Box::new(Vertical, 0);
    let button = Button::new();
    vbox.add(&button);

    //gboolean
    assert_eq!(false, vbox.get_child_expand(&button));
    vbox.set_child_expand(&button, true);
    assert_eq!(true, vbox.get_child_expand(&button));

    //guint
    assert_eq!(0, vbox.get_child_padding(&button));
    vbox.set_child_padding(&button, 50);
    assert_eq!(50, vbox.get_child_padding(&button));

    //enum Gtk.PackType
    assert_eq!(PackType::Start, vbox.get_child_pack_type(&button));
    vbox.set_child_pack_type(&button, PackType::End);
    assert_eq!(PackType::End, vbox.get_child_pack_type(&button));
}
