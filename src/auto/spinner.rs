// This file was generated by gir (28183c8) from gir-files (71d73f0)
// DO NOT EDIT

use Widget;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;

glib_wrapper! {
    pub struct Spinner(Object<ffi::GtkSpinner>): Widget;

    match fn {
        get_type => || ffi::gtk_spinner_get_type(),
    }
}

impl Spinner {
    pub fn new() -> Spinner {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_spinner_new()).downcast_unchecked()
        }
    }
}

pub trait SpinnerExt {
    fn start(&self);

    fn stop(&self);

    fn get_property_active(&self) -> bool;

    fn set_property_active(&self, active: bool);
}

impl<O: IsA<Spinner> + IsA<glib::object::Object>> SpinnerExt for O {
    fn start(&self) {
        unsafe {
            ffi::gtk_spinner_start(self.to_glib_none().0);
        }
    }

    fn stop(&self) {
        unsafe {
            ffi::gtk_spinner_stop(self.to_glib_none().0);
        }
    }

    fn get_property_active(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "active".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_active(&self, active: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "active".to_glib_none().0, Value::from(&active).to_glib_none().0);
        }
    }
}
