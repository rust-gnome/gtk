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

use ffi;

/// GtkTextTagTable — Collection of tags that can be used together

pub struct TextTagTable {
    pointer: *mut ffi::C_GtkTextTagTable
}

pub trait TextTagTableBuilder {
    fn text_tag_table(&self) -> Option<TextTagTable>;
}

impl TextTagTableBuilder for ::Gtk {
    fn text_tag_table(&self) -> Option<TextTagTable> {
        match unsafe { ffi::gtk_text_tag_table_new() } {
            pointer if !pointer.is_null() => Some(TextTagTable { pointer: pointer }),
            _ => None
        }
    }
}

impl TextTagTable {
    pub fn unwrap_pointer(&self) -> *mut ffi::C_GtkTextTagTable {
        self.pointer
    }
}

impl_drop!(TextTagTable, GTK_TEXT_TAG_TABLE);

