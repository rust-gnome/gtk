// This file was generated by gir (12a28ac) from gir-files (469db10)
// DO NOT EDIT

use EventController;
use Gesture;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use Widget;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use libc;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct GestureZoom(Object<ffi::GtkGestureZoom>): Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_gesture_zoom_get_type(),
    }
}

impl GestureZoom {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    pub fn new<P: IsA<Widget>>(widget: &P) -> GestureZoom {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_zoom_new(widget.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait GestureZoomExt {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_scale_delta(&self) -> f64;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_scale_changed<F: Fn(&Self, f64) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GestureZoom> + IsA<glib::object::Object>> GestureZoomExt for O {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_scale_delta(&self) -> f64 {
        unsafe {
            ffi::gtk_gesture_zoom_get_scale_delta(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_scale_changed<F: Fn(&Self, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, f64) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "scale-changed",
                transmute(scale_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn scale_changed_trampoline<P>(this: *mut ffi::GtkGestureZoom, scale: libc::c_double, f: glib_ffi::gpointer)
where P: IsA<GestureZoom> {
    callback_guard!();
    let f: &&(Fn(&P, f64) + 'static) = transmute(f);
    f(&GestureZoom::from_glib_borrow(this).downcast_unchecked(), scale)
}
