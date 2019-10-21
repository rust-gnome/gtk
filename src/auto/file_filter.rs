// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use FileFilterFlags;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use glib;
use glib::GString;
use glib::translate::*;
use gtk_sys;
use std::fmt;

glib_wrapper! {
    pub struct FileFilter(Object<gtk_sys::GtkFileFilter, FileFilterClass>) @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_file_filter_get_type(),
    }
}

impl FileFilter {
    pub fn new() -> FileFilter {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gtk_sys::gtk_file_filter_new())
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn new_from_gvariant(variant: &glib::Variant) -> FileFilter {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_sys::gtk_file_filter_new_from_gvariant(variant.to_glib_none().0))
        }
    }

    //pub fn add_custom(&self, needed: FileFilterFlags, func: /*Unimplemented*/Fn(/*Ignored*/FileFilterInfo) -> bool, data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call gtk_sys:gtk_file_filter_add_custom() }
    //}

    pub fn add_mime_type(&self, mime_type: &str) {
        unsafe {
            gtk_sys::gtk_file_filter_add_mime_type(self.to_glib_none().0, mime_type.to_glib_none().0);
        }
    }

    pub fn add_pattern(&self, pattern: &str) {
        unsafe {
            gtk_sys::gtk_file_filter_add_pattern(self.to_glib_none().0, pattern.to_glib_none().0);
        }
    }

    pub fn add_pixbuf_formats(&self) {
        unsafe {
            gtk_sys::gtk_file_filter_add_pixbuf_formats(self.to_glib_none().0);
        }
    }

    //pub fn filter(&self, filter_info: /*Ignored*/&FileFilterInfo) -> bool {
    //    unsafe { TODO: call gtk_sys:gtk_file_filter_filter() }
    //}

    pub fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_file_filter_get_name(self.to_glib_none().0))
        }
    }

    pub fn get_needed(&self) -> FileFilterFlags {
        unsafe {
            from_glib(gtk_sys::gtk_file_filter_get_needed(self.to_glib_none().0))
        }
    }

    pub fn set_name(&self, name: Option<&str>) {
        unsafe {
            gtk_sys::gtk_file_filter_set_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn to_gvariant(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(gtk_sys::gtk_file_filter_to_gvariant(self.to_glib_none().0))
        }
    }
}

impl fmt::Display for FileFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FileFilter")
    }
}
