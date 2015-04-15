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

//! A tabbed notebook container

use ffi;
use cast::GTK_NOTEBOOK;
use FFIWidget;
use glib::translate::{FromGlibPtr, ToGlibPtr};
use glib::{to_bool, to_gboolean};

/// GtkNotebook — A tabbed notebook container
struct_Widget!(NoteBook);

pub trait NoteBookBuilder {
    fn notebook(&self) -> Option<NoteBook>;
}

impl NoteBookBuilder for ::Gtk {
    fn notebook(&self) -> Option<NoteBook> {
        let tmp_pointer = unsafe { ffi::gtk_notebook_new() };
        check_pointer!(tmp_pointer, NoteBook)
    }
}

impl NoteBook {
    pub fn append_page<Child: ::WidgetTrait, TabLabel: ::WidgetTrait>(&mut self, child: &Child,
            tab_label: Option<&TabLabel>) -> Option<u32> {
        match unsafe { ffi::gtk_notebook_append_page(GTK_NOTEBOOK(self.pointer),
                child.unwrap_widget(), unwrap_widget!(tab_label)) } {
            x if x >= 0 => Some(x as u32),
            _ => None
        }
    }

    pub fn append_page_menu<Child: ::WidgetTrait, TabLabel: ::WidgetTrait,
            MenuLabel: ::WidgetTrait>(&mut self, child: &Child, tab_label: Option<&TabLabel>,
            menu_label: Option<&MenuLabel>) -> Option<u32> {
        match unsafe { ffi::gtk_notebook_append_page_menu(GTK_NOTEBOOK(self.pointer),
                child.unwrap_widget(), unwrap_widget!(tab_label), unwrap_widget!(menu_label)) } {
            x if x >= 0 => Some(x as u32),
            _ => None
        }
    }

    pub fn prepend_page<Child: ::WidgetTrait, TabLabel: ::WidgetTrait>(&mut self, child: &Child,
            tab_label: Option<&TabLabel>) -> Option<u32> {
        match unsafe { ffi::gtk_notebook_prepend_page(GTK_NOTEBOOK(self.pointer),
                child.unwrap_widget(), unwrap_widget!(tab_label)) } {
            x if x >= 0 => Some(x as u32),
            _ => None
        }
    }

    pub fn prepend_page_menu<Child: ::WidgetTrait, TabLabel: ::WidgetTrait,
            MenuLabel: ::WidgetTrait>(&mut self, child: &Child, tab_label: Option<&TabLabel>,
            menu_label: Option<&MenuLabel>) -> Option<u32> {
        match unsafe { ffi::gtk_notebook_prepend_page_menu(GTK_NOTEBOOK(self.pointer),
                child.unwrap_widget(), unwrap_widget!(tab_label), unwrap_widget!(menu_label)) } {
            x if x >= 0 => Some(x as u32),
            _ => None
        }
    }

    pub fn insert_page<Child: ::WidgetTrait, TabLabel: ::WidgetTrait>(&mut self, child: &Child,
            tab_label: Option<&TabLabel>, position: i32) -> Option<u32> {
        match unsafe { ffi::gtk_notebook_insert_page(GTK_NOTEBOOK(self.pointer),
                child.unwrap_widget(), unwrap_widget!(tab_label), position) } {
            x if x >= 0 => Some(x as u32),
            _ => None
        }
    }

    pub fn insert_page_menu<Child: ::WidgetTrait, TabLabel: ::WidgetTrait,
            MenuLabel: ::WidgetTrait>(&mut self, child: &Child, tab_label: Option<&TabLabel>,
            menu_label: Option<&MenuLabel>, position: i32) -> Option<u32> {
        match unsafe { ffi::gtk_notebook_insert_page_menu(GTK_NOTEBOOK(self.pointer),
                child.unwrap_widget(), unwrap_widget!(tab_label), unwrap_widget!(menu_label),
                position) } {
            x if x >= 0 => Some(x as u32),
            _ => None
        }
    }

    pub fn remove_page(&mut self, page_num: i32) {
        unsafe {
            ffi::gtk_notebook_remove_page(GTK_NOTEBOOK(self.pointer), page_num)
        }
    }

    pub fn set_group_name(&mut self, group_name: &str) {
        unsafe {
            ffi::gtk_notebook_set_group_name(GTK_NOTEBOOK(self.pointer),
                                             group_name.borrow_to_glib().0)
        }
    }

