// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

/*!

Bindings and wrappers for __GTK__

To implement __GTK+__ inheritance in rust, we implemented gtk superclasses as traits
located in `rgtk::self::traits::*`. The various widgets implement these traits and
live in `rgtk::gtk::widgets::*` and are rexported into `rgtk::gtk::*`.

GTK Inheritance in rgtk
======================

You probably know but __Gtk+__ uses its own GObject system: inherited class and interface.

To respect this design I follow a special design on __rgtk__:

* Interface -> Implement them on a trait with only default methods.
* Class -> Implement the construct on the class impl and other methods on a traits.
* Sub-class -> Implement all the methods on the class.

Exemple for GtkOrientable, GtkBox, GtkButtonBox:

GtkOrientable is an interface with all the methods implemented as default method of the trait self::traits::Orientable.

GtkBox is a class with constructors implemented on the struct `gtk::Box`, and the other method as default methods of the trait `self::traits::Box`. So `gtk::Box` implements `self::traits::Orientable` and `self::traits::Box`.

GtkButtonBox is a sub-class of GtkBox, the struct `gtk::ButtonBox` implements all the methods of GtkButtonBox and the traits `self::traits::Orientable` and `self::traits::Box`.

Finally all the gtk widgets implement the trait self::traits::Widget.
*/

//#![macro_use]

#![allow(dead_code)] // TODO: drop this
#![allow(raw_pointer_derive)]

extern crate libc;

extern crate glib_sys as glib_ffi;
extern crate gdk_sys as gdk_ffi;
extern crate gtk_sys as ffi;
extern crate cairo_sys as cairo_ffi;
extern crate pango_sys as pango_ffi;
extern crate glib;
extern crate gdk;
extern crate cairo;
extern crate pango;

pub use glib::ValuePublic;

// These are/should be inlined
pub use self::rt::{
    init,
    main,
    main_quit,
    main_level,
    main_iteration,
    main_iteration_do,
    get_major_version,
    get_minor_version,
    get_micro_version,
    get_binary_age,
    get_interface_age,
    check_version,
    events_pending
};


/// GTK Enum types
pub use ffi::enums::WindowType;
pub use ffi::enums::TextDirection;
pub use ffi::enums::WindowPosition;
pub use ffi::enums::ButtonBoxStyle;
pub use ffi::enums::Orientation;
pub use ffi::enums::DirectionType;
pub use ffi::enums::CornerType;
pub use ffi::enums::ResizeMode;
pub use ffi::enums::BorderStyle;
pub use ffi::enums::SortType;
pub use ffi::enums::StateFlags;
pub use ffi::enums::DragResult;
pub use ffi::enums::AccelFlags;
pub use ffi::enums::ArrowPlacement;
pub use ffi::enums::ArrowType;
pub use ffi::enums::AttachOptions;
pub use ffi::enums::DeleteType;
pub use ffi::enums::ExpanderStyle;
pub use ffi::enums::IMPreeditStyle;
pub use ffi::enums::IMStatusStyle;
pub use ffi::enums::Justification;
pub use ffi::enums::MovementStep;
pub use ffi::enums::PackType;
pub use ffi::enums::PathPriorityType;
pub use ffi::enums::PathType;
pub use ffi::enums::PolicyType;
pub use ffi::enums::PositionType;
pub use ffi::enums::ReliefStyle;
pub use ffi::enums::ScrollStep;
pub use ffi::enums::ScrollType;
pub use ffi::enums::SelectionMode;
pub use ffi::enums::ShadowType;
pub use ffi::enums::StateType;
pub use ffi::enums::ToolbarStyle;
pub use ffi::enums::JunctionSides;
pub use ffi::enums::RegionFlags;
pub use ffi::enums::IconSize;
pub use ffi::enums::EntryIconPosition;
pub use ffi::enums::InputHints;
pub use ffi::enums::InputPurpose;
pub use ffi::enums::ImageType;
pub use ffi::enums::SpinType;
pub use ffi::enums::SpinButtonUpdatePolicy;
pub use ffi::enums::LevelBarMode;
pub use ffi::enums::CalendarDisplayOptions;
pub use ffi::enums::MessageType;
pub use ffi::enums::License;
pub use ffi::enums::ResponseType;
pub use ffi::enums::DialogFlags;
pub use ffi::enums::FileChooserAction;
pub use ffi::enums::ButtonsType;
pub use ffi::enums::StackTransitionType;
pub use ffi::enums::RevealerTransitionType;
pub use ffi::enums::ScrollablePolicy;
pub use ffi::enums::FileFilterFlags;
pub use ffi::enums::AppInfoCreateFlags;
pub use ffi::enums::SizeRequestMode;
pub use ffi::enums::Align;
pub use ffi::enums::BuilderError;
pub use ffi::enums::PageOrientation;
pub use ffi::enums::Unit;
pub use ffi::enums::NumberUpLayout;
pub use ffi::enums::PrintPages;
pub use ffi::enums::PageSet;
pub use ffi::enums::RecentSortType;
pub use ffi::enums::RecentFilterFlags;
pub use ffi::enums::WidgetHelpType;
pub use ffi::enums::TextWindowType;
pub use ffi::enums::WrapMode;
pub use ffi::enums::TreeViewGridLines;
pub use ffi::enums::TreeViewColumnSizing;
pub use ffi::enums::CellRendererState;
pub use ffi::enums::TreeModelFlags;
pub use ffi::enums::IconViewDropPosition;
pub use ffi::enums::SensitivityType;
pub use ffi::enums::GType;
pub use ffi::enums::TextSearchFlags;
pub use ffi::enums::PlacesOpenFlags;
pub use ffi::enums::ToolPaletteDragTargets;
pub use ffi::enums::DestDefaults;
pub use ffi::enums::SizeGroupMode;

mod macros;
mod cast;
mod rt;

mod types;

pub mod prelude;

pub mod object;
pub mod widget;
pub mod container;
pub mod button;
pub mod window;

//pub mod traits;
pub mod signal;
//pub mod widgets;
//pub mod types;
