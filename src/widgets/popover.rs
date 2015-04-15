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

//! GtkPopover — Context dependent bubbles

use ffi;
use cast::GTK_POPOVER;
use glib::{to_bool, to_gboolean};
// use std::string;

struct_Widget!(Popover);

pub trait PopoverBuilder {
    fn popover<T: ::WidgetTrait>(&self, relative_to: &T) -> Option<Popover>;
}

impl PopoverBuilder for ::Gtk {
    fn popover<T: ::WidgetTrait>(&self, relative_to: &T) -> Option<Popover> {
        let tmp_pointer = unsafe { ffi::gtk_popover_new(relative_to.unwrap_widget()) };
        check_pointer!(tmp_pointer, Popover)
    }
}

impl Popover {
    pub fn set_relative_to<T: ::WidgetTrait>(&self, relative_to: &T) {
        unsafe { ffi::gtk_popover_set_relative_to(GTK_POPOVER(self.pointer), relative_to.unwrap_widget()) }
    }

    pub fn get_relative_to<T: ::WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = unsafe { ffi::gtk_popover_get_relative_to(GTK_POPOVER(self.pointer)) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn set_position(&self, position: ::PositionType) {
        unsafe { ffi::gtk_popover_set_position(GTK_POPOVER(self.pointer), position) }
    }

    pub fn get_position(&self) -> ::PositionType {
        unsafe { ffi::gtk_popover_get_position(GTK_POPOVER(self.pointer)) }
    }

    pub fn set_modal(&self, modal: bool) {
        unsafe { ffi::gtk_popover_set_modal(GTK_POPOVER(self.pointer), to_gboolean(modal)) }
    }

    pub fn get_modal(&self) -> bool {
        unsafe { to_bool(ffi::gtk_popover_get_modal(GTK_POPOVER(self.pointer))) }
    }
}

impl_drop!(Popover);
impl_TraitWidget!(Popover);

impl ::ContainerTrait for Popover {}
impl ::BinTrait for Popover {}

impl_widget_events!(Popover);

