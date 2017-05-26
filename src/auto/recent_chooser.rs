// This file was generated by gir (28183c8) from gir-files (71d73f0)
// DO NOT EDIT

use Error;
use RecentFilter;
use RecentInfo;
use RecentSortType;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct RecentChooser(Object<ffi::GtkRecentChooser>);

    match fn {
        get_type => || ffi::gtk_recent_chooser_get_type(),
    }
}

pub trait RecentChooserExt {
    fn add_filter(&self, filter: &RecentFilter);

    fn get_current_item(&self) -> Option<RecentInfo>;

    fn get_current_uri(&self) -> Option<String>;

    fn get_filter(&self) -> Option<RecentFilter>;

    fn get_items(&self) -> Vec<RecentInfo>;

    fn get_limit(&self) -> i32;

    fn get_local_only(&self) -> bool;

    fn get_select_multiple(&self) -> bool;

    fn get_show_icons(&self) -> bool;

    fn get_show_not_found(&self) -> bool;

    fn get_show_private(&self) -> bool;

    fn get_show_tips(&self) -> bool;

    fn get_sort_type(&self) -> RecentSortType;

    //fn get_uris(&self) -> (Vec<String>, /*Unimplemented*/Fundamental: Size);

    fn list_filters(&self) -> Vec<RecentFilter>;

    fn remove_filter(&self, filter: &RecentFilter);

    fn select_all(&self);

    fn select_uri(&self, uri: &str) -> Result<(), Error>;

    fn set_current_uri(&self, uri: &str) -> Result<(), Error>;

    fn set_filter<'a, P: Into<Option<&'a RecentFilter>>>(&self, filter: P);

    fn set_limit(&self, limit: i32);

    fn set_local_only(&self, local_only: bool);

    fn set_select_multiple(&self, select_multiple: bool);

    fn set_show_icons(&self, show_icons: bool);

    fn set_show_not_found(&self, show_not_found: bool);

    fn set_show_private(&self, show_private: bool);

    fn set_show_tips(&self, show_tips: bool);

    //fn set_sort_func<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a /*Ignored*/glib::DestroyNotify>>>(&self, sort_func: /*Unknown conversion*//*Unimplemented*/RecentSortFunc, sort_data: P, data_destroy: Q);

    fn set_sort_type(&self, sort_type: RecentSortType);

    fn unselect_all(&self);

    fn unselect_uri(&self, uri: &str);

