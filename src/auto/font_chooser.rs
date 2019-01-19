// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use glib_ffi::gpointer;
use libc;
use pango;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct FontChooser(Interface<ffi::GtkFontChooser>);

    match fn {
        get_type => || ffi::gtk_font_chooser_get_type(),
    }
}

pub const NONE_FONT_CHOOSER: Option<&FontChooser> = None;

pub trait FontChooserExt: 'static {
    fn get_font(&self) -> Option<GString>;

    fn get_font_desc(&self) -> Option<pango::FontDescription>;

    fn get_font_face(&self) -> Option<pango::FontFace>;

    fn get_font_family(&self) -> Option<pango::FontFamily>;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_font_map(&self) -> Option<pango::FontMap>;

    fn get_font_size(&self) -> i32;

    fn get_preview_text(&self) -> Option<GString>;

    fn get_show_preview_entry(&self) -> bool;

    fn set_filter_func<P: Fn(pango::FontFamily, pango::FontFace) -> bool + Send + Sync + 'static, Q: Into<Option<P>>>(&self, filter: Q);

    fn set_font(&self, fontname: &str);

    fn set_font_desc(&self, font_desc: &pango::FontDescription);

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn set_font_map<'a, P: IsA<pango::FontMap> + 'a, Q: Into<Option<&'a P>>>(&self, fontmap: Q);

    fn set_preview_text(&self, text: &str);

    fn set_show_preview_entry(&self, show_preview_entry: bool);

    fn connect_font_activated<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_font_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_font_desc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_preview_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_preview_entry_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FontChooser>> FontChooserExt for O {
    fn get_font(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::gtk_font_chooser_get_font(self.as_ref().to_glib_none().0))
        }
    }

    fn get_font_desc(&self) -> Option<pango::FontDescription> {
        unsafe {
            from_glib_full(ffi::gtk_font_chooser_get_font_desc(self.as_ref().to_glib_none().0))
        }
    }

    fn get_font_face(&self) -> Option<pango::FontFace> {
        unsafe {
            from_glib_none(ffi::gtk_font_chooser_get_font_face(self.as_ref().to_glib_none().0))
        }
    }

    fn get_font_family(&self) -> Option<pango::FontFamily> {
        unsafe {
            from_glib_none(ffi::gtk_font_chooser_get_font_family(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_font_map(&self) -> Option<pango::FontMap> {
        unsafe {
            from_glib_full(ffi::gtk_font_chooser_get_font_map(self.as_ref().to_glib_none().0))
        }
    }

    fn get_font_size(&self) -> i32 {
        unsafe {
            ffi::gtk_font_chooser_get_font_size(self.as_ref().to_glib_none().0)
        }
    }

    fn get_preview_text(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::gtk_font_chooser_get_preview_text(self.as_ref().to_glib_none().0))
        }
    }

    fn get_show_preview_entry(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_font_chooser_get_show_preview_entry(self.as_ref().to_glib_none().0))
        }
    }

    fn set_filter_func<P: Fn(pango::FontFamily, pango::FontFace) -> bool + Send + Sync + 'static, Q: Into<Option<P>>>(&self, filter: Q) {
        let filter = filter.into();
        let filter_data: Box_<Option<P>> = Box::new(filter.into());
        unsafe extern "C" fn filter_func<P: Fn(pango::FontFamily, pango::FontFace) -> bool + Send + Sync + 'static>(family: *const pango_ffi::PangoFontFamily, face: *const pango_ffi::PangoFontFace, data: glib_ffi::gpointer) -> glib_ffi::gboolean {
            let family = from_glib_none(family);
            let face = from_glib_none(face);
            let callback: &Box_<Option<P>> = &*(data as *mut _);
            let res = if let Some(ref callback) = **callback {
                callback(family, face)
            } else {
                panic!("cannot get closure...")
            };
            res.to_glib()
        }
        let filter = if filter_data.is_some() { Some(filter_func::<P> as _) } else { None };
        unsafe extern "C" fn destroy_func<P: Fn(pango::FontFamily, pango::FontFace) -> bool + Send + Sync + 'static>(data: glib_ffi::gpointer) {
            let _callback: Box_<Option<P>> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<Option<P>> = filter_data;
        unsafe {
            ffi::gtk_font_chooser_set_filter_func(self.as_ref().to_glib_none().0, filter, Box::into_raw(super_callback0) as *mut _, destroy_call3);
        }
    }

    fn set_font(&self, fontname: &str) {
        unsafe {
            ffi::gtk_font_chooser_set_font(self.as_ref().to_glib_none().0, fontname.to_glib_none().0);
        }
    }

    fn set_font_desc(&self, font_desc: &pango::FontDescription) {
        unsafe {
            ffi::gtk_font_chooser_set_font_desc(self.as_ref().to_glib_none().0, font_desc.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn set_font_map<'a, P: IsA<pango::FontMap> + 'a, Q: Into<Option<&'a P>>>(&self, fontmap: Q) {
        let fontmap = fontmap.into();
        unsafe {
            ffi::gtk_font_chooser_set_font_map(self.as_ref().to_glib_none().0, fontmap.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_preview_text(&self, text: &str) {
        unsafe {
            ffi::gtk_font_chooser_set_preview_text(self.as_ref().to_glib_none().0, text.to_glib_none().0);
        }
    }

    fn set_show_preview_entry(&self, show_preview_entry: bool) {
        unsafe {
            ffi::gtk_font_chooser_set_show_preview_entry(self.as_ref().to_glib_none().0, show_preview_entry.to_glib());
        }
    }

    fn connect_font_activated<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"font-activated\0".as_ptr() as *const _,
                transmute(font_activated_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_font_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::font\0".as_ptr() as *const _,
                transmute(notify_font_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_font_desc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::font-desc\0".as_ptr() as *const _,
                transmute(notify_font_desc_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_preview_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::preview-text\0".as_ptr() as *const _,
                transmute(notify_preview_text_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_preview_entry_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::show-preview-entry\0".as_ptr() as *const _,
                transmute(notify_show_preview_entry_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn font_activated_trampoline<P>(this: *mut ffi::GtkFontChooser, fontname: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<FontChooser> {
    let f: &&(Fn(&P, &str) + 'static) = transmute(f);
    f(&FontChooser::from_glib_borrow(this).unsafe_cast(), &GString::from_glib_borrow(fontname))
}

unsafe extern "C" fn notify_font_trampoline<P>(this: *mut ffi::GtkFontChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FontChooser> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FontChooser::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_font_desc_trampoline<P>(this: *mut ffi::GtkFontChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FontChooser> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FontChooser::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_preview_text_trampoline<P>(this: *mut ffi::GtkFontChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FontChooser> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FontChooser::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_show_preview_entry_trampoline<P>(this: *mut ffi::GtkFontChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FontChooser> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FontChooser::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for FontChooser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FontChooser")
    }
}
