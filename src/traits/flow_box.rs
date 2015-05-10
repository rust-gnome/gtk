// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use cast::{GTK_FLOW_BOX, GTK_FLOW_BOX_CHILD};
use ffi;
use glib::{to_bool, to_gboolean};
use FlowBoxChild;
use traits::FFIWidget;

pub trait FlowBoxTrait: ::WidgetTrait + ::ContainerTrait {
    fn set_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_flow_box_set_homogeneous(GTK_FLOW_BOX(self.unwrap_widget()),
                                              to_gboolean(homogeneous))
        }
    }

    fn is_homogeneous(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_flow_box_get_homogeneous(GTK_FLOW_BOX(self.unwrap_widget())))
        }
    }

    fn get_row_spacing(&self) -> u32 {
        unsafe {
            ffi::gtk_flow_box_get_row_spacing(GTK_FLOW_BOX(self.unwrap_widget()))
        }
    }

    fn set_row_spacing(&self, spacing: u32) {
        unsafe {
            ffi::gtk_flow_box_set_row_spacing(GTK_FLOW_BOX(self.unwrap_widget()), spacing)
        }
    }

    fn set_colum_spacing(&self, spacing: u32) {
        unsafe {
            ffi::gtk_flow_box_set_column_spacing(GTK_FLOW_BOX(self.unwrap_widget()), spacing)
        }
    }

    fn get_column_spacing(&self) -> u32 {
        unsafe {
            ffi::gtk_flow_box_get_column_spacing(GTK_FLOW_BOX(self.unwrap_widget()))
        }
    }

    fn set_min_children_per_line(&self, n_children: u32) {
        unsafe {
            ffi::gtk_flow_box_set_min_children_per_line(GTK_FLOW_BOX(self.unwrap_widget()), n_children)
        }
    }

    fn get_min_children_per_line(&self) -> u32 {
        unsafe {
            ffi::gtk_flow_box_get_min_children_per_line(GTK_FLOW_BOX(self.unwrap_widget()))
        }
    }

    fn set_max_children_per_line(&self, n_children: u32) {
        unsafe {
            ffi::gtk_flow_box_set_max_children_per_line(GTK_FLOW_BOX(self.unwrap_widget()), n_children)
        }
    }

    fn get_max_children_per_line(&self) -> u32 {
        unsafe {
            ffi::gtk_flow_box_get_max_children_per_line(GTK_FLOW_BOX(self.unwrap_widget()))
        }
    }

    fn set_activate_on_single_click(&self, single: bool) {
        unsafe {
            ffi::gtk_flow_box_set_activate_on_single_click(GTK_FLOW_BOX(self.unwrap_widget()),
                                                           to_gboolean(single))
        }
    }

    fn is_activate_on_single_click(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_flow_box_get_activate_on_single_click(GTK_FLOW_BOX(self.unwrap_widget())))
        }
    }

    fn insert<T: ::WidgetTrait>(&self, widget: &T, position: i32) {
        unsafe {
            ffi::gtk_flow_box_insert(GTK_FLOW_BOX(self.unwrap_widget()),
                                     widget.unwrap_widget(),
                                     position)
        }
    }

    fn get_child_at_index(&self, idx: i32) -> Option<FlowBoxChild> {
        let tmp_pointer = unsafe {
            ffi::gtk_flow_box_get_child_at_index(GTK_FLOW_BOX(self.unwrap_widget()), idx)
        };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer as *mut ffi::C_GtkWidget))
        }
    }

    fn select_child(&self, child: &FlowBoxChild) {
        unsafe {
            ffi::gtk_flow_box_select_child(GTK_FLOW_BOX(self.unwrap_widget()),
                                           GTK_FLOW_BOX_CHILD(child.unwrap_widget()))
        }
    }

    fn unselect_child(&self, child: &FlowBoxChild ) {
        unsafe {
            ffi::gtk_flow_box_unselect_child(GTK_FLOW_BOX(self.unwrap_widget()),
                                             GTK_FLOW_BOX_CHILD(child.unwrap_widget()))
        }
    }

    fn select_all(&self) {
        unsafe {
            ffi::gtk_flow_box_select_all(GTK_FLOW_BOX(self.unwrap_widget()))
        }
    }

    fn unselect_all(&self) {
        unsafe {
            ffi::gtk_flow_box_unselect_all(GTK_FLOW_BOX(self.unwrap_widget()))
        }
    }

    fn set_selection_mode(&self, mode: ::SelectionMode) {
        unsafe {
            ffi::gtk_flow_box_set_selection_mode(GTK_FLOW_BOX(self.unwrap_widget()),
                                                 mode)
        }
    }

    fn get_selection_mode(&self) -> ::SelectionMode {
        unsafe {
            ffi::gtk_flow_box_get_selection_mode(GTK_FLOW_BOX(self.unwrap_widget()))
        }
    }

    fn set_hadjustment(&self, adjustment: ::Adjustment) {
        unsafe {
            ffi::gtk_flow_box_set_hadjustment(GTK_FLOW_BOX(self.unwrap_widget()),
                                              adjustment.unwrap_pointer())
        }
    }

    fn set_vadjustment(&self, adjustment: ::Adjustment) {
        unsafe {
            ffi::gtk_flow_box_set_vadjustment(GTK_FLOW_BOX(self.unwrap_widget()),
                                              adjustment.unwrap_pointer())
        }
    }
}

pub trait FlowBoxChildTrait: ::WidgetTrait + ::ContainerTrait {
    fn get_index(&self) -> i32 {
        unsafe {
            ffi::gtk_flow_box_child_get_index(GTK_FLOW_BOX_CHILD(self.unwrap_widget()))
        }
    }

    fn is_selected(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_flow_box_child_is_selected(GTK_FLOW_BOX_CHILD(self.unwrap_widget())))
        }
    }

    fn changed(&self) {
        unsafe {
            ffi::gtk_flow_box_child_changed(GTK_FLOW_BOX_CHILD(self.unwrap_widget()))
        }
    }
}