    pub fn get_group_name(&mut self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_notebook_get_group_name(GTK_NOTEBOOK(self.pointer)))
        }
    }

    pub fn get_current_page(&self) -> i32 {
        unsafe {
            ffi::gtk_notebook_get_current_page(GTK_NOTEBOOK(self.pointer))
        }
    }

    pub fn get_nth_page<T: ::WidgetTrait>(&self, page_num: i32) -> Option<T> {
        let tmp_pointer = unsafe { ffi::gtk_notebook_get_nth_page(GTK_NOTEBOOK(self.pointer), page_num) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn get_n_page(&self) -> i32 {
        unsafe {
            ffi::gtk_notebook_get_n_pages(GTK_NOTEBOOK(self.pointer))
        }
    }

    pub fn page_num<T: ::WidgetTrait>(&self, child: &T) -> i32 {
        unsafe {
            ffi::gtk_notebook_page_num(GTK_NOTEBOOK(self.pointer), child.unwrap_widget())
        }
    }

    pub fn set_current_page(&mut self, page_num: i32) {
        unsafe {
            ffi::gtk_notebook_set_current_page(GTK_NOTEBOOK(self.pointer), page_num)
        }
    }

    pub fn next_page(&mut self) {
        unsafe {
            ffi::gtk_notebook_next_page(GTK_NOTEBOOK(self.pointer))
        }
    }

    pub fn prev_page(&mut self) {
        unsafe {
            ffi::gtk_notebook_prev_page(GTK_NOTEBOOK(self.pointer))
        }
    }

    pub fn set_show_border(&mut self, show_border: bool) {
        unsafe {
            ffi::gtk_notebook_set_show_border(GTK_NOTEBOOK(self.pointer),
                                              to_gboolean(show_border))
        }
    }

    pub fn get_show_border(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_notebook_get_show_border(GTK_NOTEBOOK(self.pointer)))
        }
    }

    pub fn set_show_tabs(&mut self, show_tabs: bool) {
        unsafe {
            ffi::gtk_notebook_set_show_tabs(GTK_NOTEBOOK(self.pointer),
                                            to_gboolean(show_tabs))
        }
    }

    pub fn get_show_tabs(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_notebook_get_show_tabs(GTK_NOTEBOOK(self.pointer)))
        }
    }

    pub fn set_tab_pos(&mut self, pos: ::PositionType) {
        unsafe {
            ffi::gtk_notebook_set_tab_pos(GTK_NOTEBOOK(self.pointer), pos)
        }
    }

    pub fn get_tab_pos(&self) -> ::PositionType {
        unsafe {
            ffi::gtk_notebook_get_tab_pos(GTK_NOTEBOOK(self.pointer))
        }
    }

    pub fn set_scrollable(&mut self, scrollable: bool) {
        unsafe {
            ffi::gtk_notebook_set_scrollable(GTK_NOTEBOOK(self.pointer),
                                             to_gboolean(scrollable))
        }
    }

    pub fn is_scrollable(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_notebook_get_scrollable(GTK_NOTEBOOK(self.pointer)))
        }
    }

    pub fn get_tab_hborder(&self) -> u16 {
        unsafe {
            ffi::gtk_notebook_get_tab_hborder(GTK_NOTEBOOK(self.pointer))
        }
    }

    pub fn get_tab_vborder(&self) -> u16 {
        unsafe {
            ffi::gtk_notebook_get_tab_vborder(GTK_NOTEBOOK(self.pointer))
        }
    }

    pub fn popup_enable(&mut self) {
        unsafe {
            ffi::gtk_notebook_popup_enable(GTK_NOTEBOOK(self.pointer))
        }
    }

    pub fn popup_disable(&mut self) {
        unsafe {
            ffi::gtk_notebook_popup_disable(GTK_NOTEBOOK(self.pointer))
        }
    }

    pub fn get_tab_label<Child: ::WidgetTrait, TabLabel: ::WidgetTrait>(&self, child: &Child) ->
            Option<TabLabel> {
        let tmp_pointer = unsafe {
            ffi::gtk_notebook_get_tab_label(GTK_NOTEBOOK(self.pointer),
                                            child.unwrap_widget())
        };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn set_tab_label<Child: ::WidgetTrait, TabLabel: ::WidgetTrait>(&self, child: &Child,
            tab_label: Option<&TabLabel>) {
        unsafe {
            ffi::gtk_notebook_set_tab_label(GTK_NOTEBOOK(self.pointer),
                                            child.unwrap_widget(),
                                            unwrap_widget!(tab_label))
        }
    }

    pub fn set_tab_label_text<T: ::WidgetTrait>(&mut self, child: &T, tab_text: &str) {
        unsafe {
            ffi::gtk_notebook_set_tab_label_text(GTK_NOTEBOOK(self.pointer),
                                                 child.unwrap_widget(),
                                                 tab_text.borrow_to_glib().0)
        }
    }

    pub fn get_tab_label_text<T: ::WidgetTrait>(&mut self, child: &T) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_notebook_get_tab_label_text(GTK_NOTEBOOK(self.pointer),
                                                     child.unwrap_widget()))
        }
    }

    pub fn get_menu_label<Child: ::WidgetTrait, MenuLabel: ::WidgetTrait>(&self, child: &Child) ->
            Option<MenuLabel> {
        let tmp_pointer = unsafe {
            ffi::gtk_notebook_get_menu_label(GTK_NOTEBOOK(self.pointer),
                                             child.unwrap_widget())
        };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn set_menu_label<Child: ::WidgetTrait, MenuLabel: ::WidgetTrait>(&self, child: &Child,
        menu_label: Option<&MenuLabel>) {
        unsafe {
            ffi::gtk_notebook_set_menu_label(GTK_NOTEBOOK(self.pointer),
                                             child.unwrap_widget(),
                                             unwrap_widget!(menu_label))
        }
    }

    pub fn set_menu_label_text<T: ::WidgetTrait>(&mut self, child: &T, tab_text: &str) {
        unsafe {
            ffi::gtk_notebook_set_menu_label_text(GTK_NOTEBOOK(self.pointer),
                                                  child.unwrap_widget(),
                                                  tab_text.borrow_to_glib().0)
        }
    }

    pub fn get_menu_label_text<T: ::WidgetTrait>(&mut self, child: &T) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_notebook_get_menu_label_text(GTK_NOTEBOOK(self.pointer),
                                                              child.unwrap_widget()))
        }
    }

    pub fn reorder_child<T: ::WidgetTrait>(&mut self, child: &T, position: i32) {
        unsafe {
            ffi::gtk_notebook_reorder_child(GTK_NOTEBOOK(self.pointer),
                                            child.unwrap_widget(),
                                            position)
        }
    }

    pub fn is_tab_reorderable<T: ::WidgetTrait>(&self, child: &T) -> bool {
        unsafe {
            to_bool(ffi::gtk_notebook_get_tab_reorderable(GTK_NOTEBOOK(self.pointer),
                                                               child.unwrap_widget()))
        }
    }

    pub fn set_tab_reorderable<T: ::WidgetTrait>(&mut self, child: &T, reorderable: bool) {
        unsafe {
            ffi::gtk_notebook_set_tab_reorderable(GTK_NOTEBOOK(self.pointer),
                                                 child.unwrap_widget(),
                                                 to_gboolean(reorderable))
        }
    }

    pub fn is_tab_detachable<T: ::WidgetTrait>(&self, child: &T) -> bool {
        unsafe {
            to_bool(ffi::gtk_notebook_get_tab_detachable(GTK_NOTEBOOK(self.pointer),
                                                              child.unwrap_widget()))
        }
    }

    pub fn set_tab_detachable<T: ::WidgetTrait>(&mut self, child: &T, detachable: bool) {
        unsafe {
            ffi::gtk_notebook_set_tab_detachable(GTK_NOTEBOOK(self.pointer),
                                                child.unwrap_widget(),
                                                to_gboolean(detachable))
        }
    }

    pub fn get_action_widget<T: ::WidgetTrait>(&self, pack_type: ::PackType) -> Option<T> {
        let tmp_pointer = unsafe { ffi::gtk_notebook_get_action_widget(GTK_NOTEBOOK(self.pointer),
                                                                       pack_type) };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn set_action_widget<T: ::WidgetTrait>(&mut self, child: &T, pack_type: ::PackType) {
        unsafe {
            ffi::gtk_notebook_set_action_widget(GTK_NOTEBOOK(self.pointer),
                                                child.unwrap_widget(),
                                                pack_type)
        }
    }
}

impl_drop!(NoteBook);
impl_TraitWidget!(NoteBook);

impl ::ContainerTrait for NoteBook {}

impl_widget_events!(NoteBook);

