// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use cast::GTK_MENU_BAR;
use traits::FFIWidget;
use ffi;

struct_Widget!(MenuBar);
impl_TraitWidget!(MenuBar);

impl MenuBar {
    pub fn new() -> Option<MenuBar> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_menu_bar_new() };
        check_pointer!(tmp_pointer, MenuBar)
    }
    
    pub fn set_pack_direction(&self, pack_dir: ::PackDirection) {
        unsafe {
            ffi::gtk_menu_bar_set_pack_direction(GTK_MENU_BAR(self.unwrap_widget()), pack_dir);
        }
    }
    
    pub fn get_pack_direction(&self) -> ::PackDirection {
        unsafe {
            ffi::gtk_menu_bar_get_pack_direction(GTK_MENU_BAR(self.unwrap_widget()))
        }
    }
    
    pub fn set_child_pack_direction(&self, pack_dir: ::PackDirection) {
        unsafe {
            ffi::gtk_menu_bar_set_child_pack_direction(GTK_MENU_BAR(self.unwrap_widget()), pack_dir);
        }
    }
    
    pub fn get_child_pack_direction(&self) -> ::PackDirection {
        unsafe {
            ffi::gtk_menu_bar_get_child_pack_direction(GTK_MENU_BAR(self.unwrap_widget()))
        }
    }
}

impl ::ContainerTrait for MenuBar {}
impl ::MenuShellTrait for MenuBar {}
