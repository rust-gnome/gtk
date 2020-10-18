// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

use glib::object::{Cast, IsA};
use glib::translate::*;
use glib::GString;
use glib::Object;
use glib::ObjectExt;
use gtk_sys;
use std::path::Path;
use Builder;

impl Builder {
    pub fn from_file<T: AsRef<Path>>(file_path: T) -> Builder {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_sys::gtk_builder_new_from_file(
                file_path.as_ref().to_glib_none().0,
            ))
        }
    }
}

pub trait BuilderExtManual: 'static {
    fn get_object<T: IsA<Object>>(&self, name: &str) -> Option<T>;
    fn add_from_file<T: AsRef<Path>>(&self, file_path: T) -> Result<(), glib::Error>;
    fn connect_signals<
        P: FnMut(&Builder, &str) -> Box<dyn Fn(&[glib::Value]) -> Option<glib::Value> + 'static>,
    >(
        &self,
        func: P,
    );
}

impl<O: IsA<Builder>> BuilderExtManual for O {
    fn get_object<T: IsA<Object>>(&self, name: &str) -> Option<T> {
        unsafe {
            Option::<Object>::from_glib_none(gtk_sys::gtk_builder_get_object(
                self.upcast_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
            .and_then(|obj| obj.dynamic_cast::<T>().ok())
        }
    }

    fn add_from_file<T: AsRef<Path>>(&self, file_path: T) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ::std::ptr::null_mut();
            gtk_sys::gtk_builder_add_from_file(
                self.upcast_ref().to_glib_none().0,
                file_path.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn connect_signals<
        P: FnMut(&Builder, &str) -> Box<dyn Fn(&[glib::Value]) -> Option<glib::Value> + 'static>,
    >(
        &self,
        func: P,
    ) {
        let func_data: P = func;
        unsafe extern "C" fn func_func<
            P: FnMut(&Builder, &str) -> Box<dyn Fn(&[glib::Value]) -> Option<glib::Value> + 'static>,
        >(
            builder: *mut gtk_sys::GtkBuilder,
            object: *mut gobject_sys::GObject,
            signal_name: *const libc::c_char,
            handler_name: *const libc::c_char,
            connect_object: *mut gobject_sys::GObject,
            flags: gobject_sys::GConnectFlags,
            user_data: glib_sys::gpointer,
        ) {
            assert!(connect_object.is_null(), "Connect object is not supported");
            assert!(
                flags & gobject_sys::G_CONNECT_SWAPPED == 0,
                "Swapped signal handler is not supported"
            );

            let builder = from_glib_borrow(builder);
            let object: Borrowed<glib::Object> = from_glib_borrow(object);
            let signal_name: Borrowed<GString> = from_glib_borrow(signal_name);
            let handler_name: Borrowed<GString> = from_glib_borrow(handler_name);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            let func = (*callback)(&builder, handler_name.as_str());
            object
                .connect_unsafe(
                    signal_name.as_str(),
                    flags & gobject_sys::G_CONNECT_AFTER != 0,
                    move |args| func(args),
                )
                .expect("Failed to connect to builder signal");
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            gtk_sys::gtk_builder_connect_signals_full(
                self.as_ref().to_glib_none().0,
                func,
                super_callback0 as *const _ as usize as *mut _,
            );
        }
    }
}
