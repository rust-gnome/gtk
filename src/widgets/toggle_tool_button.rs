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

//! A ToolItem containing a toggle button

use ffi;
use glib::translate::ToGlibPtr;

/// ToggleToolButton — A ToolItem containing a toggle button
struct_Widget!(ToggleToolButton);

pub trait ToggleToolButtonBuilder {
    fn toggle_tool_button(&self) -> Option<ToggleToolButton>;
    fn toggle_tool_button_from_stock(&self, stock_id: &str) -> Option<ToggleToolButton>;
}

impl ToggleToolButtonBuilder for ::Gtk {
    fn toggle_tool_button(&self) -> Option<ToggleToolButton> {
        let tmp_pointer = unsafe { ffi::gtk_toggle_tool_button_new() };
        check_pointer!(tmp_pointer, ToggleToolButton)
    }

    fn toggle_tool_button_from_stock(&self, stock_id: &str) -> Option<ToggleToolButton> {
        let tmp_pointer = unsafe { ffi::gtk_toggle_tool_button_new_from_stock(
            stock_id.borrow_to_glib().0) };
        check_pointer!(tmp_pointer, ToggleToolButton)
    }
}

impl_drop!(ToggleToolButton);
impl_TraitWidget!(ToggleToolButton);

impl ::ContainerTrait for ToggleToolButton {}
impl ::BinTrait for ToggleToolButton {}
impl ::ToolItemTrait for ToggleToolButton {}
impl ::ToolButtonTrait for ToggleToolButton {}
impl ::ToggleToolButtonTrait for ToggleToolButton {}

impl_widget_events!(ToggleToolButton);

