// This file was generated by gir (12a28ac) from gir-files (469db10)
// DO NOT EDIT

use Error;
use IconInfo;
use IconLookupFlags;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use cairo;
use ffi;
use gdk;
use gdk_pixbuf;
use gio;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct IconTheme(Object<ffi::GtkIconTheme>);

    match fn {
        get_type => || ffi::gtk_icon_theme_get_type(),
    }
}

impl IconTheme {
    pub fn new() -> IconTheme {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_icon_theme_new())
        }
    }

    pub fn add_builtin_icon(icon_name: &str, size: i32, pixbuf: &gdk_pixbuf::Pixbuf) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_icon_theme_add_builtin_icon(icon_name.to_glib_none().0, size, pixbuf.to_glib_none().0);
        }
    }

    pub fn get_default() -> Option<IconTheme> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_icon_theme_get_default())
        }
    }

    pub fn get_for_screen(screen: &gdk::Screen) -> Option<IconTheme> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_icon_theme_get_for_screen(screen.to_glib_none().0))
        }
    }
}

impl Default for IconTheme {
    fn default() -> Self {
        Self::new()
    }
}

pub trait IconThemeExt {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn add_resource_path(&self, path: &str);

    fn append_search_path<P: AsRef<std::path::Path>>(&self, path: P);

    fn get_example_icon_name(&self) -> Option<String>;

    fn has_icon(&self, icon_name: &str) -> bool;

    fn list_contexts(&self) -> Vec<String>;

    fn list_icons<'a, P: Into<Option<&'a str>>>(&self, context: P) -> Vec<String>;

    fn load_icon(&self, icon_name: &str, size: i32, flags: IconLookupFlags) -> Result<Option<gdk_pixbuf::Pixbuf>, Error>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn load_icon_for_scale(&self, icon_name: &str, size: i32, scale: i32, flags: IconLookupFlags) -> Result<Option<gdk_pixbuf::Pixbuf>, Error>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn load_surface<'a, P: Into<Option<&'a gdk::Window>>>(&self, icon_name: &str, size: i32, scale: i32, for_window: P, flags: IconLookupFlags) -> Result<Option<cairo::Surface>, Error>;

    fn lookup_by_gicon<P: IsA<gio::Icon>>(&self, icon: &P, size: i32, flags: IconLookupFlags) -> Option<IconInfo>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn lookup_by_gicon_for_scale<P: IsA<gio::Icon>>(&self, icon: &P, size: i32, scale: i32, flags: IconLookupFlags) -> Option<IconInfo>;

    fn lookup_icon(&self, icon_name: &str, size: i32, flags: IconLookupFlags) -> Option<IconInfo>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn lookup_icon_for_scale(&self, icon_name: &str, size: i32, scale: i32, flags: IconLookupFlags) -> Option<IconInfo>;

    fn prepend_search_path<P: AsRef<std::path::Path>>(&self, path: P);

    fn rescan_if_needed(&self) -> bool;

    fn set_custom_theme<'a, P: Into<Option<&'a str>>>(&self, theme_name: P);

