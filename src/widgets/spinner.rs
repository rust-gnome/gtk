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

//! Show a spinner animation

use cast::GTK_SPINNER;
use ffi;

/// Spinner — Show a spinner animation
struct_Widget!(Spinner);

pub trait SpinnerBuilder {
    fn spinner(&self) -> Option<Spinner>;
}

impl SpinnerBuilder for ::Gtk {
    fn spinner(&self) -> Option<Spinner> {
        let tmp_pointer = unsafe { ffi::gtk_spinner_new() };
        check_pointer!(tmp_pointer, Spinner)
    }
}

impl Spinner {
    pub fn start(&mut self) -> () {
        unsafe {
            ffi::gtk_spinner_start(GTK_SPINNER(self.pointer))
        }
    }

    pub fn stop(&mut self) -> () {
        unsafe {
            ffi::gtk_spinner_stop(GTK_SPINNER(self.pointer))
        }
    }

}

impl_drop!(Spinner);
impl_TraitWidget!(Spinner);

impl_widget_events!(Spinner);

