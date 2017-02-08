// This file was generated by gir (5232053) from gir-files (71d73f0)
// DO NOT EDIT

use Box;
use Container;
use Orientable;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct Statusbar(Object<ffi::GtkStatusbar>): Box, Container, Widget, Orientable;

    match fn {
        get_type => || ffi::gtk_statusbar_get_type(),
    }
}

impl Statusbar {
    pub fn new() -> Statusbar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_statusbar_new()).downcast_unchecked()
        }
    }

    pub fn get_context_id(&self, context_description: &str) -> u32 {
        unsafe {
            ffi::gtk_statusbar_get_context_id(self.to_glib_none().0, context_description.to_glib_none().0)
        }
    }

    pub fn get_message_area(&self) -> Option<Box> {
        unsafe {
            from_glib_none(ffi::gtk_statusbar_get_message_area(self.to_glib_none().0))
        }
    }

    pub fn pop(&self, context_id: u32) {
        unsafe {
            ffi::gtk_statusbar_pop(self.to_glib_none().0, context_id);
        }
    }

    pub fn push(&self, context_id: u32, text: &str) -> u32 {
        unsafe {
            ffi::gtk_statusbar_push(self.to_glib_none().0, context_id, text.to_glib_none().0)
        }
    }

    pub fn remove(&self, context_id: u32, message_id: u32) {
        unsafe {
            ffi::gtk_statusbar_remove(self.to_glib_none().0, context_id, message_id);
        }
    }

    pub fn remove_all(&self, context_id: u32) {
        unsafe {
            ffi::gtk_statusbar_remove_all(self.to_glib_none().0, context_id);
        }
    }

    pub fn connect_text_popped<F: Fn(&Statusbar, u32, &str) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Statusbar, u32, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "text-popped",
                transmute(text_popped_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_text_pushed<F: Fn(&Statusbar, u32, &str) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Statusbar, u32, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "text-pushed",
                transmute(text_pushed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn text_popped_trampoline(this: *mut ffi::GtkStatusbar, context_id: libc::c_uint, text: *mut libc::c_char, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Statusbar, u32, &str) + 'static> = transmute(f);
    f(&from_glib_none(this), context_id, &String::from_glib_none(text))
}

unsafe extern "C" fn text_pushed_trampoline(this: *mut ffi::GtkStatusbar, context_id: libc::c_uint, text: *mut libc::c_char, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Statusbar, u32, &str) + 'static> = transmute(f);
    f(&from_glib_none(this), context_id, &String::from_glib_none(text))
}
