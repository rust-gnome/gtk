// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib;
use glib::translate::*;

pub struct WidgetClass(*mut ffi::GtkWidgetClass);

impl WidgetClass {
    pub fn set_accessible_type(&self, type_: glib::types::Type) {
        unsafe {
            ffi::gtk_widget_class_set_accessible_type(self.to_glib(), type_.to_glib())
        }
    }

    // pub fn set_accessible_role(&self, role: atk::Role);
    // pub fn get_accessible_role(&self) -> role: atk::Role;

    pub fn get_css_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_widget_class_get_css_name(self.to_glib()))
        }
    }

    pub fn set_css_name<'a, I: Into<Option<&'a str>>>(&self, name: I) {
        unsafe {
            ffi::gtk_widget_class_set_css_name(self.to_glib(), name.into().to_glib_none().0)
        }
    }

    // pub fn set_template(&self, template_bytes: glib::Bytes);

    pub fn set_template_from_resource<'a, I: Into<Option<&'a str>>>(&self, resource_name: I) {
        unsafe {
            ffi::gtk_widget_class_set_template_from_resource(self.to_glib(),
                                                             resource_name.into().to_glib_none().0)
        }
    }

    // pub fn bind_template_child_full(&self, name: &str, internal_child: bool,
    //                                 struct_offset: gssize);
    // pub fn bind_template_callback_full(&self, callback_name: &str,
    //                                    callback_symbol: glib::Callback);
    // pub fn set_connect_func(&self, connect_func: /*callback*/, data: *mut c_void,
    //                         connect_data_destroy: glib::DestroyNotify);
}

#[doc(hidden)]
impl ToGlib for WidgetClass {
    type GlibType = *mut ffi::GtkWidgetClass;

    fn to_glib(&self) -> *mut ffi::GtkWidgetClass {
        self.0
    }
}

#[doc(hidden)]
impl FromGlib<*mut ffi::GtkWidgetClass> for WidgetClass {
    fn from_glib(value: *mut ffi::GtkWidgetClass) -> Self {
        skip_assert_initialized!();
        WidgetClass(value)
    }
}
