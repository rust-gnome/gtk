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

//! A stacking container

// FIXME: add missing methods (3.12)

use ffi;
use cast::GTK_STACK;
use glib::translate::{FromGlibPtr, ToGlibPtr};
use glib::{to_bool, to_gboolean};

/// GtkStack — A stacking container
struct_Widget!(Stack);

pub trait StackBuilder {
    fn stack(&self) -> Option<Stack>;
}

impl StackBuilder for ::Gtk {
    fn stack(&self) -> Option<Stack> {
        let tmp_pointer = unsafe { ffi::gtk_stack_new() };
        check_pointer!(tmp_pointer, Stack)
    }
}

impl Stack {
    pub fn add_named<T: ::WidgetTrait>(&mut self, child: &T, name: &str) {
        unsafe {
            ffi::gtk_stack_add_named(GTK_STACK(self.pointer),
                                     child.unwrap_widget(),
                                     name.borrow_to_glib().0)
        }
    }

    pub fn add_titled<T: ::WidgetTrait>(&mut self, child: &T, name: &str, title: &str) {
        unsafe {
            ffi::gtk_stack_add_titled(GTK_STACK(self.pointer),
                                      child.unwrap_widget(),
                                      name.borrow_to_glib().0,
                                      title.borrow_to_glib().0)
        }
    }

    pub fn set_visible_child<T: ::WidgetTrait>(&mut self, child: &T) {
        unsafe {
            ffi::gtk_stack_set_visible_child(GTK_STACK(self.pointer),
                                             child.unwrap_widget())
        }
    }

    pub fn get_visible_child<T: ::WidgetTrait>(&self) -> Option<T> {
        let tmp_pointer = unsafe { ffi::gtk_stack_get_visible_child(GTK_STACK(self.pointer)) };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }

    pub fn set_visible_child_name(&mut self, name: &str) {
        unsafe {
            ffi::gtk_stack_set_visible_child_name(GTK_STACK(self.pointer),
                                                  name.borrow_to_glib().0)
        }
    }

    pub fn get_visible_child_name(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow(
                ffi::gtk_stack_get_visible_child_name(GTK_STACK(self.pointer)))
        }
    }

    pub fn set_visible_child_full(&mut self, name: &str, transition: ::StackTransitionType) {
        unsafe {
            ffi::gtk_stack_set_visible_child_full(GTK_STACK(self.pointer),
                                                  name.borrow_to_glib().0,
                                                  transition)
        }
    }

    pub fn set_homogeneous(&mut self, homogeneous: bool) {
        unsafe {
            ffi::gtk_stack_set_homogeneous(GTK_STACK(self.pointer), to_gboolean(homogeneous))
        }
    }

    pub fn is_homogeneous(&self) -> bool {
        unsafe {
            to_bool(ffi::gtk_stack_get_homogeneous(GTK_STACK(self.pointer)))
        }
    }

    pub fn set_transition_duration(&mut self, duration: u32) {
        unsafe {
            ffi::gtk_stack_set_transition_duration(GTK_STACK(self.pointer), duration)
        }
    }

    pub fn get_transition_duration(&self) -> u32 {
        unsafe {
            ffi::gtk_stack_get_transition_duration(GTK_STACK(self.pointer))
        }
    }

    pub fn set_transition_type(&mut self, transition: ::StackTransitionType) {
        unsafe {
            ffi::gtk_stack_set_transition_type(GTK_STACK(self.pointer), transition)
        }
    }

    pub fn get_transition_type(&self) -> ::StackTransitionType {
        unsafe {
            ffi::gtk_stack_get_transition_type(GTK_STACK(self.pointer))
        }
    }
}

impl_drop!(Stack);
impl_TraitWidget!(Stack);

impl ::ContainerTrait for Stack {}

impl_widget_events!(Stack);

