// This file was generated by gir (28183c8) from gir-files (71d73f0)
// DO NOT EDIT

use Actionable;
use Bin;
use Button;
use CheckButton;
use Container;
use ToggleButton;
use Widget;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct RadioButton(Object<ffi::GtkRadioButton>): CheckButton, ToggleButton, Button, Bin, Container, Widget, Actionable;

    match fn {
        get_type => || ffi::gtk_radio_button_get_type(),
    }
}

impl RadioButton {
    pub fn new(group: &[RadioButton]) -> RadioButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_button_new(group.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_from_widget<'a, P: Into<Option<&'a RadioButton>>>(radio_group_member: P) -> RadioButton {
        assert_initialized_main_thread!();
        let radio_group_member = radio_group_member.into();
        let radio_group_member = radio_group_member.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_button_new_from_widget(radio_group_member.0)).downcast_unchecked()
        }
    }

    pub fn new_with_label(group: &[RadioButton], label: &str) -> RadioButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_button_new_with_label(group.to_glib_none().0, label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_label_from_widget<'a, P: Into<Option<&'a RadioButton>>>(radio_group_member: P, label: &str) -> RadioButton {
        assert_initialized_main_thread!();
        let radio_group_member = radio_group_member.into();
        let radio_group_member = radio_group_member.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_button_new_with_label_from_widget(radio_group_member.0, label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_mnemonic(group: &[RadioButton], label: &str) -> RadioButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_button_new_with_mnemonic(group.to_glib_none().0, label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_mnemonic_from_widget<'a, P: Into<Option<&'a RadioButton>>>(radio_group_member: P, label: &str) -> RadioButton {
        assert_initialized_main_thread!();
        let radio_group_member = radio_group_member.into();
        let radio_group_member = radio_group_member.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_button_new_with_mnemonic_from_widget(radio_group_member.0, label.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait RadioButtonExt {
    fn get_group(&self) -> Vec<RadioButton>;

    fn join_group<'a, P: Into<Option<&'a RadioButton>>>(&self, group_source: P);

    fn set_group(&self, group: &[RadioButton]);

    fn connect_group_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<RadioButton> + IsA<glib::object::Object>> RadioButtonExt for O {
    fn get_group(&self) -> Vec<RadioButton> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_radio_button_get_group(self.to_glib_none().0))
        }
    }

    fn join_group<'a, P: Into<Option<&'a RadioButton>>>(&self, group_source: P) {
        let group_source = group_source.into();
        let group_source = group_source.to_glib_none();
        unsafe {
            ffi::gtk_radio_button_join_group(self.to_glib_none().0, group_source.0);
        }
    }

    fn set_group(&self, group: &[RadioButton]) {
        unsafe {
            ffi::gtk_radio_button_set_group(self.to_glib_none().0, group.to_glib_none().0);
        }
    }

    fn connect_group_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "group-changed",
                transmute(group_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn group_changed_trampoline<P>(this: *mut ffi::GtkRadioButton, f: glib_ffi::gpointer)
where P: IsA<RadioButton> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&RadioButton::from_glib_none(this).downcast_unchecked())
}
