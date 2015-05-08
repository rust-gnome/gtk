// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

pub trait FFIWidget: Sized {
    fn unwrap_widget(&self) -> *mut ::ffi::C_GtkWidget;
    fn wrap_widget(widget: *mut ::ffi::C_GtkWidget) -> Self;
}

pub use self::widget::WidgetTrait;
pub use self::container::ContainerTrait;
pub use self::window::WindowTrait;
pub use self::button::ButtonTrait;

/*
pub use signal::{
    WidgetSignals,
    ButtonSignals,
    ToolButtonSignals,
};
*/

pub mod widget;
pub mod container;
pub mod window;
pub mod button;
