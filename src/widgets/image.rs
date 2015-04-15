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

//! A widget displaying an image

use ffi;
use cast::GTK_IMAGE;
use FFIWidget;
use glib::translate::ToGlibPtr;
use gdk;

/// Image — A widget displaying an image
struct_Widget!(Image);

pub trait ImageBuilder {
    fn image(&self) -> Option<Image>;
    fn image_from_file(&self, filename: &str) -> Option<Image>;
    fn image_from_pixbuf(&self, pixbuf: &gdk::Pixbuf) -> Option<Image>;
    fn image_from_icon_name(&self, icon_name: &str, size: ::IconSize) -> Option<Image>;
}

impl ImageBuilder for ::Gtk {
    fn image(&self) -> Option<Image> {
        let tmp_pointer = unsafe {
            ffi::gtk_image_new()
        };
        check_pointer!(tmp_pointer, Image)
    }

    fn image_from_file(&self, filename: &str) -> Option<Image> {
        let tmp_pointer = unsafe {
            ffi::gtk_image_new_from_file(filename.borrow_to_glib().0)
        };
        check_pointer!(tmp_pointer, Image)
    }

    fn image_from_pixbuf(&self, pixbuf: &gdk::Pixbuf) -> Option<Image> {
        let tmp_pointer = unsafe {
            ffi::gtk_image_new_from_pixbuf(pixbuf.unwrap_pointer())
        };
        check_pointer!(tmp_pointer, Image)
    }

    fn image_from_icon_name(&self, icon_name: &str, size: ::IconSize) -> Option<Image> {
        let tmp_pointer = unsafe {
            ffi::gtk_image_new_from_icon_name(icon_name.borrow_to_glib().0, size)
        };
        check_pointer!(tmp_pointer, Image)
    }
}

impl Image {
    pub fn set_from_file(&self, filename: &str) {
        unsafe {
            ffi::gtk_image_set_from_file(GTK_IMAGE(self.unwrap_widget()),
                                         filename.borrow_to_glib().0);
        };
    }

    pub fn set_from_pixbuf(&self, pixbuf: &gdk::Pixbuf) {
        unsafe {
            ffi::gtk_image_set_from_pixbuf(GTK_IMAGE(self.unwrap_widget()), pixbuf.unwrap_pointer())
        };
    }

    pub fn set_from_icon_name(&self, icon_name: &str, size: ::IconSize) {
        unsafe {
            ffi::gtk_image_set_from_icon_name(GTK_IMAGE(self.unwrap_widget()),
                                              icon_name.borrow_to_glib().0, size)
        };
    }

    pub fn get_pixbuf(&self) -> gdk::Pixbuf {
        gdk::Pixbuf::wrap_pointer(unsafe {
            ffi::gtk_image_get_pixbuf(GTK_IMAGE(self.unwrap_widget()))
        })
    }
}

impl_drop!(Image);
impl_TraitWidget!(Image);

impl ::MiscTrait for Image {}

impl_widget_events!(Image);

