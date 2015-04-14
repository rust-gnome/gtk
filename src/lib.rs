// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

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
extern crate c_vec;

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
pub use glib::traits::Connect;

// These are/should be inlined
pub use self::rt::Gtk;

/// GTK Widgets for all versions
pub use self::widgets::{
    Window, WindowBuilder,
    Label, LabelBuilder,
    Button, ButtonBuilder,
    Box, BoxBuilder,
    ButtonBox, ButtonBoxBuilder,
    Frame, FrameBuilder,
    AspectFrame, AspectFrameBuilder,
    Fixed, FixedBuilder,
    Separator, SeparatorBuilder,
    FontButton, FontButtonBuilder,
    ToggleButton, ToggleButtonBuilder,
    CheckButton, CheckButtonBuilder,
    ColorButton, ColorButtonBuilder,
    LinkButton, LinkButtonBuilder,
    Adjustment, AdjustmentBuilder,
    ScaleButton,
    VolumeButton, VolumeButtonBuilder,
    Grid, GridBuilder,
    EntryBuffer, EntryBufferBuilder,
    Entry, EntryBuilder,
    Switch, SwitchBuilder,
    Scale, ScaleBuilder,
    SpinButton, SpinButtonBuilder,
    Spinner, SpinnerBuilder,
    Image, ImageBuilder,
    ProgressBar, ProgressBarBuilder,
    Arrow, ArrowBuilder,
    Calendar, CalendarBuilder,
    Alignment, AlignmentBuilder,
    Expander, ExpanderBuilder,
    Paned, PanedBuilder,
    InfoBar, InfoBarBuilder,
    Toolbar, ToolbarBuilder,
    ToolItem, ToolItemBuilder,
    SeparatorToolItem, SeparatorToolItemBuilder,
    ToolButton, ToolButtonBuilder,
    ToggleToolButton, ToggleToolButtonBuilder,
    MenuToolButton, MenuToolButtonBuilder,
    Dialog, DialogBuilder,
    AboutDialog, AboutDialogBuilder,
    ColorChooserDialog,
    FontChooserDialog,
    MessageDialog, MessageDialogBuilder,
    NoteBook, NoteBookBuilder,
    Overlay, OverlayBuilder,
    Layout, LayoutBuilder,
    FileFilter,
    FileChooserDialog,
    AppInfo,
    AppLaunchContext,
    AppChooserDialog,
    DrawingArea, DrawingAreaBuilder,
    PageSetup, PageSetupBuilder,
    PaperSize, PaperSizeBuilder,
    PrintSettings, PrintSettingsBuilder,
    RecentChooserDialog,
    //PageSetupUnixDialog
    RecentInfo,
    RecentFilter,
    RecentFilterInfo,
    RecentData,
    RecentManager,
    TextView, TextViewBuilder,
    TextBuffer, TextBufferBuilder,
    TextTagTable, TextTagTableBuilder,
    ScrolledWindow, ScrolledWindowBuilder,
    RadioButton, RadioButtonBuilder,
    TreeView, TreeViewBuilder,
    TreeViewColumn, TreeViewColumnBuilder,
    TreePath, TreePathBuilder,
    TreeIter, TreeIterBuilder,
    TreeModel,
    ListStore, ListStoreBuilder,
    TreeStore, TreeStoreBuilder,
    MenuItem, MenuItemBuilder,
    SeparatorMenuItem, SeparatorMenuItemBuilder,
    CheckMenuItem,
    ScrollBar, ScrollBarBuilder,
    Viewport, ViewportBuilder,
    StatusBar, StatusBarBuilder,
    CellRendererText,
    CellRendererToggle,
    LockButton, LockButtonBuilder,
    EntryCompletion, EntryCompletionBuilder,
    IconView, IconViewBuilder,
    TreeSelection,
    RecentChooserWidget,
    ComboBox, ComboBoxBuilder,
    //g_type,
    ComboBoxText,
    TextMark, TextMarkBuilder,
    TextTag, TextTagBuilder,
    TextAttributes, TextAttributesBuilder,
    TextIter, TextIterBuilder,
    TextChildAnchor, TextChildAnchorBuilder,
    ToolPalette, ToolPaletteBuilder,
    ToolItemGroup, ToolItemGroupBuilder,
    SizeGroup, SizeGroupBuilder,
    AppChooserWidget,
    FileChooserWidget,
    ColorChooserWidget,
    FontChooserWidget,
    EventBox, EventBoxBuilder,
};

#[cfg(target_os = "linux")]
pub use self::widgets::{Socket, SocketBuilder};

#[cfg(feature = "gtk_3_6")]
/// GTK Widgets for versions since GTK 3.6
pub use self::widgets::{
    MenuButton, MenuButtonBuilder,
    LevelBar, LevelBarBuilder,
};

#[cfg(feature = "gtk_3_10")]
/// GTK Widgets for versions since GTK 3.10
pub use self::widgets::{
    SearchEntry, SearchEntryBuilder,
    SearchBar, SearchBarBuilder,
    Stack, StackBuilder,
    StackSwitcher, StackSwitcherBuilder,
    Revealer, RevealerBuilder,
    HeaderBar, HeaderBarBuilder,
    ListBox, ListBoxBuilder,
    ListBoxRow,
    PlacesSidebar
};

#[cfg(feature = "gtk_3_12")]
/// GTK Widgets for versions since GTK 3.12
pub use self::widgets::{
    FlowBox, FlowBoxBuilder,
    FlowBoxChild,
    ActionBar, ActionBarBuilder,
    Popover, PopoverBuilder,
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
pub use ffi::enums::GConnectFlags;
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

/// Gtk Traits
pub use self::traits::FFIWidget;
pub use self::traits::GObjectTrait;
pub use self::traits::BoxTrait;
pub use self::traits::ActionableTrait;
pub use self::traits::AppChooserTrait;
pub use self::traits::BinTrait;
pub use self::traits::ButtonTrait;
pub use self::traits::CellEditableTrait;
pub use self::traits::CellLayoutTrait;
pub use self::traits::CellRendererTrait;
pub use self::traits::CheckMenuItemTrait;
pub use self::traits::ColorChooserTrait;
pub use self::traits::ComboBoxTrait;
pub use self::traits::ContainerTrait;
pub use self::traits::DialogButtons;
pub use self::traits::DialogTrait;
pub use self::traits::EditableTrait;
pub use self::traits::EntryTrait;
pub use self::traits::FileChooserTrait;
pub use self::traits::FontChooserTrait;
pub use self::traits::FrameTrait;
pub use self::traits::LabelTrait;
pub use self::traits::MenuItemTrait;
pub use self::traits::MenuShellTrait;
pub use self::traits::MiscTrait;
pub use self::traits::OrientableTrait;
pub use self::traits::RangeTrait;
pub use self::traits::RecentChooserTrait;
pub use self::traits::ScaleButtonTrait;
pub use self::traits::ScrollableTrait;
pub use self::traits::ScrolledWindowTrait;
pub use self::traits::TextBufferTrait;
pub use self::traits::ToggleButtonTrait;
pub use self::traits::ToggleToolButtonTrait;
pub use self::traits::ToolButtonTrait;
pub use self::traits::ToolItemTrait;
pub use self::traits::ToolShellTrait;
pub use self::traits::WidgetTrait;
pub use self::traits::WindowTrait;

/// GTK various struct
pub use self::types::{
    Tooltip,
};

mod macros;
mod cast;
mod rt;

pub mod traits;
pub mod signals;
pub mod widgets;
pub mod types;

