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

//! A widget that displays a small to medium amount of text

use glib::translate::ToGlibPtr;
use ffi;

/// Label — A widget that displays a small to medium amount of text
/*
* # Available signals:
* * `activate-current-link` : Action
* * `activate-link` : Run Last
* * `copy-clipboard` : Action
* * `move-cursor` : Action
* * `populate-popup` : Run Last
*/
struct_Widget!(Label);

pub trait LabelBuilder {
    fn label(&self, text: &str) -> Option<Label>;
    fn label_with_mnemonic(&self, text: &str) -> Option<Label>;
}

impl LabelBuilder for ::Gtk {
    fn label(&self, text: &str) -> Option<Label> {
        let tmp_pointer = unsafe {
            ffi::gtk_label_new(text.borrow_to_glib().0)
        };
        check_pointer!(tmp_pointer, Label)
    }

    fn label_with_mnemonic(&self, text: &str) -> Option<Label> {
        let tmp_pointer = unsafe {
            ffi::gtk_label_new_with_mnemonic(text.borrow_to_glib().0)
        };
        check_pointer!(tmp_pointer, Label)
    }
}

impl_drop!(Label);
impl_TraitWidget!(Label);

impl ::MiscTrait for Label {}
impl ::LabelTrait for Label {}

impl_widget_events!(Label);

