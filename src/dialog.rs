// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

use auto::DialogExt;
use glib::object::Cast;
use glib::translate::*;
use glib::IsA;
use gtk_sys;
use std::ptr;
use Dialog;
use DialogFlags;
use ResponseType;
use Widget;
use Window;

impl Dialog {
    pub fn with_buttons<T: IsA<Window>>(
        title: Option<&str>,
        parent: Option<&T>,
        flags: DialogFlags,
        buttons: &[(&str, ResponseType)],
    ) -> Dialog {
        assert_initialized_main_thread!();
        let ret: Dialog = unsafe {
            Widget::from_glib_none(gtk_sys::gtk_dialog_new_with_buttons(
                title.to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                flags.to_glib(),
                ptr::null_mut(),
            ))
            .unsafe_cast()
        };

        ret.add_buttons(buttons);
        ret
    }
}

pub trait DialogExtManual: 'static {
    fn add_buttons(&self, buttons: &[(&str, ResponseType)]);
}

impl<O: IsA<Dialog>> DialogExtManual for O {
    fn add_buttons(&self, buttons: &[(&str, ResponseType)]) {
        for &(text, id) in buttons {
            //FIXME: self.add_button don't work on 1.8
            O::add_button(self, text, id);
        }
    }
}
