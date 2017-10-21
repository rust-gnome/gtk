// This file was generated by gir (12a28ac) from gir-files (469db10)
// DO NOT EDIT

use Adjustment;
use ScrollablePolicy;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Scrollable(Object<ffi::GtkScrollable>);

    match fn {
        get_type => || ffi::gtk_scrollable_get_type(),
    }
}

pub trait ScrollableExt {
    //#[cfg(any(feature = "v3_16", feature = "dox"))]
    //fn get_border(&self, border: /*Ignored*/Border) -> bool;

    fn get_hadjustment(&self) -> Option<Adjustment>;

    fn get_hscroll_policy(&self) -> ScrollablePolicy;

    fn get_vadjustment(&self) -> Option<Adjustment>;

    fn get_vscroll_policy(&self) -> ScrollablePolicy;

    fn set_hadjustment<'a, P: Into<Option<&'a Adjustment>>>(&self, hadjustment: P);

    fn set_hscroll_policy(&self, policy: ScrollablePolicy);

    fn set_vadjustment<'a, P: Into<Option<&'a Adjustment>>>(&self, vadjustment: P);

    fn set_vscroll_policy(&self, policy: ScrollablePolicy);

    fn connect_property_hadjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_hscroll_policy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_vadjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_vscroll_policy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Scrollable> + IsA<glib::object::Object>> ScrollableExt for O {
    //#[cfg(any(feature = "v3_16", feature = "dox"))]
    //fn get_border(&self, border: /*Ignored*/Border) -> bool {
    //    unsafe { TODO: call ffi::gtk_scrollable_get_border() }
    //}

    fn get_hadjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_scrollable_get_hadjustment(self.to_glib_none().0))
        }
    }

    fn get_hscroll_policy(&self) -> ScrollablePolicy {
        unsafe {
            from_glib(ffi::gtk_scrollable_get_hscroll_policy(self.to_glib_none().0))
        }
    }

    fn get_vadjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_scrollable_get_vadjustment(self.to_glib_none().0))
        }
    }

    fn get_vscroll_policy(&self) -> ScrollablePolicy {
        unsafe {
            from_glib(ffi::gtk_scrollable_get_vscroll_policy(self.to_glib_none().0))
        }
    }

    fn set_hadjustment<'a, P: Into<Option<&'a Adjustment>>>(&self, hadjustment: P) {
        let hadjustment = hadjustment.into();
        let hadjustment = hadjustment.to_glib_none();
        unsafe {
            ffi::gtk_scrollable_set_hadjustment(self.to_glib_none().0, hadjustment.0);
        }
    }

    fn set_hscroll_policy(&self, policy: ScrollablePolicy) {
        unsafe {
            ffi::gtk_scrollable_set_hscroll_policy(self.to_glib_none().0, policy.to_glib());
        }
    }

    fn set_vadjustment<'a, P: Into<Option<&'a Adjustment>>>(&self, vadjustment: P) {
        let vadjustment = vadjustment.into();
        let vadjustment = vadjustment.to_glib_none();
        unsafe {
            ffi::gtk_scrollable_set_vadjustment(self.to_glib_none().0, vadjustment.0);
        }
    }

    fn set_vscroll_policy(&self, policy: ScrollablePolicy) {
        unsafe {
            ffi::gtk_scrollable_set_vscroll_policy(self.to_glib_none().0, policy.to_glib());
        }
    }

    fn connect_property_hadjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::hadjustment",
                transmute(notify_hadjustment_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_hscroll_policy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::hscroll-policy",
                transmute(notify_hscroll_policy_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_vadjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::vadjustment",
                transmute(notify_vadjustment_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_vscroll_policy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::vscroll-policy",
                transmute(notify_vscroll_policy_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_hadjustment_trampoline<P>(this: *mut ffi::GtkScrollable, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Scrollable> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Scrollable::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_hscroll_policy_trampoline<P>(this: *mut ffi::GtkScrollable, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Scrollable> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Scrollable::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_vadjustment_trampoline<P>(this: *mut ffi::GtkScrollable, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Scrollable> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Scrollable::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_vscroll_policy_trampoline<P>(this: *mut ffi::GtkScrollable, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Scrollable> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Scrollable::from_glib_borrow(this).downcast_unchecked())
}
