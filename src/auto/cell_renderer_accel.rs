// This file was generated by gir (add4ad6) from gir-files (0bcaef9)
// DO NOT EDIT

use CellRenderer;
use CellRendererAccelMode;
use CellRendererText;
use TreePath;
use ffi;
use gdk;
use gdk_ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct CellRendererAccel(Object<ffi::GtkCellRendererAccel>): CellRendererText, CellRenderer;

    match fn {
        get_type => || ffi::gtk_cell_renderer_accel_get_type(),
    }
}

impl CellRendererAccel {
    pub fn new() -> CellRendererAccel {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(ffi::gtk_cell_renderer_accel_new()).downcast_unchecked()
        }
    }
}

pub trait CellRendererAccelExt {
    fn get_property_accel_key(&self) -> u32;

    fn set_property_accel_key(&self, accel_key: u32);

    fn get_property_accel_mode(&self) -> CellRendererAccelMode;

    fn set_property_accel_mode(&self, accel_mode: CellRendererAccelMode);

    fn get_property_accel_mods(&self) -> gdk::ModifierType;

    fn set_property_accel_mods(&self, accel_mods: gdk::ModifierType);

    fn get_property_keycode(&self) -> u32;

    fn set_property_keycode(&self, keycode: u32);

    fn connect_accel_cleared<F: Fn(&Self, TreePath) + 'static>(&self, f: F) -> u64;

    fn connect_accel_edited<F: Fn(&Self, TreePath, u32, gdk::ModifierType, u32) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<CellRendererAccel> + IsA<glib::object::Object>> CellRendererAccelExt for O {
    fn get_property_accel_key(&self) -> u32 {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "accel-key".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_accel_key(&self, accel_key: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "accel-key".to_glib_none().0, Value::from(&accel_key).to_glib_none().0);
        }
    }

    fn get_property_accel_mode(&self) -> CellRendererAccelMode {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "accel-mode".to_glib_none().0, value.to_glib_none_mut().0);
            transmute(value.get::<i32>().unwrap())
        }
    }

    fn set_property_accel_mode(&self, accel_mode: CellRendererAccelMode) {
        let accel_mode = accel_mode as i32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "accel-mode".to_glib_none().0, Value::from(&accel_mode).to_glib_none().0);
        }
    }

    fn get_property_accel_mods(&self) -> gdk::ModifierType {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "accel-mods".to_glib_none().0, value.to_glib_none_mut().0);
            transmute(value.get::<u32>().unwrap())
        }
    }

    fn set_property_accel_mods(&self, accel_mods: gdk::ModifierType) {
        let accel_mods = accel_mods.bits() as u32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "accel-mods".to_glib_none().0, Value::from(&accel_mods).to_glib_none().0);
        }
    }

    fn get_property_keycode(&self) -> u32 {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "keycode".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_keycode(&self, keycode: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "keycode".to_glib_none().0, Value::from(&keycode).to_glib_none().0);
        }
    }

    fn connect_accel_cleared<F: Fn(&Self, TreePath) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, TreePath) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "accel-cleared",
                transmute(accel_cleared_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_accel_edited<F: Fn(&Self, TreePath, u32, gdk::ModifierType, u32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, TreePath, u32, gdk::ModifierType, u32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "accel-edited",
                transmute(accel_edited_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn accel_cleared_trampoline<P>(this: *mut ffi::GtkCellRendererAccel, path_string: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<CellRendererAccel> {
    callback_guard!();
    let f: &Box_<Fn(&P, TreePath) + 'static> = transmute(f);
    let path = from_glib_full(ffi::gtk_tree_path_new_from_string(path_string));
    f(&CellRendererAccel::from_glib_none(this).downcast_unchecked(), path)
}

unsafe extern "C" fn accel_edited_trampoline<P>(this: *mut ffi::GtkCellRendererAccel, path_string: *mut libc::c_char, accel_key: libc::c_uint, accel_mods: gdk_ffi::GdkModifierType, hardware_keycode: libc::c_uint, f: glib_ffi::gpointer)
where P: IsA<CellRendererAccel> {
    callback_guard!();
    let f: &Box_<Fn(&P, TreePath, u32, gdk::ModifierType, u32) + 'static> = transmute(f);
    let path = from_glib_full(ffi::gtk_tree_path_new_from_string(path_string));
    f(&CellRendererAccel::from_glib_none(this).downcast_unchecked(), path, accel_key, accel_mods, hardware_keycode)
}
