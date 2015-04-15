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

//! GtkEntryCompletion — Completion functionality for GtkEntry

use ffi;
use TreeModel;
use cast::GTK_ENTRY_COMPLETION;
use glib::translate::{FromGlibPtr, ToGlibPtr};

struct_Widget!(EntryCompletion);

pub trait EntryCompletionBuilder {
    fn entry_completion(&self) -> Option<EntryCompletion>;
}

impl EntryCompletionBuilder for ::Gtk {
    fn entry_completion(&self) -> Option<EntryCompletion> {
        match unsafe { ffi::gtk_entry_completion_new() } {
            pointer if !pointer.is_null() => Some(EntryCompletion {
                pointer: pointer as *mut ffi::C_GtkWidget }),
            _ => None
        }
    }
}

impl EntryCompletion {
    pub fn get_entry<T: ::WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = unsafe { ffi::gtk_entry_completion_get_entry(GTK_ENTRY_COMPLETION(self.pointer)) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn set_model(&self, model: &TreeModel) {
        unsafe { ffi::gtk_entry_completion_set_model(GTK_ENTRY_COMPLETION(self.pointer), model.unwrap_pointer()) }
    }

    pub fn get_model(&self) -> Option<TreeModel> {
        let tmp_pointer = unsafe { ffi::gtk_entry_completion_get_model(GTK_ENTRY_COMPLETION(self.pointer)) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(TreeModel::wrap_pointer(tmp_pointer))
        }
    }

    pub fn set_minimum_key_length(&self, length: i32) {
        unsafe { ffi::gtk_entry_completion_set_minimum_key_length(GTK_ENTRY_COMPLETION(self.pointer), length) }
    }

    pub fn get_minimum_key_length(&self) -> i32 {
        unsafe { ffi::gtk_entry_completion_get_minimum_key_length(GTK_ENTRY_COMPLETION(self.pointer)) }
    }

    pub fn compute_prefix(&self, key: &str) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_entry_completion_compute_prefix(
                    GTK_ENTRY_COMPLETION(self.pointer),
                    key.borrow_to_glib().0))
        }
    }

    pub fn complete(&self) {
        unsafe { ffi::gtk_entry_completion_complete(GTK_ENTRY_COMPLETION(self.pointer)) }
    }

    pub fn get_completion_prefix(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_entry_completion_get_completion_prefix(GTK_ENTRY_COMPLETION(self.pointer)))
        }
    }

    pub fn insert_prefix(&self) {
        unsafe { ffi::gtk_entry_completion_insert_prefix(GTK_ENTRY_COMPLETION(self.pointer)) }
    }

    pub fn insert_action_text(&self, index_: i32, text: &str) {
        unsafe {
            ffi::gtk_entry_completion_insert_action_text(GTK_ENTRY_COMPLETION(self.pointer), index_, text.borrow_to_glib().0)
        }
    }

    pub fn insert_action_markup(&self, index_: i32, markup: &str) {
        unsafe {
            ffi::gtk_entry_completion_insert_action_markup(GTK_ENTRY_COMPLETION(self.pointer), index_, markup.borrow_to_glib().0)
        }
    }

    pub fn delete_action(&self, index_: i32) {
        unsafe { ffi::gtk_entry_completion_delete_action(GTK_ENTRY_COMPLETION(self.pointer), index_) }
    }

    pub fn set_text_column(&self, column: i32) {
        unsafe { ffi::gtk_entry_completion_set_text_column(GTK_ENTRY_COMPLETION(self.pointer), column) }
    }

    pub fn get_text_column(&self) -> i32 {
        unsafe { ffi::gtk_entry_completion_get_text_column(GTK_ENTRY_COMPLETION(self.pointer)) }
    }

    pub fn set_inline_completion(&self, inline_completion: bool) {
        unsafe { ffi::gtk_entry_completion_set_inline_completion(GTK_ENTRY_COMPLETION(self.pointer), match inline_completion {
            true => 1,
            false => 0
        }) }
    }

    pub fn get_inline_completion(&self) -> bool {
        match unsafe { ffi::gtk_entry_completion_get_inline_completion(GTK_ENTRY_COMPLETION(self.pointer)) } {
            0 => false,
            _ => true
        }
    }

    pub fn set_inline_selection(&self, inline_completion: bool) {
        unsafe { ffi::gtk_entry_completion_set_inline_selection(GTK_ENTRY_COMPLETION(self.pointer), match inline_completion {
            true => 1,
            false => 0
        }) }
    }

    pub fn get_inline_selection(&self) -> bool {
        match unsafe { ffi::gtk_entry_completion_get_inline_selection(GTK_ENTRY_COMPLETION(self.pointer)) } {
            0 => false,
            _ => true
        }
    }

    pub fn set_popup_completion(&self, inline_completion: bool) {
        unsafe { ffi::gtk_entry_completion_set_inline_selection(GTK_ENTRY_COMPLETION(self.pointer), match inline_completion {
            true => 1,
            false => 0
        }) }
    }

    pub fn get_popup_completion(&self) -> bool {
        match unsafe { ffi::gtk_entry_completion_get_inline_selection(GTK_ENTRY_COMPLETION(self.pointer)) } {
            0 => false,
            _ => true
        }
    }

    pub fn set_popup_set_width(&self, inline_completion: bool) {
        unsafe { ffi::gtk_entry_completion_set_popup_set_width(GTK_ENTRY_COMPLETION(self.pointer), match inline_completion {
            true => 1,
            false => 0
        }) }
    }

    pub fn get_popup_set_width(&self) -> bool {
        match unsafe { ffi::gtk_entry_completion_get_popup_set_width(GTK_ENTRY_COMPLETION(self.pointer)) } {
            0 => false,
            _ => true
        }
    }

    pub fn set_popup_single_match(&self, inline_completion: bool) {
        unsafe { ffi::gtk_entry_completion_set_popup_single_match(GTK_ENTRY_COMPLETION(self.pointer),
                                                                  match inline_completion {
                                                                      true => 1,
                                                                      false => 0
                                                                  })
        }
    }

    pub fn get_popup_single_match(&self) -> bool {
        match unsafe { ffi::gtk_entry_completion_get_popup_single_match(GTK_ENTRY_COMPLETION(self.pointer)) } {
            0 => false,
            _ => true
        }
    }
}

impl_drop!(EntryCompletion);
impl_TraitWidget!(EntryCompletion);

impl ::CellLayoutTrait for EntryCompletion {}

impl_widget_events!(EntryCompletion);

