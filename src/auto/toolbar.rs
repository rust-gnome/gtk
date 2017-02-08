// This file was generated by gir (5232053) from gir-files (71d73f0)
// DO NOT EDIT

use Container;
use IconSize;
use Orientable;
use Orientation;
use ToolItem;
use ToolShell;
use ToolbarStyle;
use Widget;
use ffi;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use signal::Inhibit;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct Toolbar(Object<ffi::GtkToolbar>): Container, Widget, Orientable, ToolShell;

    match fn {
        get_type => || ffi::gtk_toolbar_get_type(),
    }
}

impl Toolbar {
    pub fn new() -> Toolbar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_toolbar_new()).downcast_unchecked()
        }
    }

    pub fn get_drop_index(&self, x: i32, y: i32) -> i32 {
        unsafe {
            ffi::gtk_toolbar_get_drop_index(self.to_glib_none().0, x, y)
        }
    }

    pub fn get_icon_size(&self) -> IconSize {
        unsafe {
            from_glib(ffi::gtk_toolbar_get_icon_size(self.to_glib_none().0))
        }
    }

    pub fn get_item_index<T: IsA<ToolItem>>(&self, item: &T) -> i32 {
        unsafe {
            ffi::gtk_toolbar_get_item_index(self.to_glib_none().0, item.to_glib_none().0)
        }
    }

    pub fn get_n_items(&self) -> i32 {
        unsafe {
            ffi::gtk_toolbar_get_n_items(self.to_glib_none().0)
        }
    }

    pub fn get_nth_item(&self, n: i32) -> Option<ToolItem> {
        unsafe {
            from_glib_none(ffi::gtk_toolbar_get_nth_item(self.to_glib_none().0, n))
        }
    }

    pub fn get_show_arrow(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_toolbar_get_show_arrow(self.to_glib_none().0))
        }
    }

    pub fn insert<T: IsA<ToolItem>>(&self, item: &T, pos: i32) {
        unsafe {
            ffi::gtk_toolbar_insert(self.to_glib_none().0, item.to_glib_none().0, pos);
        }
    }

    pub fn set_drop_highlight_item<T: IsA<ToolItem>>(&self, tool_item: Option<&T>, index_: i32) {
        unsafe {
            ffi::gtk_toolbar_set_drop_highlight_item(self.to_glib_none().0, tool_item.to_glib_none().0, index_);
        }
    }

    pub fn set_icon_size(&self, icon_size: IconSize) {
        unsafe {
            ffi::gtk_toolbar_set_icon_size(self.to_glib_none().0, icon_size.to_glib());
        }
    }

    pub fn set_show_arrow(&self, show_arrow: bool) {
        unsafe {
            ffi::gtk_toolbar_set_show_arrow(self.to_glib_none().0, show_arrow.to_glib());
        }
    }

    pub fn set_style(&self, style: ToolbarStyle) {
        unsafe {
            ffi::gtk_toolbar_set_style(self.to_glib_none().0, style.to_glib());
        }
    }

    pub fn unset_icon_size(&self) {
        unsafe {
            ffi::gtk_toolbar_unset_icon_size(self.to_glib_none().0);
        }
    }

    pub fn unset_style(&self) {
        unsafe {
            ffi::gtk_toolbar_unset_style(self.to_glib_none().0);
        }
    }

    pub fn get_property_icon_size_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "icon-size-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_property_icon_size_set(&self, icon_size_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "icon-size-set".to_glib_none().0, Value::from(&icon_size_set).to_glib_none().0);
        }
    }

    pub fn get_property_toolbar_style(&self) -> ToolbarStyle {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "toolbar-style".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    pub fn set_property_toolbar_style(&self, toolbar_style: ToolbarStyle) {
        let toolbar_style = toolbar_style.to_glib() as i32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "toolbar-style".to_glib_none().0, Value::from(&toolbar_style).to_glib_none().0);
        }
    }

    pub fn get_item_expand<T: IsA<Widget>>(&self, item: &T) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "expand".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_item_expand<T: IsA<Widget>>(&self, item: &T, expand: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "expand".to_glib_none().0, Value::from(&expand).to_glib_none().0);
        }
    }

    pub fn get_item_homogeneous<T: IsA<Widget>>(&self, item: &T) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "homogeneous".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_item_homogeneous<T: IsA<Widget>>(&self, item: &T, homogeneous: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "homogeneous".to_glib_none().0, Value::from(&homogeneous).to_glib_none().0);
        }
    }

    pub fn connect_focus_home_or_end<F: Fn(&Toolbar, bool) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Toolbar, bool) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "focus-home-or-end",
                transmute(focus_home_or_end_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_orientation_changed<F: Fn(&Toolbar, Orientation) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Toolbar, Orientation) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "orientation-changed",
                transmute(orientation_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_popup_context_menu<F: Fn(&Toolbar, i32, i32, i32) -> Inhibit + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Toolbar, i32, i32, i32) -> Inhibit + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "popup-context-menu",
                transmute(popup_context_menu_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_style_changed<F: Fn(&Toolbar, ToolbarStyle) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Toolbar, ToolbarStyle) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "style-changed",
                transmute(style_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn focus_home_or_end_trampoline(this: *mut ffi::GtkToolbar, focus_home: glib_ffi::gboolean, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&Toolbar, bool) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this), from_glib(focus_home)).to_glib()
}

unsafe extern "C" fn orientation_changed_trampoline(this: *mut ffi::GtkToolbar, orientation: ffi::GtkOrientation, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Toolbar, Orientation) + 'static> = transmute(f);
    f(&from_glib_none(this), from_glib(orientation))
}

unsafe extern "C" fn popup_context_menu_trampoline(this: *mut ffi::GtkToolbar, x: libc::c_int, y: libc::c_int, button: libc::c_int, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&Toolbar, i32, i32, i32) -> Inhibit + 'static> = transmute(f);
    f(&from_glib_none(this), x, y, button).to_glib()
}

unsafe extern "C" fn style_changed_trampoline(this: *mut ffi::GtkToolbar, style: ffi::GtkToolbarStyle, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Toolbar, ToolbarStyle) + 'static> = transmute(f);
    f(&from_glib_none(this), from_glib(style))
}
