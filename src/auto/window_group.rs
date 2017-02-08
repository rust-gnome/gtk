// This file was generated by gir (5232053) from gir-files (71d73f0)
// DO NOT EDIT

use Widget;
use Window;
use ffi;
use gdk;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct WindowGroup(Object<ffi::GtkWindowGroup>);

    match fn {
        get_type => || ffi::gtk_window_group_get_type(),
    }
}

impl WindowGroup {
    pub fn new() -> WindowGroup {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_window_group_new())
        }
    }

    pub fn add_window<T: IsA<Window>>(&self, window: &T) {
        unsafe {
            ffi::gtk_window_group_add_window(self.to_glib_none().0, window.to_glib_none().0);
        }
    }

    pub fn get_current_device_grab<T: IsA<gdk::Device>>(&self, device: &T) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_window_group_get_current_device_grab(self.to_glib_none().0, device.to_glib_none().0))
        }
    }

    pub fn get_current_grab(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_window_group_get_current_grab(self.to_glib_none().0))
        }
    }

    pub fn list_windows(&self) -> Vec<Window> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_window_group_list_windows(self.to_glib_none().0))
        }
    }

    pub fn remove_window<T: IsA<Window>>(&self, window: &T) {
        unsafe {
            ffi::gtk_window_group_remove_window(self.to_glib_none().0, window.to_glib_none().0);
        }
    }
}
