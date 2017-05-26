// This file was generated by gir (28183c8) from gir-files (71d73f0)
// DO NOT EDIT

use Actionable;
use Bin;
use Button;
use Container;
use Widget;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use signal::Inhibit;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct LinkButton(Object<ffi::GtkLinkButton>): Button, Bin, Container, Widget, Actionable;

    match fn {
        get_type => || ffi::gtk_link_button_get_type(),
    }
}

impl LinkButton {
    pub fn new(uri: &str) -> LinkButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_link_button_new(uri.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_label<'a, P: Into<Option<&'a str>>>(uri: &str, label: P) -> LinkButton {
        assert_initialized_main_thread!();
        let label = label.into();
        let label = label.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_link_button_new_with_label(uri.to_glib_none().0, label.0)).downcast_unchecked()
        }
    }
}

pub trait LinkButtonExt {
    fn get_uri(&self) -> Option<String>;

    fn get_visited(&self) -> bool;

    fn set_uri(&self, uri: &str);

    fn set_visited(&self, visited: bool);

    fn connect_activate_link<F: Fn(&Self) -> Inhibit + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<LinkButton> + IsA<glib::object::Object>> LinkButtonExt for O {
    fn get_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_link_button_get_uri(self.to_glib_none().0))
        }
    }

    fn get_visited(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_link_button_get_visited(self.to_glib_none().0))
        }
    }

    fn set_uri(&self, uri: &str) {
        unsafe {
            ffi::gtk_link_button_set_uri(self.to_glib_none().0, uri.to_glib_none().0);
        }
    }

    fn set_visited(&self, visited: bool) {
        unsafe {
            ffi::gtk_link_button_set_visited(self.to_glib_none().0, visited.to_glib());
        }
    }

    fn connect_activate_link<F: Fn(&Self) -> Inhibit + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) -> Inhibit + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate-link",
                transmute(activate_link_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_link_trampoline<P>(this: *mut ffi::GtkLinkButton, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<LinkButton> {
    callback_guard!();
    let f: &Box_<Fn(&P) -> Inhibit + 'static> = transmute(f);
    f(&LinkButton::from_glib_none(this).downcast_unchecked()).to_glib()
}
