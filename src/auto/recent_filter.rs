// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use glib::GString;
use gtk_sys;
use std::fmt;
use Buildable;
use RecentFilterFlags;

glib_wrapper! {
    pub struct RecentFilter(Object<gtk_sys::GtkRecentFilter, RecentFilterClass>) @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_recent_filter_get_type(),
    }
}

impl RecentFilter {
    pub fn new() -> RecentFilter {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(gtk_sys::gtk_recent_filter_new()) }
    }

    pub fn add_age(&self, days: i32) {
        unsafe {
            gtk_sys::gtk_recent_filter_add_age(self.to_glib_none().0, days);
        }
    }

    pub fn add_application(&self, application: &str) {
        unsafe {
            gtk_sys::gtk_recent_filter_add_application(
                self.to_glib_none().0,
                application.to_glib_none().0,
            );
        }
    }

    //pub fn add_custom(&self, needed: RecentFilterFlags, func: /*Unimplemented*/Fn(/*Ignored*/RecentFilterInfo) -> bool, data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call gtk_sys:gtk_recent_filter_add_custom() }
    //}

    pub fn add_group(&self, group: &str) {
        unsafe {
            gtk_sys::gtk_recent_filter_add_group(self.to_glib_none().0, group.to_glib_none().0);
        }
    }

    pub fn add_mime_type(&self, mime_type: &str) {
        unsafe {
            gtk_sys::gtk_recent_filter_add_mime_type(
                self.to_glib_none().0,
                mime_type.to_glib_none().0,
            );
        }
    }

    pub fn add_pattern(&self, pattern: &str) {
        unsafe {
            gtk_sys::gtk_recent_filter_add_pattern(self.to_glib_none().0, pattern.to_glib_none().0);
        }
    }

    pub fn add_pixbuf_formats(&self) {
        unsafe {
            gtk_sys::gtk_recent_filter_add_pixbuf_formats(self.to_glib_none().0);
        }
    }

    //pub fn filter(&self, filter_info: /*Ignored*/&RecentFilterInfo) -> bool {
    //    unsafe { TODO: call gtk_sys:gtk_recent_filter_filter() }
    //}

    pub fn get_name(&self) -> Option<GString> {
        unsafe { from_glib_none(gtk_sys::gtk_recent_filter_get_name(self.to_glib_none().0)) }
    }

    pub fn get_needed(&self) -> RecentFilterFlags {
        unsafe { from_glib(gtk_sys::gtk_recent_filter_get_needed(self.to_glib_none().0)) }
    }

    pub fn set_name(&self, name: &str) {
        unsafe {
            gtk_sys::gtk_recent_filter_set_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }
}

impl Default for RecentFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for RecentFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RecentFilter")
    }
}