    fn set_screen(&self, screen: &gdk::Screen);

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<IconTheme> + IsA<glib::object::Object>> IconThemeExt for O {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn add_resource_path(&self, path: &str) {
        unsafe {
            ffi::gtk_icon_theme_add_resource_path(self.to_glib_none().0, path.to_glib_none().0);
        }
    }

    fn append_search_path<P: AsRef<std::path::Path>>(&self, path: P) {
        unsafe {
            ffi::gtk_icon_theme_append_search_path(self.to_glib_none().0, path.as_ref().to_glib_none().0);
        }
    }

    fn get_example_icon_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_icon_theme_get_example_icon_name(self.to_glib_none().0))
        }
    }

    fn has_icon(&self, icon_name: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_icon_theme_has_icon(self.to_glib_none().0, icon_name.to_glib_none().0))
        }
    }

    fn list_contexts(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_icon_theme_list_contexts(self.to_glib_none().0))
        }
    }

    fn list_icons<'a, P: Into<Option<&'a str>>>(&self, context: P) -> Vec<String> {
        let context = context.into();
        let context = context.to_glib_none();
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_icon_theme_list_icons(self.to_glib_none().0, context.0))
        }
    }

    fn load_icon(&self, icon_name: &str, size: i32, flags: IconLookupFlags) -> Result<Option<gdk_pixbuf::Pixbuf>, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_icon_theme_load_icon(self.to_glib_none().0, icon_name.to_glib_none().0, size, flags.to_glib(), &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn load_icon_for_scale(&self, icon_name: &str, size: i32, scale: i32, flags: IconLookupFlags) -> Result<Option<gdk_pixbuf::Pixbuf>, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_icon_theme_load_icon_for_scale(self.to_glib_none().0, icon_name.to_glib_none().0, size, scale, flags.to_glib(), &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn load_surface<'a, P: Into<Option<&'a gdk::Window>>>(&self, icon_name: &str, size: i32, scale: i32, for_window: P, flags: IconLookupFlags) -> Result<Option<cairo::Surface>, Error> {
        let for_window = for_window.into();
        let for_window = for_window.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_icon_theme_load_surface(self.to_glib_none().0, icon_name.to_glib_none().0, size, scale, for_window.0, flags.to_glib(), &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn lookup_by_gicon<P: IsA<gio::Icon>>(&self, icon: &P, size: i32, flags: IconLookupFlags) -> Option<IconInfo> {
        unsafe {
            from_glib_full(ffi::gtk_icon_theme_lookup_by_gicon(self.to_glib_none().0, icon.to_glib_none().0, size, flags.to_glib()))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn lookup_by_gicon_for_scale<P: IsA<gio::Icon>>(&self, icon: &P, size: i32, scale: i32, flags: IconLookupFlags) -> Option<IconInfo> {
        unsafe {
            from_glib_full(ffi::gtk_icon_theme_lookup_by_gicon_for_scale(self.to_glib_none().0, icon.to_glib_none().0, size, scale, flags.to_glib()))
        }
    }

    fn lookup_icon(&self, icon_name: &str, size: i32, flags: IconLookupFlags) -> Option<IconInfo> {
        unsafe {
            from_glib_full(ffi::gtk_icon_theme_lookup_icon(self.to_glib_none().0, icon_name.to_glib_none().0, size, flags.to_glib()))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn lookup_icon_for_scale(&self, icon_name: &str, size: i32, scale: i32, flags: IconLookupFlags) -> Option<IconInfo> {
        unsafe {
            from_glib_full(ffi::gtk_icon_theme_lookup_icon_for_scale(self.to_glib_none().0, icon_name.to_glib_none().0, size, scale, flags.to_glib()))
        }
    }

    fn prepend_search_path<P: AsRef<std::path::Path>>(&self, path: P) {
        unsafe {
            ffi::gtk_icon_theme_prepend_search_path(self.to_glib_none().0, path.as_ref().to_glib_none().0);
        }
    }

    fn rescan_if_needed(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_icon_theme_rescan_if_needed(self.to_glib_none().0))
        }
    }

    fn set_custom_theme<'a, P: Into<Option<&'a str>>>(&self, theme_name: P) {
        let theme_name = theme_name.into();
        let theme_name = theme_name.to_glib_none();
        unsafe {
            ffi::gtk_icon_theme_set_custom_theme(self.to_glib_none().0, theme_name.0);
        }
    }

    fn set_screen(&self, screen: &gdk::Screen) {
        unsafe {
            ffi::gtk_icon_theme_set_screen(self.to_glib_none().0, screen.to_glib_none().0);
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "changed",
                transmute(changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn changed_trampoline<P>(this: *mut ffi::GtkIconTheme, f: glib_ffi::gpointer)
where P: IsA<IconTheme> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IconTheme::from_glib_borrow(this).downcast_unchecked())
}
