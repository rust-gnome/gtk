use Application;
use ffi;
use gio::ApplicationFlags;
use glib::object::IsA;
use glib::translate::*;
use rt;
use glib;
use glib_ffi;

impl Application {
    #[cfg(feature = "v3_6")]
    pub fn new(application_id: Option<&str>, flags: ApplicationFlags) -> Result<Application, glib::BoolError> {
        skip_assert_initialized!();
        try!(rt::init());
        unsafe {
            Option::from_glib_full(
                ffi::gtk_application_new(application_id.to_glib_none().0, flags.to_glib()))
                .ok_or(glib::BoolError("Failed to create application"))
        }
    }

    #[cfg(not(feature = "v3_6"))]
    pub fn new(application_id: &str, flags: ApplicationFlags) -> Result<Application, glib::BoolError> {
        skip_assert_initialized!();
        try!(rt::init());
        unsafe {
            Option::from_glib_full(
                ffi::gtk_application_new(application_id.to_glib_none().0, flags.to_glib()))
                .ok_or(glib::BoolError("Failed to create application"))
        }
    }
}
