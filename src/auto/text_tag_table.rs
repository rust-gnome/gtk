// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Buildable;
use TextTag;

glib_wrapper! {
    pub struct TextTagTable(Object<gtk_sys::GtkTextTagTable, gtk_sys::GtkTextTagTableClass, TextTagTableClass>) @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_text_tag_table_get_type(),
    }
}

impl TextTagTable {
    pub fn new() -> TextTagTable {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gtk_sys::gtk_text_tag_table_new()) }
    }
}

impl Default for TextTagTable {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_TEXT_TAG_TABLE: Option<&TextTagTable> = None;

pub trait TextTagTableExt: 'static {
    fn add<P: IsA<TextTag>>(&self, tag: &P) -> bool;

    fn foreach<P: FnMut(&TextTag)>(&self, func: P);

    fn get_size(&self) -> i32;

    fn lookup(&self, name: &str) -> Option<TextTag>;

    fn remove<P: IsA<TextTag>>(&self, tag: &P);

    fn connect_tag_added<F: Fn(&Self, &TextTag) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_tag_changed<F: Fn(&Self, &TextTag, bool) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_tag_removed<F: Fn(&Self, &TextTag) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TextTagTable>> TextTagTableExt for O {
    fn add<P: IsA<TextTag>>(&self, tag: &P) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_tag_table_add(
                self.as_ref().to_glib_none().0,
                tag.as_ref().to_glib_none().0,
            ))
        }
    }

    fn foreach<P: FnMut(&TextTag)>(&self, func: P) {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&TextTag)>(
            tag: *mut gtk_sys::GtkTextTag,
            data: glib_sys::gpointer,
        ) {
            let tag = from_glib_borrow(tag);
            let callback: *mut P = data as *const _ as usize as *mut P;
            (*callback)(&tag);
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            gtk_sys::gtk_text_tag_table_foreach(
                self.as_ref().to_glib_none().0,
                func,
                super_callback0 as *const _ as usize as *mut _,
            );
        }
    }

    fn get_size(&self) -> i32 {
        unsafe { gtk_sys::gtk_text_tag_table_get_size(self.as_ref().to_glib_none().0) }
    }

    fn lookup(&self, name: &str) -> Option<TextTag> {
        unsafe {
            from_glib_none(gtk_sys::gtk_text_tag_table_lookup(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    fn remove<P: IsA<TextTag>>(&self, tag: &P) {
        unsafe {
            gtk_sys::gtk_text_tag_table_remove(
                self.as_ref().to_glib_none().0,
                tag.as_ref().to_glib_none().0,
            );
        }
    }

    fn connect_tag_added<F: Fn(&Self, &TextTag) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn tag_added_trampoline<P, F: Fn(&P, &TextTag) + 'static>(
            this: *mut gtk_sys::GtkTextTagTable,
            tag: *mut gtk_sys::GtkTextTag,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TextTagTable>,
        {
            let f: &F = &*(f as *const F);
            f(
                &TextTagTable::from_glib_borrow(this).unsafe_cast(),
                &from_glib_borrow(tag),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"tag-added\0".as_ptr() as *const _,
                Some(transmute(tag_added_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_tag_changed<F: Fn(&Self, &TextTag, bool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn tag_changed_trampoline<P, F: Fn(&P, &TextTag, bool) + 'static>(
            this: *mut gtk_sys::GtkTextTagTable,
            tag: *mut gtk_sys::GtkTextTag,
            size_changed: glib_sys::gboolean,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TextTagTable>,
        {
            let f: &F = &*(f as *const F);
            f(
                &TextTagTable::from_glib_borrow(this).unsafe_cast(),
                &from_glib_borrow(tag),
                from_glib(size_changed),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"tag-changed\0".as_ptr() as *const _,
                Some(transmute(tag_changed_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_tag_removed<F: Fn(&Self, &TextTag) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn tag_removed_trampoline<P, F: Fn(&P, &TextTag) + 'static>(
            this: *mut gtk_sys::GtkTextTagTable,
            tag: *mut gtk_sys::GtkTextTag,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TextTagTable>,
        {
            let f: &F = &*(f as *const F);
            f(
                &TextTagTable::from_glib_borrow(this).unsafe_cast(),
                &from_glib_borrow(tag),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"tag-removed\0".as_ptr() as *const _,
                Some(transmute(tag_removed_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for TextTagTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TextTagTable")
    }
}
