// This file was generated by gir (5232053) from gir-files (71d73f0)
// DO NOT EDIT

use ApplicationInhibitFlags;
use Window;
use ffi;
use gio;
use gio_ffi;
use glib;
use glib::Value;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct Application(Object<ffi::GtkApplication>): [
        gio::Application => gio_ffi::GApplication,
        gio::ActionGroup => gio_ffi::GActionGroup,
        gio::ActionMap => gio_ffi::GActionMap,
    ];

    match fn {
        get_type => || ffi::gtk_application_get_type(),
    }
}

impl Application {
    pub fn add_accelerator(&self, accelerator: &str, action_name: &str, parameter: Option<&glib::Variant>) {
        unsafe {
            ffi::gtk_application_add_accelerator(self.to_glib_none().0, accelerator.to_glib_none().0, action_name.to_glib_none().0, parameter.to_glib_none().0);
        }
    }

    pub fn add_window<T: IsA<Window>>(&self, window: &T) {
        unsafe {
            ffi::gtk_application_add_window(self.to_glib_none().0, window.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_12")]
    pub fn get_accels_for_action(&self, detailed_action_name: &str) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_application_get_accels_for_action(self.to_glib_none().0, detailed_action_name.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn get_actions_for_accel(&self, accel: &str) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_application_get_actions_for_accel(self.to_glib_none().0, accel.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn get_active_window(&self) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_active_window(self.to_glib_none().0))
        }
    }

    pub fn get_app_menu(&self) -> Option<gio::MenuModel> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_app_menu(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn get_menu_by_id(&self, id: &str) -> Option<gio::Menu> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_menu_by_id(self.to_glib_none().0, id.to_glib_none().0))
        }
    }

    pub fn get_menubar(&self) -> Option<gio::MenuModel> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_menubar(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn get_window_by_id(&self, id: u32) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_window_by_id(self.to_glib_none().0, id))
        }
    }

    pub fn get_windows(&self) -> Vec<Window> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_application_get_windows(self.to_glib_none().0))
        }
    }

    pub fn inhibit<T: IsA<Window>>(&self, window: Option<&T>, flags: ApplicationInhibitFlags, reason: Option<&str>) -> u32 {
        unsafe {
            ffi::gtk_application_inhibit(self.to_glib_none().0, window.to_glib_none().0, flags.to_glib(), reason.to_glib_none().0)
        }
    }

    pub fn is_inhibited(&self, flags: ApplicationInhibitFlags) -> bool {
        unsafe {
            from_glib(ffi::gtk_application_is_inhibited(self.to_glib_none().0, flags.to_glib()))
        }
    }

    #[cfg(feature = "v3_12")]
    pub fn list_action_descriptions(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_application_list_action_descriptions(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn prefers_app_menu(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_application_prefers_app_menu(self.to_glib_none().0))
        }
    }

    pub fn remove_accelerator(&self, action_name: &str, parameter: Option<&glib::Variant>) {
        unsafe {
            ffi::gtk_application_remove_accelerator(self.to_glib_none().0, action_name.to_glib_none().0, parameter.to_glib_none().0);
        }
    }

    pub fn remove_window<T: IsA<Window>>(&self, window: &T) {
        unsafe {
            ffi::gtk_application_remove_window(self.to_glib_none().0, window.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_12")]
    pub fn set_accels_for_action(&self, detailed_action_name: &str, accels: &[&str]) {
        unsafe {
            ffi::gtk_application_set_accels_for_action(self.to_glib_none().0, detailed_action_name.to_glib_none().0, accels.to_glib_none().0);
        }
    }

    pub fn set_app_menu<T: IsA<gio::MenuModel>>(&self, app_menu: Option<&T>) {
        unsafe {
            ffi::gtk_application_set_app_menu(self.to_glib_none().0, app_menu.to_glib_none().0);
        }
    }

    pub fn set_menubar<T: IsA<gio::MenuModel>>(&self, menubar: Option<&T>) {
        unsafe {
            ffi::gtk_application_set_menubar(self.to_glib_none().0, menubar.to_glib_none().0);
        }
    }

    pub fn uninhibit(&self, cookie: u32) {
        unsafe {
            ffi::gtk_application_uninhibit(self.to_glib_none().0, cookie);
        }
    }

    pub fn get_property_active_window(&self) -> Option<Window> {
        let mut value = Value::from(None::<&Window>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "active-window".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn get_property_register_session(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "register-session".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_property_register_session(&self, register_session: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "register-session".to_glib_none().0, Value::from(&register_session).to_glib_none().0);
        }
    }

    pub fn connect_window_added<F: Fn(&Application, &Window) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Application, &Window) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "window-added",
                transmute(window_added_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_window_removed<F: Fn(&Application, &Window) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Application, &Window) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "window-removed",
                transmute(window_removed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn window_added_trampoline(this: *mut ffi::GtkApplication, window: *mut ffi::GtkWindow, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Application, &Window) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(window))
}

unsafe extern "C" fn window_removed_trampoline(this: *mut ffi::GtkApplication, window: *mut ffi::GtkWindow, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Application, &Window) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(window))
}
