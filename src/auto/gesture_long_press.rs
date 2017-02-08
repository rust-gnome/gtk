// This file was generated by gir (5232053) from gir-files (71d73f0)
// DO NOT EDIT

use EventController;
use Gesture;
use GestureSingle;
#[cfg(feature = "v3_14")]
use Widget;
use ffi;
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
    pub fn new<T: IsA<Widget>>(widget: &T) -> GestureLongPress {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_long_press_new(widget.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn get_property_delay_factor(&self) -> f64 {
        let mut value = Value::from(&0f64);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "delay-factor".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_property_delay_factor(&self, delay_factor: f64) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "delay-factor".to_glib_none().0, Value::from(&delay_factor).to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn connect_cancelled<F: Fn(&GestureLongPress) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&GestureLongPress) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "cancelled",
                transmute(cancelled_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn connect_pressed<F: Fn(&GestureLongPress, f64, f64) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&GestureLongPress, f64, f64) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "pressed",
                transmute(pressed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn cancelled_trampoline(this: *mut ffi::GtkGestureLongPress, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&GestureLongPress) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn pressed_trampoline(this: *mut ffi::GtkGestureLongPress, x: libc::c_double, y: libc::c_double, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&GestureLongPress, f64, f64) + 'static> = transmute(f);
    f(&from_glib_none(this), x, y)
}
