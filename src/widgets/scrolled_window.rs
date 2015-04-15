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
use cast::GTK_SCROLLED_WINDOW;

/// GtkScrolledWindow — Adds scrollbars to its child widget
struct_Widget!(ScrolledWindow);

pub trait ScrolledWindowBuilder {
    fn scrolled_window(&self, h_adjustment: Option<::Adjustment>,
        v_adjustment: Option<::Adjustment>) -> Option<ScrolledWindow>;
}

impl ScrolledWindowBuilder for ::Gtk {
    fn scrolled_window(&self, h_adjustment: Option<::Adjustment>,
            v_adjustment: Option<::Adjustment>) -> Option<ScrolledWindow> {
        let tmp_pointer = unsafe {
            ffi::gtk_scrolled_window_new(
                h_adjustment.map_or(ptr::null_mut(), |p| { p.unwrap_pointer() }),
                v_adjustment.map_or(ptr::null_mut(), |p| { p.unwrap_pointer() })
            )
        };

        check_pointer!(tmp_pointer, ScrolledWindow)
    }
}

impl ScrolledWindow {
    pub fn get_min_content_width(&self) -> i32 {
        unsafe { ffi::gtk_scrolled_window_get_min_content_width(GTK_SCROLLED_WINDOW(self.pointer)) }
    }

    pub fn set_min_content_width(&self, width: i32) {
        unsafe { ffi::gtk_scrolled_window_set_min_content_width(GTK_SCROLLED_WINDOW(self.pointer), width) }
    }

    pub fn get_min_content_height(&self) -> i32 {
        unsafe { ffi::gtk_scrolled_window_get_min_content_height(GTK_SCROLLED_WINDOW(self.pointer)) }
    }

    pub fn set_min_content_height(&self, height: i32) {
        unsafe { ffi::gtk_scrolled_window_set_min_content_height(GTK_SCROLLED_WINDOW(self.pointer), height) }
    }
}

impl_drop!(ScrolledWindow);
impl_TraitWidget!(ScrolledWindow);

impl ::ScrolledWindowTrait for ScrolledWindow {}
impl ::ContainerTrait for ScrolledWindow {}

impl_widget_events!(ScrolledWindow);

