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

//! A Scrollbar

use ffi;

/// GtkScrollBar — A Scrollbar
struct_Widget!(ScrollBar);

pub trait ScrollBarBuilder {
    fn scroll_bar(&self, orientation: ::Orientation, adjustment: &::Adjustment) ->
        Option<ScrollBar>;
}

impl ScrollBarBuilder for ::Gtk {
    fn scroll_bar(&self, orientation: ::Orientation, adjustment: &::Adjustment) ->
            Option<ScrollBar> {
        let tmp_pointer = unsafe { ffi::gtk_scrollbar_new(orientation,
            adjustment.unwrap_pointer()) };
        check_pointer!(tmp_pointer, ScrollBar)
    }
}

impl_drop!(ScrollBar);
impl_TraitWidget!(ScrollBar);

impl ::RangeTrait for ScrollBar {}
impl ::OrientableTrait for ScrollBar {}

impl_widget_events!(ScrollBar);
impl_range_events!(ScrollBar);

