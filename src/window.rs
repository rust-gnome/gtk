// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use object::{Object, Downcast, Upcast};
use widget::Widget;

use ffi;
use {WindowPosition, WindowType};

pub type Window = Object<ffi::C_GtkWindow>;

impl Window {
    pub fn new(window_type: WindowType) -> Window {
        unsafe {
            Widget::from_glib_none(ffi::gtk_window_new(window_type)).downcast_unchecked()
        }
    }
}

pub trait WindowTrait {
    fn set_title(&self, title: &str);
    fn set_decorated(&self, setting: bool);
    fn get_title(&self) -> Option<String>;
    fn set_default_size(&self, width: i32, height: i32);
    fn set_window_position(&self, window_position: WindowPosition);
    #[cfg(feature = "gtk_3_10")]
    fn set_titlebar<T: Upcast<Widget>>(&self, titlebar: &T);
}

impl<W: Upcast<Window>> WindowTrait for W {
    fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_window_set_title(self.upcast().to_glib_none().0, title.to_glib_none().0);
        }
    }

    fn set_decorated(&self, setting: bool) {
        unsafe {
            ffi::gtk_window_set_decorated(self.upcast().to_glib_none().0, setting.to_glib());
        }
    }

    fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_window_get_title(self.upcast().to_glib_none().0))
        }
    }

    fn set_default_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::gtk_window_set_default_size(self.upcast().to_glib_none().0, width, height)
        }
    }

    fn set_window_position(&self, window_position: WindowPosition) {
        unsafe {
            ffi::gtk_window_set_position(self.upcast().to_glib_none().0, window_position);
        }
    }

    #[cfg(feature = "gtk_3_10")]
    fn set_titlebar<T: Upcast<Widget>>(&self, titlebar: &T) {
        unsafe {
            ffi::gtk_window_set_titlebar(self.upcast().to_glib_none().0,
                                         titlebar.upcast().to_glib_none().0);
        }
    }
}
