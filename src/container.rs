// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use object::{Object, Upcast};
use widget::Widget;
use ffi;

use ResizeMode;

pub type Container = Object<ffi::C_GtkContainer>;

pub trait ContainerTrait {
    fn add<T: Upcast<Widget>>(&self, widget: &T);
    fn remove<T: Upcast<Widget>>(&self, widget: &T);
    fn get_resize_mode(&self) -> ResizeMode;
    fn set_resize_mode(&self, resize_mode: ResizeMode);
    fn get_border_width(&self) -> u32;
    fn set_border_width(&self, border_width: u32);
    fn get_children(&self) -> Vec<Widget>;
}

impl<W: Upcast<Container>> ContainerTrait for W {
    fn add<T: Upcast<Widget>>(&self, widget: &T) {
        unsafe {
            ffi::gtk_container_add(self.upcast().to_glib_none().0, widget.upcast().to_glib_none().0);
        }
    }

    fn remove<T: Upcast<Widget>>(&self, widget: &T) {
        unsafe {
            ffi::gtk_container_remove(self.upcast().to_glib_none().0, widget.upcast().to_glib_none().0);
        }
    }

    fn get_resize_mode(&self) -> ResizeMode {
        unsafe {
            ffi::gtk_container_get_resize_mode(self.upcast().to_glib_none().0)
        }
    }

    fn set_resize_mode(&self, resize_mode: ResizeMode) {
        unsafe {
            ffi::gtk_container_set_resize_mode(self.upcast().to_glib_none().0, resize_mode);
        }
    }

    fn get_border_width(&self) -> u32 {
        unsafe {
            ffi::gtk_container_get_border_width(self.upcast().to_glib_none().0) as u32
        }
    }

    fn set_border_width(&self, border_width: u32) {
        unsafe {
            ffi::gtk_container_set_border_width(self.upcast().to_glib_none().0, border_width);
        }
    }

    fn get_children(&self) -> Vec<Widget> {
        unsafe {
            Vec::from_glib_container(
                ffi::gtk_container_get_children(self.upcast().to_glib_none().0))
        }
    }
}
