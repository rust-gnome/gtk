// This file was generated by gir (add4ad6) from gir-files (0bcaef9)
// DO NOT EDIT

use EventController;
use Gesture;
use GestureSingle;
#[cfg(feature = "v3_14")]
use Widget;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(feature = "v3_14")]
use glib::signal::connect;
use glib::translate::*;
#[cfg(feature = "v3_14")]
use glib_ffi;
use gobject_ffi;
#[cfg(feature = "v3_14")]
use libc;
#[cfg(feature = "v3_14")]
use std::boxed::Box as Box_;
#[cfg(feature = "v3_14")]
use std::mem::transmute;

glib_wrapper! {
    pub struct GestureLongPress(Object<ffi::GtkGestureLongPress>): GestureSingle, Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_gesture_long_press_get_type(),
    }
}

impl GestureLongPress {
    #[cfg(feature = "v3_14")]
    pub fn new<P: IsA<Widget>>(widget: &P) -> GestureLongPress {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_long_press_new(widget.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait GestureLongPressExt {
    fn get_property_delay_factor(&self) -> f64;

    fn set_property_delay_factor(&self, delay_factor: f64);

    #[cfg(feature = "v3_14")]
    fn connect_cancelled<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    #[cfg(feature = "v3_14")]
    fn connect_pressed<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<GestureLongPress> + IsA<glib::object::Object>> GestureLongPressExt for O {
    fn get_property_delay_factor(&self) -> f64 {
        let mut value = Value::from(&0f64);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "delay-factor".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_delay_factor(&self, delay_factor: f64) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "delay-factor".to_glib_none().0, Value::from(&delay_factor).to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_14")]
    fn connect_cancelled<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "cancelled",
                transmute(cancelled_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_14")]
    fn connect_pressed<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, f64, f64) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "pressed",
                transmute(pressed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn cancelled_trampoline<P>(this: *mut ffi::GtkGestureLongPress, f: glib_ffi::gpointer)
where P: IsA<GestureLongPress> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&GestureLongPress::from_glib_none(this).downcast_unchecked())
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn pressed_trampoline<P>(this: *mut ffi::GtkGestureLongPress, x: libc::c_double, y: libc::c_double, f: glib_ffi::gpointer)
where P: IsA<GestureLongPress> {
    callback_guard!();
    let f: &Box_<Fn(&P, f64, f64) + 'static> = transmute(f);
    f(&GestureLongPress::from_glib_none(this).downcast_unchecked(), x, y)
}
