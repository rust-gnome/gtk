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

//! A button to launch a font chooser dialog

use glib::translate::{FromGlibPtr, ToGlibPtr};
use ffi;
use glib::{to_bool, to_gboolean};
use cast::GTK_FONTBUTTON;

/// FontButton — A button to launch a font chooser dialog
/*
* # Availables signals :
* * `font-set` : Run First
*/
struct_Widget!(FontButton);

pub trait FontButtonBuilder {
    fn font_button(&self) -> Option<FontButton>;
    fn font_button_with_font(&self, font_name: &str) -> Option<FontButton>;
}

impl FontButtonBuilder for ::Gtk {
    fn font_button(&self) -> Option<FontButton> {
        let tmp_pointer = unsafe { ffi::gtk_font_button_new() };
        check_pointer!(tmp_pointer, FontButton)
    }

    fn font_button_with_font(&self, font_name: &str) -> Option<FontButton> {
        let tmp_pointer = unsafe {
            ffi::gtk_font_button_new_with_font(font_name.borrow_to_glib().0)
        };
        check_pointer!(tmp_pointer, FontButton)
    }
}

impl FontButton {
    pub fn set_font_name(&mut self, font_name: &str) -> bool {
        unsafe { to_bool(ffi::gtk_font_button_set_font_name(GTK_FONTBUTTON(self.pointer), font_name.borrow_to_glib().0)) }
    }

    pub fn get_font_name(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_font_button_get_font_name(GTK_FONTBUTTON(self.pointer)))
        }
    }

    pub fn set_show_style(&mut self, show_style: bool) -> () {
        unsafe { ffi::gtk_font_button_set_show_style(GTK_FONTBUTTON(self.pointer), to_gboolean(show_style)); }
    }

    pub fn get_show_style(&self) -> bool {
        unsafe { to_bool(ffi::gtk_font_button_get_show_style(GTK_FONTBUTTON(self.pointer))) }
    }

    pub fn set_show_size(&mut self, show_size: bool) -> () {
        unsafe { ffi::gtk_font_button_set_show_size(GTK_FONTBUTTON(self.pointer), to_gboolean(show_size)); }
    }

    pub fn get_show_size(&self) -> bool {
        unsafe { to_bool(ffi::gtk_font_button_get_show_size(GTK_FONTBUTTON(self.pointer))) }
    }

    pub fn set_use_font(&mut self, use_font: bool) -> () {
        unsafe { ffi::gtk_font_button_set_use_font(GTK_FONTBUTTON(self.pointer), to_gboolean(use_font)); }
    }

    pub fn get_use_font(&self) -> bool {
        unsafe { to_bool(ffi::gtk_font_button_get_use_font(GTK_FONTBUTTON(self.pointer))) }
    }

    pub fn set_use_size(&mut self, use_size: bool) -> () {
        unsafe { ffi::gtk_font_button_set_use_size(GTK_FONTBUTTON(self.pointer), to_gboolean(use_size)); }
    }

    pub fn get_use_size(&self) -> bool {
        unsafe { to_bool(ffi::gtk_font_button_get_use_size(GTK_FONTBUTTON(self.pointer))) }
    }

    pub fn set_title(&mut self, title: &str) -> () {
        unsafe {
            ffi::gtk_font_button_set_title(GTK_FONTBUTTON(self.pointer), title.borrow_to_glib().0);
        }
    }

    pub fn get_title(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_font_button_get_title(GTK_FONTBUTTON(self.pointer)))
        }
    }
}

impl_drop!(FontButton);
impl_TraitWidget!(FontButton);

impl ::ContainerTrait for FontButton {}
impl ::ButtonTrait for FontButton {}

impl_widget_events!(FontButton);
impl_button_events!(FontButton);

