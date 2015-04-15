// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

use std::ptr;
use ffi;
use glib::translate::{FromGlibPtr};
use glib::{to_bool, to_gboolean};

pub struct Gtk;

impl Gtk {
    pub fn new() -> Gtk {
        unsafe {
            ffi::gtk_init(ptr::null(), ptr::null());
        }

        Gtk
    }

    pub fn main(&self) {
        unsafe {
            ffi::gtk_main();
        }
    }

    pub fn main_quit(&self) {
        unsafe {
            ffi::gtk_main_quit();
        }
    }

    pub fn main_level(&self) -> u32 {
        unsafe {
            ffi::gtk_main_level() as u32
        }
    }

    pub fn main_iteration(&self) -> bool {
        unsafe { to_bool(ffi::gtk_main_iteration()) }
    }

    pub fn main_iteration_do(&self, blocking: bool) -> bool {
        unsafe { to_bool(ffi::gtk_main_iteration_do(to_gboolean(blocking))) }
    }

    pub fn events_pending(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_events_pending())
        }
    }

    pub fn get_major_version(&self) -> u32 {
        unsafe {
            ffi::gtk_get_major_version() as u32
        }
    }

    pub fn get_minor_version(&self) -> u32 {
        unsafe {
            ffi::gtk_get_minor_version() as u32
        }
    }

    pub fn get_micro_version(&self) -> u32 {
        unsafe {
            ffi::gtk_get_micro_version() as u32
        }
    }

    pub fn get_binary_age(&self) -> u32 {
        unsafe {
            ffi::gtk_get_binary_age() as u32
        }
    }

    pub fn get_interface_age(&self, ) -> u32 {
        unsafe {
            ffi::gtk_get_interface_age() as u32
        }
    }

    pub fn check_version(&self, required_major: u32, required_minor: u32, required_micro: u32) ->
            Option<String> {
        unsafe {
            FromGlibPtr::borrow(ffi::gtk_check_version(required_major, required_minor,
                required_micro))
        }
    }
}

