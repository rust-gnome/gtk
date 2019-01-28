// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use ffi::{GtkTreeModel, GtkTreePath, GtkTreeSelection};
use glib::object::{Cast, IsA};
use glib::translate::*;
use glib_ffi::{gboolean, gpointer};
use std::mem::transmute;
use {TreeModel, TreePath, TreeSelection};

pub trait TreeSelectionExtManual: 'static {
    fn set_select_function<F>(&self, f: F)
    where
        F: Fn(&Self, &TreeModel, &TreePath, bool) -> bool;
}

unsafe extern "C" fn trampoline<T>(
    this: *mut GtkTreeSelection,
    model: *mut GtkTreeModel,
    path: *mut GtkTreePath,
    selected: gboolean,
    f: gpointer,
) -> i32
where
    T: IsA<TreeSelection>,
{
    let f: &&(Fn(&T, &TreeModel, &TreePath, bool) -> bool) = transmute(f);
    f(
        &TreeSelection::from_glib_none(this).unsafe_cast(),
        &from_glib_borrow(model),
        &from_glib_borrow(path),
        from_glib(selected),
    )
    .to_glib()
}

unsafe extern "C" fn destroy_closure<T>(ptr: gpointer) {
    Box::<Box<Fn(&T, &TreeModel, &TreePath, bool) -> bool>>::from_raw(ptr as *mut _);
}

fn into_raw<F, T>(func: F) -> gpointer
where
    F: Fn(&T, &TreeModel, &TreePath, bool) -> bool,
{
    skip_assert_initialized!();
    let func: Box<Box<Fn(&T, &TreeModel, &TreePath, bool) -> bool>> = Box::new(Box::new(func));
    Box::into_raw(func) as gpointer
}

impl<O: IsA<TreeSelection>> TreeSelectionExtManual for O {
    fn set_select_function<F>(&self, f: F)
    where
        F: Fn(&Self, &TreeModel, &TreePath, bool) -> bool,
    {
        unsafe {
            ffi::gtk_tree_selection_set_select_function(
                self.as_ref().to_glib_none().0,
                Some(trampoline::<Self>),
                into_raw(f),
                Some(destroy_closure::<Self>),
            )
        }
    }
}
