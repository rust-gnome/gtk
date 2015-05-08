// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc::c_uint;
use std::ptr;
use ffi;
use glib::translate::*;
use glib::{to_bool, to_gboolean};

pub fn init() {
    unsafe {
        ffi::gtk_init(ptr::null(), ptr::null());
    }
}

pub fn main() {
    unsafe {
        ffi::gtk_main();
    }
}

pub fn main_quit() {
    unsafe {
        ffi::gtk_main_quit();
    }
}

pub fn main_level() -> u32 {
    unsafe {
        ffi::gtk_main_level() as u32
    }
}

pub fn main_iteration() -> bool {
    unsafe { to_bool(ffi::gtk_main_iteration()) }
}

pub fn main_iteration_do(blocking: bool) -> bool {
    unsafe { to_bool(ffi::gtk_main_iteration_do(to_gboolean(blocking))) }
}

pub fn events_pending() -> bool {
    unsafe {
        to_bool(ffi::gtk_events_pending())
    }
}


pub fn get_major_version() -> u32 {
    unsafe {
        ffi::gtk_get_major_version() as u32
    }
}

pub fn get_minor_version() -> u32 {
    unsafe {
        ffi::gtk_get_minor_version() as u32
    }
}

pub fn get_micro_version() -> u32 {
    unsafe {
        ffi::gtk_get_micro_version() as u32
    }
}

pub fn get_binary_age() -> u32 {
    unsafe {
        ffi::gtk_get_binary_age() as u32
    }
}

pub fn get_interface_age() -> u32 {
    unsafe {
        ffi::gtk_get_interface_age() as u32
    }
}

pub fn check_version(required_major: u32, required_minor: u32, required_micro: u32) -> Option<String> {
    unsafe {
        from_glib_none(
            ffi::gtk_check_version(required_major as c_uint, required_minor as c_uint, required_micro as c_uint))
    }
 }
