// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use EventController;
use Gesture;
use GestureDrag;
use GestureSingle;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use Orientation;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use PanDirection;
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
use std::fmt;
use std::mem;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct GesturePan(Object<ffi::GtkGesturePan, ffi::GtkGesturePanClass>): GestureDrag, GestureSingle, Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_gesture_pan_get_type(),
    }
}

impl GesturePan {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    pub fn new<P: IsA<Widget>>(widget: &P, orientation: Orientation) -> GesturePan {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_pan_new(widget.to_glib_none().0, orientation.to_glib())).downcast_unchecked()
        }
    }
}

pub trait GesturePanExt {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_orientation(&self) -> Orientation;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_orientation(&self, orientation: Orientation);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_pan<F: Fn(&Self, PanDirection, f64) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GesturePan> + IsA<glib::object::Object>> GesturePanExt for O {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_orientation(&self) -> Orientation {
        unsafe {
            from_glib(ffi::gtk_gesture_pan_get_orientation(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_orientation(&self, orientation: Orientation) {
        unsafe {
            ffi::gtk_gesture_pan_set_orientation(self.to_glib_none().0, orientation.to_glib());
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_pan<F: Fn(&Self, PanDirection, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, PanDirection, f64) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "pan",
                transmute(pan_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::orientation",
                transmute(notify_orientation_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn pan_trampoline<P>(this: *mut ffi::GtkGesturePan, direction: ffi::GtkPanDirection, offset: libc::c_double, f: glib_ffi::gpointer)
where P: IsA<GesturePan> {
    let f: &&(Fn(&P, PanDirection, f64) + 'static) = transmute(f);
    f(&GesturePan::from_glib_borrow(this).downcast_unchecked(), from_glib(direction), offset)
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn notify_orientation_trampoline<P>(this: *mut ffi::GtkGesturePan, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<GesturePan> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GesturePan::from_glib_borrow(this).downcast_unchecked())
}

impl fmt::Display for GesturePan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GesturePan")
    }
}
