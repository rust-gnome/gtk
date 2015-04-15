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
use FFIWidget;
use cast::{GTK_WINDOW, GTK_PAGE_SETUP_UNIX_DIALOG, GTK_PAGE_SETUP, GTK_PRINT_SETTINGS};
use std::str;

struct_Widget!(PageSetupUnixDialog);

pub trait PageSetupUnixDialogBuilder {
    fn page_setup_unix_dialog(&self, title: &str, parent: Option<::Window>) ->
        Option<PageSetupUnixDialog>;
}

impl PageSetupUnixDialogBuilder for ::Gtk {
    fn page_setup_unix_dialog(&self, title: &str, parent: Option<::Window>) ->
            Option<PageSetupUnixDialog> {
        let tmp_pointer = unsafe {
            title.with_c_str(|c_str|{
                ffi::gtk_page_setup_unix_dialog_new(match parent {
                    Some(ref p) => GTK_WINDOW(p.unwrap_widget()),
                    None => ::std::ptr::null_mut()
                })
            })
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }
}

impl PageSetupUnixDialog {
    pub fn set_page_setup(&self, page_setup: &::PageSetup) {
        unsafe { ffi::gtk_page_setup_unix_dialog_set_page_setup(GTK_PAGE_SETUP_UNIX_DIALOG(self.unwrap_widget()), GTK_PAGE_SETUP(page_setup.unwrap_widget())) }
    }

    pub fn get_page_setup(&self) -> Option<PageSetup> {
        let tmp = unsafe { ffi::gtk_page_setup_unix_dialog_get_page_setup(GTK_PAGE_SETUP_UNIX_DIALOG(self.unwrap_widget())) };

        if tmp.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn set_print_settings(&self, print_settings: &::PrintSettings) {
        unsafe { ffi::gtk_page_setup_unix_dialog_set_print_settings(GTK_PAGE_SETUP_UNIX_DIALOG(self.unwrap_widget()), GTK_PRINT_SETTINGS(print_settings.unwrap_widget())) }
    }

    pub fn get_print_settings(&self) -> Option<PrintSettings> {
        let tmp = unsafe { ffi::gtk_page_setup_unix_dialog_get_print_settings(GTK_PAGE_SETUP_UNIX_DIALOG(self.unwrap_widget())) };

        if tmp.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }
}

impl_drop!(PageSetupUnixDialog);
impl_TraitWidget!(PageSetupUnixDialog);

impl ::ContainerTrait for PageSetupUnixDialog {}
impl ::BinTrait for PageSetupUnixDialog {}
impl ::WindowTrait for PageSetupUnixDialog {}
impl ::DialogTrait for PageSetupUnixDialog {}

impl_widget_events!(PageSetupUnixDialog);

