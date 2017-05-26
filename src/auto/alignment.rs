// This file was generated by gir (28183c8) from gir-files (71d73f0)
// DO NOT EDIT

use Bin;
use Container;
use Widget;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;
use std::mem;

glib_wrapper! {
    pub struct Alignment(Object<ffi::GtkAlignment>): Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_alignment_get_type(),
    }
}

impl Alignment {
    pub fn new(xalign: f32, yalign: f32, xscale: f32, yscale: f32) -> Alignment {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_alignment_new(xalign, yalign, xscale, yscale)).downcast_unchecked()
        }
    }
}

pub trait AlignmentExt {
    fn get_padding(&self) -> (u32, u32, u32, u32);

    fn set(&self, xalign: f32, yalign: f32, xscale: f32, yscale: f32);

    fn set_padding(&self, padding_top: u32, padding_bottom: u32, padding_left: u32, padding_right: u32);

    fn get_property_bottom_padding(&self) -> u32;

    fn set_property_bottom_padding(&self, bottom_padding: u32);

    fn get_property_left_padding(&self) -> u32;

    fn set_property_left_padding(&self, left_padding: u32);

    fn get_property_right_padding(&self) -> u32;

    fn set_property_right_padding(&self, right_padding: u32);

    fn get_property_top_padding(&self) -> u32;

    fn set_property_top_padding(&self, top_padding: u32);

    fn get_property_xalign(&self) -> f32;

    fn set_property_xalign(&self, xalign: f32);

    fn get_property_xscale(&self) -> f32;

    fn set_property_xscale(&self, xscale: f32);

    fn get_property_yalign(&self) -> f32;

    fn set_property_yalign(&self, yalign: f32);

    fn get_property_yscale(&self) -> f32;

    fn set_property_yscale(&self, yscale: f32);
}

impl<O: IsA<Alignment> + IsA<glib::object::Object>> AlignmentExt for O {
    fn get_padding(&self) -> (u32, u32, u32, u32) {
        unsafe {
            let mut padding_top = mem::uninitialized();
            let mut padding_bottom = mem::uninitialized();
            let mut padding_left = mem::uninitialized();
            let mut padding_right = mem::uninitialized();
            ffi::gtk_alignment_get_padding(self.to_glib_none().0, &mut padding_top, &mut padding_bottom, &mut padding_left, &mut padding_right);
            (padding_top, padding_bottom, padding_left, padding_right)
        }
    }

    fn set(&self, xalign: f32, yalign: f32, xscale: f32, yscale: f32) {
        unsafe {
            ffi::gtk_alignment_set(self.to_glib_none().0, xalign, yalign, xscale, yscale);
        }
    }

    fn set_padding(&self, padding_top: u32, padding_bottom: u32, padding_left: u32, padding_right: u32) {
        unsafe {
            ffi::gtk_alignment_set_padding(self.to_glib_none().0, padding_top, padding_bottom, padding_left, padding_right);
        }
    }

    fn get_property_bottom_padding(&self) -> u32 {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "bottom-padding".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_bottom_padding(&self, bottom_padding: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "bottom-padding".to_glib_none().0, Value::from(&bottom_padding).to_glib_none().0);
        }
    }

    fn get_property_left_padding(&self) -> u32 {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "left-padding".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_left_padding(&self, left_padding: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "left-padding".to_glib_none().0, Value::from(&left_padding).to_glib_none().0);
        }
    }

    fn get_property_right_padding(&self) -> u32 {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "right-padding".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_right_padding(&self, right_padding: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "right-padding".to_glib_none().0, Value::from(&right_padding).to_glib_none().0);
        }
    }

    fn get_property_top_padding(&self) -> u32 {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "top-padding".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_top_padding(&self, top_padding: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "top-padding".to_glib_none().0, Value::from(&top_padding).to_glib_none().0);
        }
    }

    fn get_property_xalign(&self) -> f32 {
        let mut value = Value::from(&0f32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "xalign".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_xalign(&self, xalign: f32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "xalign".to_glib_none().0, Value::from(&xalign).to_glib_none().0);
        }
    }

    fn get_property_xscale(&self) -> f32 {
        let mut value = Value::from(&0f32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "xscale".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_xscale(&self, xscale: f32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "xscale".to_glib_none().0, Value::from(&xscale).to_glib_none().0);
        }
    }

    fn get_property_yalign(&self) -> f32 {
        let mut value = Value::from(&0f32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "yalign".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_yalign(&self, yalign: f32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "yalign".to_glib_none().0, Value::from(&yalign).to_glib_none().0);
        }
    }

    fn get_property_yscale(&self) -> f32 {
        let mut value = Value::from(&0f32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "yscale".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_yscale(&self, yscale: f32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "yscale".to_glib_none().0, Value::from(&yscale).to_glib_none().0);
        }
    }
}
