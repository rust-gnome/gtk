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

//! GtkTextTag — A tag that can be applied to text in a GtkTextBuffer

use ffi;
use glib::translate::ToGlibPtr;

pub struct TextTag {
    pointer: *mut ffi::C_GtkTextTag
}

pub trait TextTagBuilder {
    fn text_tag(&self, name: &str) -> Option<TextTag>;
}

impl TextTagBuilder for ::Gtk {
    fn text_tag(&self, name: &str) -> Option<TextTag> {
        match unsafe { ffi::gtk_text_tag_new(name.borrow_to_glib().0) } {
            pointer if !pointer.is_null() => Some(TextTag { pointer: pointer }),
            _ => None
        }
    }
}

impl TextTag {
    pub fn get_priority(&self) -> i32 {
        unsafe { ffi::gtk_text_tag_get_priority(self.pointer) }
    }

    pub fn set_priority(&self, priority: i32) {
        unsafe { ffi::gtk_text_tag_set_priority(self.pointer, priority as ::libc::c_int) }
    }
}

impl_GObjectFunctions!(TextTag, C_GtkTextTag);
impl_TraitObject!(TextTag, C_GtkTextTag);

