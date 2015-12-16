// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use WindowType;

/**
* Window — Toplevel which can contain other widgets
*
* # Available signals:
* * `activate-default` : Action
* * `activate-focus` : Action
* * `keys-changed` : Run First
* * `set-focus` : Run Last
*/
struct_Widget!(Window);

/// Go to the  GTK website for complete
/// [documentation](https://developer.gnome.org/gtk3/stable/GtkWindow.html).
///
/// # Examples
///
/// ```
/// use gtk::traits::*;
///
/// gtk::init();
/// let window = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();
/// window.set_window_position(gtk::WindowPosition::Center);
/// window.show_all();
/// ```
///
impl Window {
    pub fn new(window_type: WindowType) -> Option<Window> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_window_new(window_type) };
        check_pointer!(tmp_pointer, Window)
    }
}

impl_drop!(Window);
impl_TraitWidget!(Window);

impl ::ContainerTrait for Window {}
impl ::WindowTrait for Window {}
impl ::BinTrait for Window {}
