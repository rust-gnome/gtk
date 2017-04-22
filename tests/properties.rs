extern crate gtk;
extern crate gtk_sys as ffi;

use gtk::prelude::*;
use gtk::*;

#[test]
fn properties() {
    //HACK: workaround for travis (or for Ubuntu 16.04)
    //gtk::init().unwrap();
    unsafe {
        let mut argc = 0;
        ffi::gtk_init(&mut argc, std::ptr::null_mut());
        gtk::set_initialized();
    }

    let align = Alignment::new(0f32, 0f32, 1f32, 1f32);
    let cell_area_box = CellAreaBox::new();
    let combobox = ComboBox::new_with_area(&cell_area_box);
    let cell_area = cell_area_box.upcast::<CellArea>();
    let arrow = Arrow::new(ArrowType::Down, ShadowType::None);
    let vbox = gtk::Box::new(Orientation::Vertical, 0);

    //gfloat
    assert_eq!(0f32, align.get_property_xalign());
    align.set_property_xalign(1f32);
    assert_eq!(1f32, align.get_property_xalign());

    //guint
    assert_eq!(0, align.get_property_bottom_padding());
    align.set_property_bottom_padding(10);
    assert_eq!(10, align.get_property_bottom_padding());

    //gboolean
    assert_eq!(true, combobox.get_property_has_frame());
    combobox.set_property_has_frame(false);
    assert_eq!(false, combobox.get_property_has_frame());

    //utf8
    assert_eq!(None, combobox.get_property_tearoff_title());
    combobox.set_property_tearoff_title(Some("test"));
    assert_eq!(Some("test".to_owned()), combobox.get_property_tearoff_title());

    //enum ArrowType
    assert_eq!(ArrowType::Down, arrow.get_property_arrow_type());
    arrow.set_property_arrow_type(ArrowType::Up);
    assert_eq!(ArrowType::Up, arrow.get_property_arrow_type());

    //object get
    assert_eq!(Some(cell_area), combobox.get_property_cell_area());
    //object set
    assert_eq!(0usize, vbox.get_children().len());
    vbox.set_property_child(Some(&arrow));
    assert_eq!(1usize, vbox.get_children().len());
}
