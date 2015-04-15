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

//! GtkComboBox — A widget used to choose from a list of items

use ffi;

struct_Widget!(ComboBox);

pub trait ComboBoxBuilder {
    fn combo_box() -> Option<ComboBox>;
    fn combo_box_with_entry() -> Option<ComboBox>;
    fn combo_box_with_model(model: &::TreeModel) -> Option<ComboBox>;
    fn combo_box_with_model_and_entry(model: &::TreeModel) -> Option<ComboBox>;
    /*fn combo_box_with_area(area: &::CellArea) -> Option<ComboBox>;
    fn combo_box_with_area_and_entry(area: &::CellArea) -> Option<ComboBox>;*/
}

impl ComboBoxBuilder for ::Gtk {
    fn combo_box() -> Option<ComboBox> {
        let tmp_pointer = unsafe { ffi::gtk_combo_box_new() };
        check_pointer!(tmp_pointer, ComboBox)
    }

    fn combo_box_with_entry() -> Option<ComboBox> {
        let tmp_pointer = unsafe { ffi::gtk_combo_box_new_with_entry() };
        check_pointer!(tmp_pointer, ComboBox)
    }

    fn combo_box_with_model(model: &::TreeModel) -> Option<ComboBox> {
        let tmp_pointer = unsafe { ffi::gtk_combo_box_new_with_model(model.unwrap_pointer()) };
        check_pointer!(tmp_pointer, ComboBox)
    }

    fn combo_box_with_model_and_entry(model: &::TreeModel) -> Option<ComboBox> {
        let tmp_pointer = unsafe { ffi::gtk_combo_box_new_with_model_and_entry(model.unwrap_pointer()) };
        check_pointer!(tmp_pointer, ComboBox)
    }

    /*fn combo_box_with_area(area: &::CellArea) -> Option<ComboBox> {
        let tmp_pointer = unsafe { ffi::gtk_combo_box_new_with_area(area.unwrap_pointer()) };
        check_pointer!(tmp_pointer, ComboBox)
    }

    fn combo_box_with_area_and_entry(area: &::CellArea) -> Option<ComboBox> {
        let tmp_pointer = unsafe { ffi::gtk_combo_box_new_with_area_and_entry(area.unwrap_pointer()) };
        check_pointer!(tmp_pointer, ComboBox)
    }*/
}

impl_drop!(ComboBox);
impl_TraitWidget!(ComboBox);

impl ::ContainerTrait for ComboBox {}
impl ::BinTrait for ComboBox {}
impl ::ComboBoxTrait for ComboBox {}

impl_widget_events!(ComboBox);

