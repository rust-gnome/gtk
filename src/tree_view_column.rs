use std::mem::transmute;
use std::boxed::Box as Box_;

use CellRenderer;
use TreeIter;
use TreeModel;
use TreeViewColumn;
use ffi;
use glib_ffi;
use glib;
use glib::object::{Downcast, IsA};
use glib::translate::FromGlibPtrBorrow;

pub trait TreeViewColumnExtManual {
    fn set_cell_data_func<'a, P, F>(&self, cell_renderer: &P, closure: F)
        where P: IsA<CellRenderer>,
        F: Fn(&Self, &CellRenderer, &TreeModel, &TreeIter) + 'static;
}

impl<O: IsA<TreeViewColumn> + IsA<glib::object::Object>> TreeViewColumnExtManual for O {
    fn set_cell_data_func<'a, P, F>(&self, cell_renderer: &P, closure: F)
        where P: IsA<CellRenderer>,
        F: Fn(&Self, &CellRenderer, &TreeModel, &TreeIter) + 'static
    {
       unsafe {
           let f: Box_<Box_<Fn(&Self, &CellRenderer, &TreeModel, &TreeIter) + 'static>> = Box_::new(Box_::new(closure));
           ffi::gtk_tree_view_column_set_cell_data_func(
               self.to_glib_none().0,
               cell_renderer.to_glib_none().0,
               Some(transmute(tree_cell_data_func_trampoline::<Self> as usize)),
               Box_::into_raw(f) as *mut _,
               Some(destroy_notify)
           )
        }
    }
}

unsafe extern "C" fn tree_cell_data_func_trampoline<P>(
    col: *mut ffi::GtkTreeViewColumn,
    cell: *mut ffi::GtkCellRenderer,
    model: *mut ffi::GtkTreeModel,
    iter: *mut ffi::GtkTreeIter,
    f: glib_ffi::gpointer
) where P: IsA<TreeViewColumn>
{
    // callback_guard!();
    let f: &&(Fn(&P, &CellRenderer, &TreeModel, &TreeIter) + 'static) = transmute(f);
    f(
        &TreeViewColumn::from_glib_borrow(col).downcast_unchecked(),
        &CellRenderer::from_glib_borrow(cell).downcast_unchecked(),
        &TreeModel::from_glib_borrow(model).downcast_unchecked(),
        &TreeIter::from_glib_borrow(iter),
    );
}

unsafe extern fn destroy_notify(ptr: glib_ffi::gpointer) {
    Box::<Box<Fn(&TreeViewColumn, &CellRenderer, &TreeModel, &TreeIter) + 'static>>::from_raw(ptr as *mut _);
}
