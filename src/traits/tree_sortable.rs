// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GtkTreeSortable — The interface for sortable models used by GtkTreeView

use cast::GTK_TREE_SORTABLE;
use ffi;
use glib::to_bool;
use std::{ptr, mem};
use libc::c_void;

fn ffi_sort_func(model: *mut ffi::GtkTreeModel, a: *mut ffi::GtkTreeIter, b: *mut ffi::GtkTreeIter,
    data: &mut (FnMut(&::TreeModel, &::TreeIter, &::TreeIter, *mut c_void), *mut c_void)) {
    unsafe { data.0(&::TreeModel::wrap_pointer(model), &::TreeIter::wrap_pointer(a), &::TreeIter::wrap_pointer(b),
        mem::transmute(data.1)) }
}

pub trait TreeSortableTrait: ::WidgetTrait {
    fn get_sort_column_id(&self, sort_column_id: &mut i32, order: &mut ffi::enums::SortType) -> bool {
        unsafe { to_bool(ffi::gtk_tree_sortable_get_sort_column_id(GTK_TREE_SORTABLE(self.unwrap_widget()),
            sort_column_id, order)) }
    }

    fn set_sort_column_id(&self, sort_column_id: i32, order: ffi::enums::SortType) {
        unsafe { ffi::gtk_tree_sortable_set_sort_column_id(GTK_TREE_SORTABLE(self.unwrap_widget()),
            sort_column_id, order) }
    }

    fn set_sort_func<T, F>(&self, sort_column_id: i32, sort_func: Option<F>, user_data: Option<&mut T>)
        where F : FnMut(&::TreeModel, &::TreeIter, &::TreeIter, &mut T) {
        match sort_func {
            Some(f) => unsafe { ffi::gtk_tree_sortable_set_sort_func(GTK_TREE_SORTABLE(self.unwrap_widget()), sort_column_id,
                mem::transmute(ffi_sort_func), mem::transmute(&mut (f, user_data.unwrap())), ptr::null_mut()) },
            None => unsafe { ffi::gtk_tree_sortable_set_sort_func(GTK_TREE_SORTABLE(self.unwrap_widget()), sort_column_id,
                ptr::null_mut(), ptr::null_mut(), ptr::null_mut()) }
        }
    }

    fn set_default_func<T, F>(&self, sort_column_id: i32, sort_func: Option<F>, user_data: Option<&mut T>)
        where F : FnMut(&::TreeModel, &::TreeIter, &::TreeIter, &mut T) {
        match sort_func {
            Some(f) => unsafe { ffi::gtk_tree_sortable_set_default_sort_func(GTK_TREE_SORTABLE(self.unwrap_widget()), sort_column_id,
                mem::transmute(ffi_sort_func), mem::transmute(&mut (f, user_data.unwrap())), ptr::null_mut()) },
            None => unsafe { ffi::gtk_tree_sortable_set_default_sort_func(GTK_TREE_SORTABLE(self.unwrap_widget()), sort_column_id,
                ptr::null_mut(), ptr::null_mut(), ptr::null_mut()) }
        }
    }

    fn has_default_sort_func(&self) -> bool {
        unsafe { to_bool(ffi::gtk_tree_sortable_has_default_sort_func(GTK_TREE_SORTABLE(self.unwrap_widget()))) }
    }
}