    fn connect_item_activated<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_selection_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<RecentChooser> + IsA<glib::object::Object>> RecentChooserExt for O {
    fn add_filter(&self, filter: &RecentFilter) {
        unsafe {
            ffi::gtk_recent_chooser_add_filter(self.to_glib_none().0, filter.to_glib_none().0);
        }
    }

    fn get_current_item(&self) -> Option<RecentInfo> {
        unsafe {
            from_glib_full(ffi::gtk_recent_chooser_get_current_item(self.to_glib_none().0))
        }
    }

    fn get_current_uri(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_recent_chooser_get_current_uri(self.to_glib_none().0))
        }
    }

    fn get_filter(&self) -> Option<RecentFilter> {
        unsafe {
            from_glib_none(ffi::gtk_recent_chooser_get_filter(self.to_glib_none().0))
        }
    }

    fn get_items(&self) -> Vec<RecentInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_recent_chooser_get_items(self.to_glib_none().0))
        }
    }

    fn get_limit(&self) -> i32 {
        unsafe {
            ffi::gtk_recent_chooser_get_limit(self.to_glib_none().0)
        }
    }

    fn get_local_only(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_local_only(self.to_glib_none().0))
        }
    }

    fn get_select_multiple(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_select_multiple(self.to_glib_none().0))
        }
    }

    fn get_show_icons(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_show_icons(self.to_glib_none().0))
        }
    }

    fn get_show_not_found(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_show_not_found(self.to_glib_none().0))
        }
    }

    fn get_show_private(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_show_private(self.to_glib_none().0))
        }
    }

    fn get_show_tips(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_show_tips(self.to_glib_none().0))
        }
    }

    fn get_sort_type(&self) -> RecentSortType {
        unsafe {
            from_glib(ffi::gtk_recent_chooser_get_sort_type(self.to_glib_none().0))
        }
    }

    //fn get_uris(&self) -> (Vec<String>, /*Unimplemented*/Fundamental: Size) {
    //    unsafe { TODO: call ffi::gtk_recent_chooser_get_uris() }
    //}

    fn list_filters(&self) -> Vec<RecentFilter> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_recent_chooser_list_filters(self.to_glib_none().0))
        }
    }

    fn remove_filter(&self, filter: &RecentFilter) {
        unsafe {
            ffi::gtk_recent_chooser_remove_filter(self.to_glib_none().0, filter.to_glib_none().0);
        }
    }

    fn select_all(&self) {
        unsafe {
            ffi::gtk_recent_chooser_select_all(self.to_glib_none().0);
        }
    }

    fn select_uri(&self, uri: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_recent_chooser_select_uri(self.to_glib_none().0, uri.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_current_uri(&self, uri: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_recent_chooser_set_current_uri(self.to_glib_none().0, uri.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_filter<'a, P: Into<Option<&'a RecentFilter>>>(&self, filter: P) {
        let filter = filter.into();
        let filter = filter.to_glib_none();
        unsafe {
            ffi::gtk_recent_chooser_set_filter(self.to_glib_none().0, filter.0);
        }
    }

    fn set_limit(&self, limit: i32) {
        unsafe {
            ffi::gtk_recent_chooser_set_limit(self.to_glib_none().0, limit);
        }
    }

    fn set_local_only(&self, local_only: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_local_only(self.to_glib_none().0, local_only.to_glib());
        }
    }

    fn set_select_multiple(&self, select_multiple: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_select_multiple(self.to_glib_none().0, select_multiple.to_glib());
        }
    }

    fn set_show_icons(&self, show_icons: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_show_icons(self.to_glib_none().0, show_icons.to_glib());
        }
    }

    fn set_show_not_found(&self, show_not_found: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_show_not_found(self.to_glib_none().0, show_not_found.to_glib());
        }
    }

    fn set_show_private(&self, show_private: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_show_private(self.to_glib_none().0, show_private.to_glib());
        }
    }

    fn set_show_tips(&self, show_tips: bool) {
        unsafe {
            ffi::gtk_recent_chooser_set_show_tips(self.to_glib_none().0, show_tips.to_glib());
        }
    }

    //fn set_sort_func<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a /*Ignored*/glib::DestroyNotify>>>(&self, sort_func: /*Unknown conversion*//*Unimplemented*/RecentSortFunc, sort_data: P, data_destroy: Q) {
    //    unsafe { TODO: call ffi::gtk_recent_chooser_set_sort_func() }
    //}

    fn set_sort_type(&self, sort_type: RecentSortType) {
        unsafe {
            ffi::gtk_recent_chooser_set_sort_type(self.to_glib_none().0, sort_type.to_glib());
        }
    }

    fn unselect_all(&self) {
        unsafe {
            ffi::gtk_recent_chooser_unselect_all(self.to_glib_none().0);
        }
    }

    fn unselect_uri(&self, uri: &str) {
        unsafe {
            ffi::gtk_recent_chooser_unselect_uri(self.to_glib_none().0, uri.to_glib_none().0);
        }
    }

    fn connect_item_activated<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "item-activated",
                transmute(item_activated_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_selection_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "selection-changed",
                transmute(selection_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn item_activated_trampoline<P>(this: *mut ffi::GtkRecentChooser, f: glib_ffi::gpointer)
where P: IsA<RecentChooser> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&RecentChooser::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn selection_changed_trampoline<P>(this: *mut ffi::GtkRecentChooser, f: glib_ffi::gpointer)
where P: IsA<RecentChooser> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&RecentChooser::from_glib_none(this).downcast_unchecked())
}
