use glib::object::{Upcast, VirtualRef, Wrapper};
use glib::translate::from_glib;
use glib::type_::{StaticType, Type};
use object::Object;
use ffi;
use glib_ffi;

macro_rules! gtype {
    ($ty:ident, $func:ident) => (
        impl StaticType for Object<ffi::$ty> {
            fn static_type() -> Type { unsafe { from_glib(ffi::$func()) } }
        }
    )
}

gtype!(C_GtkWidget, gtk_widget_get_type);
gtype!(C_GtkContainer, gtk_container_get_type);
//gtype!(C_GtkBin, gtk_bin_get_type);
//gtype!(C_GtkBox, gtk_box_get_type);
gtype!(C_GtkButton, gtk_button_get_type);
//gtype!(C_GtkToggleButton, gtk_toggle_button_get_type);
//gtype!(C_GtkCheckButton, gtk_check_button_get_type);
gtype!(C_GtkWindow, gtk_window_get_type);

macro_rules! hier {
    ($sub:ident, $sup:ident) => (
        unsafe impl Upcast<Object<ffi::$sup>> for Object<ffi::$sub> {
            fn virt(&self) -> VirtualRef<ffi::$sup> { VirtualRef::new(self.as_ref()) }
        }
    );
    ($sub:ident, $sup:ident, $($supsup:ident),+) => (
        hier!($sub, $sup);
        $(
            hier!($sub, $supsup);
        )+
    )
}

unsafe impl<T> Upcast<::glib::object::Object> for Object<T> where Object<T>: StaticType {
    fn virt(&self) -> VirtualRef<glib_ffi::C_GObject> { VirtualRef::new(&self.as_ref()) }
}

hier!(C_GtkContainer, C_GtkWidget);
hier!(C_GtkWindow, C_GtkContainer, C_GtkWidget);
hier!(C_GtkButton, C_GtkContainer, C_GtkWidget);
