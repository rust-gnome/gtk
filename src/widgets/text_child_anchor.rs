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
use glib::to_bool;

pub struct TextChildAnchor {
    pointer: *mut ffi::C_GtkTextChildAnchor
}

pub trait TextChildAnchorBuilder {
    fn text_child_anchor(&self) -> Option<TextChildAnchor>;
}

impl TextChildAnchorBuilder for ::Gtk {
    fn text_child_anchor(&self) -> Option<TextChildAnchor> {
        match unsafe { ffi::gtk_text_child_anchor_new() } {
            pointer if !pointer.is_null() => Some(TextChildAnchor { pointer: pointer }),
            _ => None
        }
    }
}

impl TextChildAnchor {
    pub fn get_deleted(&self) -> bool {
        unsafe { to_bool(ffi::gtk_text_child_anchor_get_deleted(self.pointer)) }
    }
}

impl_GObjectFunctions!(TextChildAnchor, C_GtkTextChildAnchor);
impl_TraitObject!(TextChildAnchor, C_GtkTextChildAnchor);

