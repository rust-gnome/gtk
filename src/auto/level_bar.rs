// This file was generated by gir (28183c8) from gir-files (71d73f0)
// DO NOT EDIT

#[cfg(feature = "v3_6")]
use LevelBarMode;
use Orientable;
use Widget;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(feature = "v3_6")]
use glib::signal::connect;
use glib::translate::*;
#[cfg(feature = "v3_6")]
use glib_ffi;
#[cfg(feature = "v3_6")]
use libc;
#[cfg(feature = "v3_6")]
use std::boxed::Box as Box_;
#[cfg(feature = "v3_6")]
use std::mem;
#[cfg(feature = "v3_6")]
use std::mem::transmute;

glib_wrapper! {
    pub struct LevelBar(Object<ffi::GtkLevelBar>): Widget, Orientable;

    match fn {
        get_type => || ffi::gtk_level_bar_get_type(),
    }
}

impl LevelBar {
    #[cfg(feature = "v3_6")]
    pub fn new() -> LevelBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_level_bar_new()).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn new_for_interval(min_value: f64, max_value: f64) -> LevelBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_level_bar_new_for_interval(min_value, max_value)).downcast_unchecked()
        }
    }
}

pub trait LevelBarExt {
    #[cfg(feature = "v3_6")]
    fn add_offset_value(&self, name: &str, value: f64);

    #[cfg(feature = "v3_8")]
    fn get_inverted(&self) -> bool;

    #[cfg(feature = "v3_6")]
    fn get_max_value(&self) -> f64;

    #[cfg(feature = "v3_6")]
    fn get_min_value(&self) -> f64;

    #[cfg(feature = "v3_6")]
    fn get_mode(&self) -> LevelBarMode;

    #[cfg(feature = "v3_6")]
    fn get_offset_value<'a, P: Into<Option<&'a str>>>(&self, name: P) -> Option<f64>;

    #[cfg(feature = "v3_6")]
    fn get_value(&self) -> f64;

    #[cfg(feature = "v3_6")]
    fn remove_offset_value<'a, P: Into<Option<&'a str>>>(&self, name: P);

    #[cfg(feature = "v3_8")]
    fn set_inverted(&self, inverted: bool);

    #[cfg(feature = "v3_6")]
    fn set_max_value(&self, value: f64);

    #[cfg(feature = "v3_6")]
    fn set_min_value(&self, value: f64);

    #[cfg(feature = "v3_6")]
    fn set_mode(&self, mode: LevelBarMode);

    #[cfg(feature = "v3_6")]
    fn set_value(&self, value: f64);

    #[cfg(feature = "v3_6")]
    fn connect_offset_changed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<LevelBar> + IsA<glib::object::Object>> LevelBarExt for O {
    #[cfg(feature = "v3_6")]
    fn add_offset_value(&self, name: &str, value: f64) {
        unsafe {
            ffi::gtk_level_bar_add_offset_value(self.to_glib_none().0, name.to_glib_none().0, value);
        }
    }

    #[cfg(feature = "v3_8")]
    fn get_inverted(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_level_bar_get_inverted(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_6")]
    fn get_max_value(&self) -> f64 {
        unsafe {
            ffi::gtk_level_bar_get_max_value(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_6")]
    fn get_min_value(&self) -> f64 {
        unsafe {
            ffi::gtk_level_bar_get_min_value(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_6")]
    fn get_mode(&self) -> LevelBarMode {
        unsafe {
            from_glib(ffi::gtk_level_bar_get_mode(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_6")]
    fn get_offset_value<'a, P: Into<Option<&'a str>>>(&self, name: P) -> Option<f64> {
        let name = name.into();
        let name = name.to_glib_none();
        unsafe {
            let mut value = mem::uninitialized();
            let ret = from_glib(ffi::gtk_level_bar_get_offset_value(self.to_glib_none().0, name.0, &mut value));
            if ret { Some(value) } else { None }
        }
    }

    #[cfg(feature = "v3_6")]
    fn get_value(&self) -> f64 {
        unsafe {
            ffi::gtk_level_bar_get_value(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_6")]
    fn remove_offset_value<'a, P: Into<Option<&'a str>>>(&self, name: P) {
        let name = name.into();
        let name = name.to_glib_none();
        unsafe {
            ffi::gtk_level_bar_remove_offset_value(self.to_glib_none().0, name.0);
        }
    }

    #[cfg(feature = "v3_8")]
    fn set_inverted(&self, inverted: bool) {
        unsafe {
            ffi::gtk_level_bar_set_inverted(self.to_glib_none().0, inverted.to_glib());
        }
    }

    #[cfg(feature = "v3_6")]
    fn set_max_value(&self, value: f64) {
        unsafe {
            ffi::gtk_level_bar_set_max_value(self.to_glib_none().0, value);
        }
    }

    #[cfg(feature = "v3_6")]
    fn set_min_value(&self, value: f64) {
        unsafe {
            ffi::gtk_level_bar_set_min_value(self.to_glib_none().0, value);
        }
    }

    #[cfg(feature = "v3_6")]
    fn set_mode(&self, mode: LevelBarMode) {
        unsafe {
            ffi::gtk_level_bar_set_mode(self.to_glib_none().0, mode.to_glib());
        }
    }

    #[cfg(feature = "v3_6")]
    fn set_value(&self, value: f64) {
        unsafe {
            ffi::gtk_level_bar_set_value(self.to_glib_none().0, value);
        }
    }

    #[cfg(feature = "v3_6")]
    fn connect_offset_changed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "offset-changed",
                transmute(offset_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v3_6")]
unsafe extern "C" fn offset_changed_trampoline<P>(this: *mut ffi::GtkLevelBar, name: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<LevelBar> {
    callback_guard!();
    let f: &Box_<Fn(&P, &str) + 'static> = transmute(f);
    f(&LevelBar::from_glib_none(this).downcast_unchecked(), &String::from_glib_none(name))
}
