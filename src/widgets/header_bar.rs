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

//! A Box::new(with) a centered child

// FIXME: add missing methods (3.12)

use cast::{GTK_HEADER_BAR};
use ffi;
use glib::translate::{FromGlibPtr, ToGlibPtr};
use glib::{to_bool, to_gboolean};

/// GtkHeaderBar — A Box::new(with) a centered child
struct_Widget!(HeaderBar);

pub trait HeaderBar {
    fn header_bar(&self) -> Option<HeaderBar>;
}

impl HeaderBar {
    fn header_bar(&self) -> Option<HeaderBar> {
        let tmp_pointer = unsafe { ffi::gtk_header_bar_new() };
        check_pointer!(tmp_pointer, HeaderBar)
    }
}

impl HeaderBar {
    pub fn set_title(&mut self, title: &str) {
        unsafe {
            ffi::gtk_header_bar_set_title(GTK_HEADER_BAR(self.pointer),
                                          title.borrow_to_glib().0)
        }
    }

    pub fn get_title(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_header_bar_get_title(GTK_HEADER_BAR(self.pointer)))
        }
    }

    pub fn set_subtitle(&mut self, subtitle: &str) {
        unsafe {
            ffi::gtk_header_bar_set_subtitle(GTK_HEADER_BAR(self.pointer),
                                             subtitle.borrow_to_glib().0)
        }
    }

    pub fn get_subtitle(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_header_bar_get_title(GTK_HEADER_BAR(self.pointer)))
        }
    }

    pub fn set_custom_title<T: ::WidgetTrait>(&mut self, title_widget: Option<&T>) {
        unsafe {
            ffi::gtk_header_bar_set_custom_title(GTK_HEADER_BAR(self.pointer),
                                                 unwrap_widget!(title_widget))
        }
    }

    pub fn get_custom_title<T: ::WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = unsafe {
            ffi::gtk_header_bar_get_custom_title(GTK_HEADER_BAR(self.pointer))
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn pack_start<T: ::WidgetTrait>(&mut self, child: &T) {
        unsafe {
            ffi::gtk_header_bar_pack_start(GTK_HEADER_BAR(self.pointer),
                                           child.unwrap_widget())
        }
    }

    pub fn pack_end<T: ::WidgetTrait>(&mut self, child: &T) {
        unsafe {
            ffi::gtk_header_bar_pack_end(GTK_HEADER_BAR(self.pointer),
                                         child.unwrap_widget())
        }
    }

    pub fn is_show_close_button(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_header_bar_get_show_close_button(GTK_HEADER_BAR(self.pointer)))
        }
    }

    pub fn set_show_close_button(&mut self, setting: bool) {
        unsafe {
            ffi::gtk_header_bar_set_show_close_button(GTK_HEADER_BAR(self.pointer),
                                                      to_gboolean(setting))
        }
    }
}

impl_drop!(HeaderBar);
impl_TraitWidget!(HeaderBar);

impl ::ContainerTrait for HeaderBar {}

impl_widget_events!(HeaderBar);

