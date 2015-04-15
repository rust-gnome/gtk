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

//! Create widgets with a discrete toggle button

use glib::translate::ToGlibPtr;
use ffi;

/// CheckButton — Create widgets with a discrete toggle button
struct_Widget!(CheckButton);

pub trait CheckButtonBuilder {
    fn check_button(&self) -> Option<CheckButton>;
    fn check_button_with_label(&self, label: &str) -> Option<CheckButton>;
    fn check_button_with_mnemonic(&self, mnemonic: &str) -> Option<CheckButton>;
}

impl CheckButtonBuilder for ::Gtk {
    fn check_button(&self) -> Option<CheckButton> {
        let tmp_pointer = unsafe { ffi::gtk_check_button_new() };
        check_pointer!(tmp_pointer, CheckButton)
    }

    fn check_button_with_label(&self, label: &str) -> Option<CheckButton> {
        let tmp_pointer = unsafe {
            ffi::gtk_check_button_new_with_label(label.borrow_to_glib().0)
        };
        check_pointer!(tmp_pointer, CheckButton)
    }

    fn check_button_with_mnemonic(&self, mnemonic: &str) -> Option<CheckButton> {
        let tmp_pointer = unsafe {
            ffi::gtk_check_button_new_with_mnemonic(mnemonic.borrow_to_glib().0)
        };
        check_pointer!(tmp_pointer, CheckButton)
    }
}

impl_drop!(CheckButton);
impl_TraitWidget!(CheckButton);

impl ::ContainerTrait for CheckButton {}
impl ::ButtonTrait for CheckButton {}
impl ::ToggleButtonTrait for CheckButton {}

impl_widget_events!(CheckButton);

