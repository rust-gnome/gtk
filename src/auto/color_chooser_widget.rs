// This file was generated by gir (add4ad6) from gir-files (0bcaef9)
// DO NOT EDIT

use Box;
use ColorChooser;
use Container;
use Orientable;
use Widget;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;

glib_wrapper! {
    pub struct ColorChooserWidget(Object<ffi::GtkColorChooserWidget>): Box, Container, Widget, Orientable, ColorChooser;

    match fn {
        get_type => || ffi::gtk_color_chooser_widget_get_type(),
    }
}

impl ColorChooserWidget {
    pub fn new() -> ColorChooserWidget {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_color_chooser_widget_new()).downcast_unchecked()
        }
    }
}

pub trait ColorChooserWidgetExt {
    fn get_property_show_editor(&self) -> bool;

    fn set_property_show_editor(&self, show_editor: bool);
}

impl<O: IsA<ColorChooserWidget> + IsA<glib::object::Object>> ColorChooserWidgetExt for O {
    fn get_property_show_editor(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "show-editor".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_show_editor(&self, show_editor: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "show-editor".to_glib_none().0, Value::from(&show_editor).to_glib_none().0);
        }
    }
}
