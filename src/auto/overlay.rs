// This file was generated by gir (12a28ac) from gir-files (469db10)
// DO NOT EDIT

use Bin;
use Container;
use Widget;
use ffi;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Overlay(Object<ffi::GtkOverlay>): Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_overlay_get_type(),
    }
}

impl Overlay {
    pub fn new() -> Overlay {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_overlay_new()).downcast_unchecked()
        }
    }
}

impl Default for Overlay {
    fn default() -> Self {
        Self::new()
    }
}

pub trait OverlayExt {
    fn add_overlay<P: IsA<Widget>>(&self, widget: &P);

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_overlay_pass_through<P: IsA<Widget>>(&self, widget: &P) -> bool;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn reorder_overlay<P: IsA<Widget>>(&self, child: &P, position: i32);

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn set_overlay_pass_through<P: IsA<Widget>>(&self, widget: &P, pass_through: bool);

    fn get_child_index<T: IsA<Widget>>(&self, item: &T) -> i32;

    fn set_child_index<T: IsA<Widget>>(&self, item: &T, index: i32);

    //fn connect_get_child_position<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Overlay> + IsA<Container>> OverlayExt for O {
    fn add_overlay<P: IsA<Widget>>(&self, widget: &P) {
        unsafe {
            ffi::gtk_overlay_add_overlay(self.to_glib_none().0, widget.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_overlay_pass_through<P: IsA<Widget>>(&self, widget: &P) -> bool {
        unsafe {
            from_glib(ffi::gtk_overlay_get_overlay_pass_through(self.to_glib_none().0, widget.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn reorder_overlay<P: IsA<Widget>>(&self, child: &P, position: i32) {
        unsafe {
            ffi::gtk_overlay_reorder_overlay(self.to_glib_none().0, child.to_glib_none().0, position);
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn set_overlay_pass_through<P: IsA<Widget>>(&self, widget: &P, pass_through: bool) {
        unsafe {
            ffi::gtk_overlay_set_overlay_pass_through(self.to_glib_none().0, widget.to_glib_none().0, pass_through.to_glib());
        }
    }

    fn get_child_index<T: IsA<Widget>>(&self, item: &T) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "index".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_child_index<T: IsA<Widget>>(&self, item: &T, index: i32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "index".to_glib_none().0, Value::from(&index).to_glib_none().0);
        }
    }

    //fn connect_get_child_position<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Out allocation: Gdk.Rectangle
    //}
}
