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

//! The widget used for item in menus

use ffi;
use glib::translate::ToGlibPtr;

/// MenuItem — The widget used for item in menus
struct_Widget!(MenuItem);

pub trait MenuItemBuilder {
    fn menu_item(&self) -> Option<MenuItem>;
    fn menu_item_with_label(&self, label: &str) -> Option<MenuItem>;
    fn menu_item_with_mnemonic(&self, mnemonic: &str) -> Option<MenuItem>;
}

impl MenuItemBuilder for ::Gtk {
    fn menu_item(&self) -> Option<MenuItem> {
        let tmp_pointer = unsafe { ffi::gtk_menu_item_new() };
        check_pointer!(tmp_pointer, MenuItem)
    }

    fn menu_item_with_label(&self, label: &str) -> Option<MenuItem> {
        let tmp_pointer = unsafe {
            ffi::gtk_menu_item_new_with_label(label.borrow_to_glib().0)
        };
        check_pointer!(tmp_pointer, MenuItem)
    }

    fn menu_item_with_mnemonic(&self, mnemonic: &str) -> Option<MenuItem> {
        let tmp_pointer = unsafe {
            ffi::gtk_menu_item_new_with_mnemonic(mnemonic.borrow_to_glib().0)
        };
        check_pointer!(tmp_pointer, MenuItem)
    }
}

impl_drop!(MenuItem);
impl_TraitWidget!(MenuItem);

impl ::ContainerTrait for MenuItem {}
impl ::BinTrait for MenuItem {}
impl ::MenuItemTrait for MenuItem {}

impl_widget_events!(MenuItem);

