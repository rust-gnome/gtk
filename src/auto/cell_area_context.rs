// This file was generated by gir (28183c8) from gir-files (71d73f0)
// DO NOT EDIT

use CellArea;
use ffi;
use glib;
use glib::Value;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;
use std::mem;

glib_wrapper! {
    pub struct CellAreaContext(Object<ffi::GtkCellAreaContext>);

    match fn {
        get_type => || ffi::gtk_cell_area_context_get_type(),
    }
}

pub trait CellAreaContextExt {
    fn allocate(&self, width: i32, height: i32);

    fn get_allocation(&self) -> (i32, i32);

    fn get_area(&self) -> Option<CellArea>;

    fn get_preferred_height(&self) -> (i32, i32);

    fn get_preferred_height_for_width(&self, width: i32) -> (i32, i32);

    fn get_preferred_width(&self) -> (i32, i32);

    fn get_preferred_width_for_height(&self, height: i32) -> (i32, i32);

    fn push_preferred_height(&self, minimum_height: i32, natural_height: i32);

    fn push_preferred_width(&self, minimum_width: i32, natural_width: i32);

    fn reset(&self);

    fn get_property_minimum_height(&self) -> i32;

    fn get_property_minimum_width(&self) -> i32;

    fn get_property_natural_height(&self) -> i32;

    fn get_property_natural_width(&self) -> i32;
}

impl<O: IsA<CellAreaContext> + IsA<glib::object::Object>> CellAreaContextExt for O {
    fn allocate(&self, width: i32, height: i32) {
        unsafe {
            ffi::gtk_cell_area_context_allocate(self.to_glib_none().0, width, height);
        }
    }

    fn get_allocation(&self) -> (i32, i32) {
        unsafe {
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            ffi::gtk_cell_area_context_get_allocation(self.to_glib_none().0, &mut width, &mut height);
            (width, height)
        }
    }

    fn get_area(&self) -> Option<CellArea> {
        unsafe {
            from_glib_none(ffi::gtk_cell_area_context_get_area(self.to_glib_none().0))
        }
    }

    fn get_preferred_height(&self) -> (i32, i32) {
        unsafe {
            let mut minimum_height = mem::uninitialized();
            let mut natural_height = mem::uninitialized();
            ffi::gtk_cell_area_context_get_preferred_height(self.to_glib_none().0, &mut minimum_height, &mut natural_height);
            (minimum_height, natural_height)
        }
    }

    fn get_preferred_height_for_width(&self, width: i32) -> (i32, i32) {
        unsafe {
            let mut minimum_height = mem::uninitialized();
            let mut natural_height = mem::uninitialized();
            ffi::gtk_cell_area_context_get_preferred_height_for_width(self.to_glib_none().0, width, &mut minimum_height, &mut natural_height);
            (minimum_height, natural_height)
        }
    }

    fn get_preferred_width(&self) -> (i32, i32) {
        unsafe {
            let mut minimum_width = mem::uninitialized();
            let mut natural_width = mem::uninitialized();
            ffi::gtk_cell_area_context_get_preferred_width(self.to_glib_none().0, &mut minimum_width, &mut natural_width);
            (minimum_width, natural_width)
        }
    }

    fn get_preferred_width_for_height(&self, height: i32) -> (i32, i32) {
        unsafe {
            let mut minimum_width = mem::uninitialized();
            let mut natural_width = mem::uninitialized();
            ffi::gtk_cell_area_context_get_preferred_width_for_height(self.to_glib_none().0, height, &mut minimum_width, &mut natural_width);
            (minimum_width, natural_width)
        }
    }

    fn push_preferred_height(&self, minimum_height: i32, natural_height: i32) {
        unsafe {
            ffi::gtk_cell_area_context_push_preferred_height(self.to_glib_none().0, minimum_height, natural_height);
        }
    }

    fn push_preferred_width(&self, minimum_width: i32, natural_width: i32) {
        unsafe {
            ffi::gtk_cell_area_context_push_preferred_width(self.to_glib_none().0, minimum_width, natural_width);
        }
    }

    fn reset(&self) {
        unsafe {
            ffi::gtk_cell_area_context_reset(self.to_glib_none().0);
        }
    }

    fn get_property_minimum_height(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "minimum-height".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_minimum_width(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "minimum-width".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_natural_height(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "natural-height".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_natural_width(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "natural-width".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }
}
