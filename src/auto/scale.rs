// This file was generated by gir (add4ad6) from gir-files (0bcaef9)
// DO NOT EDIT

use Adjustment;
use Orientable;
use Orientation;
use PositionType;
use Range;
use Widget;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use libc;
use pango;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;

glib_wrapper! {
    pub struct Scale(Object<ffi::GtkScale>): Range, Widget, Orientable;

    match fn {
        get_type => || ffi::gtk_scale_get_type(),
    }
}

impl Scale {
    pub fn new<'a, P: Into<Option<&'a Adjustment>>>(orientation: Orientation, adjustment: P) -> Scale {
        assert_initialized_main_thread!();
        let adjustment = adjustment.into();
        let adjustment = adjustment.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_scale_new(orientation, adjustment.0)).downcast_unchecked()
        }
    }

    pub fn new_with_range(orientation: Orientation, min: f64, max: f64, step: f64) -> Scale {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_scale_new_with_range(orientation, min, max, step)).downcast_unchecked()
        }
    }
}

pub trait ScaleExt {
    fn add_mark<'a, P: Into<Option<&'a str>>>(&self, value: f64, position: PositionType, markup: P);

    fn clear_marks(&self);

    fn get_digits(&self) -> i32;

    fn get_draw_value(&self) -> bool;

    fn get_has_origin(&self) -> bool;

    fn get_layout(&self) -> Option<pango::Layout>;

    fn get_layout_offsets(&self) -> (i32, i32);

    fn get_value_pos(&self) -> PositionType;

    fn set_digits(&self, digits: i32);

    fn set_draw_value(&self, draw_value: bool);

    fn set_has_origin(&self, has_origin: bool);

    fn set_value_pos(&self, pos: PositionType);

    fn connect_format_value<F: Fn(&Self, f64) -> String + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<Scale> + IsA<glib::object::Object>> ScaleExt for O {
    fn add_mark<'a, P: Into<Option<&'a str>>>(&self, value: f64, position: PositionType, markup: P) {
        let markup = markup.into();
        let markup = markup.to_glib_none();
        unsafe {
            ffi::gtk_scale_add_mark(self.to_glib_none().0, value, position, markup.0);
        }
    }

    fn clear_marks(&self) {
        unsafe {
            ffi::gtk_scale_clear_marks(self.to_glib_none().0);
        }
    }

    fn get_digits(&self) -> i32 {
        unsafe {
            ffi::gtk_scale_get_digits(self.to_glib_none().0)
        }
    }

    fn get_draw_value(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_scale_get_draw_value(self.to_glib_none().0))
        }
    }

    fn get_has_origin(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_scale_get_has_origin(self.to_glib_none().0))
        }
    }

    fn get_layout(&self) -> Option<pango::Layout> {
        unsafe {
            from_glib_none(ffi::gtk_scale_get_layout(self.to_glib_none().0))
        }
    }

    fn get_layout_offsets(&self) -> (i32, i32) {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            ffi::gtk_scale_get_layout_offsets(self.to_glib_none().0, &mut x, &mut y);
            (x, y)
        }
    }

    fn get_value_pos(&self) -> PositionType {
        unsafe {
            ffi::gtk_scale_get_value_pos(self.to_glib_none().0)
        }
    }

    fn set_digits(&self, digits: i32) {
        unsafe {
            ffi::gtk_scale_set_digits(self.to_glib_none().0, digits);
        }
    }

    fn set_draw_value(&self, draw_value: bool) {
        unsafe {
            ffi::gtk_scale_set_draw_value(self.to_glib_none().0, draw_value.to_glib());
        }
    }

    fn set_has_origin(&self, has_origin: bool) {
        unsafe {
            ffi::gtk_scale_set_has_origin(self.to_glib_none().0, has_origin.to_glib());
        }
    }

    fn set_value_pos(&self, pos: PositionType) {
        unsafe {
            ffi::gtk_scale_set_value_pos(self.to_glib_none().0, pos);
        }
    }

    fn connect_format_value<F: Fn(&Self, f64) -> String + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, f64) -> String + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "format-value",
                transmute(format_value_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn format_value_trampoline<P>(this: *mut ffi::GtkScale, value: libc::c_double, f: glib_ffi::gpointer) -> *mut libc::c_char
where P: IsA<Scale> {
    callback_guard!();
    let f: &Box_<Fn(&P, f64) -> String + 'static> = transmute(f);
    f(&Scale::from_glib_none(this).downcast_unchecked(), value).to_glib_full()
}
