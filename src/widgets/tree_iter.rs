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

pub struct TreeIter {
    pointer: *mut ffi::C_GtkTreeIter,
    is_owned: bool,
}

pub trait TreeIterBuilder {
    fn tree_iter(&self) -> Option<TreeIter>;
}

impl TreeIterBuilder for ::Gtk {
    fn tree_iter(&self) -> Option<TreeIter> {
        match unsafe { ffi::gtk_tree_iter_copy(::std::mem::uninitialized()) } {
            pointer if !pointer.is_null() => Some(TreeIter { pointer: pointer, is_owned: true }),
            _ => None
        }
    }
}

impl TreeIter {
    pub fn copy(&self) -> Option<TreeIter> {
        let tmp_pointer = unsafe { ffi::gtk_tree_iter_copy(self.pointer) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(TreeIter {
                pointer: tmp_pointer,
                is_owned: true,
            })
        }
    }

    #[doc(hidden)]
    pub fn unwrap_pointer(&self) -> *mut ffi::C_GtkTreeIter {
        self.pointer
    }

    #[doc(hidden)]
    pub fn wrap_pointer(c_treeiter: *mut ffi::C_GtkTreeIter) -> TreeIter {
        TreeIter {
            pointer: c_treeiter,
            is_owned: false,
        }
    }
}

impl Drop for TreeIter {
    fn drop(&mut self) {
        if !self.pointer.is_null() && self.is_owned {
            unsafe { ffi::gtk_tree_iter_free(self.pointer) };
            self.pointer = ::std::ptr::null_mut();
        }
    }
}

