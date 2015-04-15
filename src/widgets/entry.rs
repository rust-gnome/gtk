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

//! A single line text entry field

use ffi;

/// Entry — A single line text entry field
/*
* # Availables signals :
* * `activate` : Action
* * `backspace` : Action
* * `copy-clipboard` : Action
* * `cut-clipboard` : Action
* * `delete-from-cursor` : Action
* * `icon-press` : Run Last
* * `icon-release` : Run Last
* * `insert-at-cursor` : Action
* * `move-cursor` : Action
* * `paste-clipboard` : Action
* * `populate-popup` : Run Last
* * `preedit-changed` : Action
* * `toggle-overwrite` : Action
*/
struct_Widget!(Entry);

pub trait EntryBuilder {
    fn entry(&self) -> Option<Entry>;
    fn entry_with_buffer(&self, buffer: &::EntryBuffer) -> Option<Entry>;
}

impl EntryBuilder for ::Gtk {
    fn entry(&self) -> Option<Entry> {
        let tmp_pointer = unsafe { ffi::gtk_entry_new() };
        check_pointer!(tmp_pointer, Entry)
    }

    fn entry_with_buffer(&self, buffer: &::EntryBuffer) -> Option<Entry> {
        let tmp_pointer = unsafe { ffi::gtk_entry_new_with_buffer(buffer.unwrap_pointer()) };
        check_pointer!(tmp_pointer, Entry)
    }
}

impl_drop!(Entry);
impl_TraitWidget!(Entry);

impl ::EntryTrait for Entry {}
impl ::EditableTrait for Entry {}

impl_widget_events!(Entry);

