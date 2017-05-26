// This file was generated by gir (28183c8) from gir-files (71d73f0)
// DO NOT EDIT

use ffi;
use glib::translate::*;

bitflags! {
    pub flags AccelFlags: u32 {
        const ACCEL_VISIBLE = 1,
        const ACCEL_LOCKED = 2,
        const ACCEL_MASK = 7,
    }
}

#[doc(hidden)]
impl ToGlib for AccelFlags {
    type GlibType = ffi::GtkAccelFlags;

    fn to_glib(&self) -> ffi::GtkAccelFlags {
        ffi::GtkAccelFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkAccelFlags> for AccelFlags {
    fn from_glib(value: ffi::GtkAccelFlags) -> AccelFlags {
        skip_assert_initialized!();
        AccelFlags::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub flags ApplicationInhibitFlags: u32 {
        const APPLICATION_INHIBIT_LOGOUT = 1,
        const APPLICATION_INHIBIT_SWITCH = 2,
        const APPLICATION_INHIBIT_SUSPEND = 4,
        const APPLICATION_INHIBIT_IDLE = 8,
    }
}

#[doc(hidden)]
impl ToGlib for ApplicationInhibitFlags {
    type GlibType = ffi::GtkApplicationInhibitFlags;

    fn to_glib(&self) -> ffi::GtkApplicationInhibitFlags {
        ffi::GtkApplicationInhibitFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkApplicationInhibitFlags> for ApplicationInhibitFlags {
    fn from_glib(value: ffi::GtkApplicationInhibitFlags) -> ApplicationInhibitFlags {
        skip_assert_initialized!();
        ApplicationInhibitFlags::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub flags CalendarDisplayOptions: u32 {
        const CALENDAR_SHOW_HEADING = 1,
        const CALENDAR_SHOW_DAY_NAMES = 2,
        const CALENDAR_NO_MONTH_CHANGE = 4,
        const CALENDAR_SHOW_WEEK_NUMBERS = 8,
        const CALENDAR_SHOW_DETAILS = 32,
    }
}

#[doc(hidden)]
impl ToGlib for CalendarDisplayOptions {
    type GlibType = ffi::GtkCalendarDisplayOptions;

    fn to_glib(&self) -> ffi::GtkCalendarDisplayOptions {
        ffi::GtkCalendarDisplayOptions::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkCalendarDisplayOptions> for CalendarDisplayOptions {
    fn from_glib(value: ffi::GtkCalendarDisplayOptions) -> CalendarDisplayOptions {
        skip_assert_initialized!();
        CalendarDisplayOptions::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub flags CellRendererState: u32 {
        const CELL_RENDERER_SELECTED = 1,
        const CELL_RENDERER_PRELIT = 2,
        const CELL_RENDERER_INSENSITIVE = 4,
        const CELL_RENDERER_SORTED = 8,
        const CELL_RENDERER_FOCUSED = 16,
        const CELL_RENDERER_EXPANDABLE = 32,
        const CELL_RENDERER_EXPANDED = 64,
    }
}

#[doc(hidden)]
impl ToGlib for CellRendererState {
    type GlibType = ffi::GtkCellRendererState;

    fn to_glib(&self) -> ffi::GtkCellRendererState {
        ffi::GtkCellRendererState::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkCellRendererState> for CellRendererState {
    fn from_glib(value: ffi::GtkCellRendererState) -> CellRendererState {
        skip_assert_initialized!();
        CellRendererState::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub flags DestDefaults: u32 {
        const DEST_DEFAULT_MOTION = 1,
        const DEST_DEFAULT_HIGHLIGHT = 2,
        const DEST_DEFAULT_DROP = 4,
        const DEST_DEFAULT_ALL = 7,
    }
}

#[doc(hidden)]
impl ToGlib for DestDefaults {
    type GlibType = ffi::GtkDestDefaults;

    fn to_glib(&self) -> ffi::GtkDestDefaults {
        ffi::GtkDestDefaults::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkDestDefaults> for DestDefaults {
    fn from_glib(value: ffi::GtkDestDefaults) -> DestDefaults {
        skip_assert_initialized!();
        DestDefaults::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub flags DialogFlags: u32 {
        const DIALOG_MODAL = 1,
        const DIALOG_DESTROY_WITH_PARENT = 2,
        const DIALOG_USE_HEADER_BAR = 4,
    }
}

#[doc(hidden)]
impl ToGlib for DialogFlags {
    type GlibType = ffi::GtkDialogFlags;

    fn to_glib(&self) -> ffi::GtkDialogFlags {
        ffi::GtkDialogFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkDialogFlags> for DialogFlags {
    fn from_glib(value: ffi::GtkDialogFlags) -> DialogFlags {
        skip_assert_initialized!();
        DialogFlags::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub flags FileFilterFlags: u32 {
        const FILE_FILTER_FILENAME = 1,
        const FILE_FILTER_URI = 2,
        const FILE_FILTER_DISPLAY_NAME = 4,
        const FILE_FILTER_MIME_TYPE = 8,
    }
}

#[doc(hidden)]
impl ToGlib for FileFilterFlags {
    type GlibType = ffi::GtkFileFilterFlags;

    fn to_glib(&self) -> ffi::GtkFileFilterFlags {
        ffi::GtkFileFilterFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkFileFilterFlags> for FileFilterFlags {
    fn from_glib(value: ffi::GtkFileFilterFlags) -> FileFilterFlags {
        skip_assert_initialized!();
        FileFilterFlags::from_bits_truncate(value.bits())
    }
}

#[cfg(feature = "v3_6")]
bitflags! {
    pub flags InputHints: u32 {
        const INPUT_HINT_NONE = 0,
        const INPUT_HINT_SPELLCHECK = 1,
        const INPUT_HINT_NO_SPELLCHECK = 2,
        const INPUT_HINT_WORD_COMPLETION = 4,
        const INPUT_HINT_LOWERCASE = 8,
        const INPUT_HINT_UPPERCASE_CHARS = 16,
        const INPUT_HINT_UPPERCASE_WORDS = 32,
        const INPUT_HINT_UPPERCASE_SENTENCES = 64,
        const INPUT_HINT_INHIBIT_OSK = 128,
        const INPUT_HINT_VERTICAL_WRITING = 256,
    }
}

#[cfg(feature = "v3_6")]
#[doc(hidden)]
impl ToGlib for InputHints {
    type GlibType = ffi::GtkInputHints;

    fn to_glib(&self) -> ffi::GtkInputHints {
        ffi::GtkInputHints::from_bits_truncate(self.bits())
    }
}

#[cfg(feature = "v3_6")]
#[doc(hidden)]
impl FromGlib<ffi::GtkInputHints> for InputHints {
    fn from_glib(value: ffi::GtkInputHints) -> InputHints {
        skip_assert_initialized!();
        InputHints::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub flags JunctionSides: u32 {
        const JUNCTION_NONE = 0,
        const JUNCTION_CORNER_TOPLEFT = 1,
        const JUNCTION_CORNER_TOPRIGHT = 2,
        const JUNCTION_CORNER_BOTTOMLEFT = 4,
        const JUNCTION_CORNER_BOTTOMRIGHT = 8,
        const JUNCTION_TOP = 3,
        const JUNCTION_BOTTOM = 12,
        const JUNCTION_LEFT = 5,
        const JUNCTION_RIGHT = 10,
    }
}

#[doc(hidden)]
impl ToGlib for JunctionSides {
    type GlibType = ffi::GtkJunctionSides;

    fn to_glib(&self) -> ffi::GtkJunctionSides {
        ffi::GtkJunctionSides::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkJunctionSides> for JunctionSides {
    fn from_glib(value: ffi::GtkJunctionSides) -> JunctionSides {
        skip_assert_initialized!();
        JunctionSides::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub flags PlacesOpenFlags: u32 {
        const PLACES_OPEN_NORMAL = 1,
        const PLACES_OPEN_NEW_TAB = 2,
        const PLACES_OPEN_NEW_WINDOW = 4,
    }
}

#[doc(hidden)]
impl ToGlib for PlacesOpenFlags {
    type GlibType = ffi::GtkPlacesOpenFlags;

    fn to_glib(&self) -> ffi::GtkPlacesOpenFlags {
        ffi::GtkPlacesOpenFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkPlacesOpenFlags> for PlacesOpenFlags {
    fn from_glib(value: ffi::GtkPlacesOpenFlags) -> PlacesOpenFlags {
        skip_assert_initialized!();
        PlacesOpenFlags::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub flags RecentFilterFlags: u32 {
        const RECENT_FILTER_URI = 1,
        const RECENT_FILTER_DISPLAY_NAME = 2,
        const RECENT_FILTER_MIME_TYPE = 4,
        const RECENT_FILTER_APPLICATION = 8,
        const RECENT_FILTER_GROUP = 16,
        const RECENT_FILTER_AGE = 32,
    }
}

#[doc(hidden)]
impl ToGlib for RecentFilterFlags {
    type GlibType = ffi::GtkRecentFilterFlags;

    fn to_glib(&self) -> ffi::GtkRecentFilterFlags {
        ffi::GtkRecentFilterFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkRecentFilterFlags> for RecentFilterFlags {
    fn from_glib(value: ffi::GtkRecentFilterFlags) -> RecentFilterFlags {
        skip_assert_initialized!();
        RecentFilterFlags::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub flags RegionFlags: u32 {
        const REGION_EVEN = 1,
        const REGION_ODD = 2,
        const REGION_FIRST = 4,
        const REGION_LAST = 8,
        const REGION_ONLY = 16,
        const REGION_SORTED = 32,
    }
}

#[doc(hidden)]
impl ToGlib for RegionFlags {
    type GlibType = ffi::GtkRegionFlags;

    fn to_glib(&self) -> ffi::GtkRegionFlags {
        ffi::GtkRegionFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkRegionFlags> for RegionFlags {
    fn from_glib(value: ffi::GtkRegionFlags) -> RegionFlags {
        skip_assert_initialized!();
        RegionFlags::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub flags StateFlags: u32 {
        const STATE_FLAG_NORMAL = 0,
        const STATE_FLAG_ACTIVE = 1,
        const STATE_FLAG_PRELIGHT = 2,
        const STATE_FLAG_SELECTED = 4,
        const STATE_FLAG_INSENSITIVE = 8,
        const STATE_FLAG_INCONSISTENT = 16,
        const STATE_FLAG_FOCUSED = 32,
        const STATE_FLAG_BACKDROP = 64,
        const STATE_FLAG_DIR_LTR = 128,
        const STATE_FLAG_DIR_RTL = 256,
        const STATE_FLAG_LINK = 512,
        const STATE_FLAG_VISITED = 1024,
        const STATE_FLAG_CHECKED = 2048,
        const STATE_FLAG_DROP_ACTIVE = 4096,
    }
}

#[doc(hidden)]
impl ToGlib for StateFlags {
    type GlibType = ffi::GtkStateFlags;

    fn to_glib(&self) -> ffi::GtkStateFlags {
        ffi::GtkStateFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkStateFlags> for StateFlags {
    fn from_glib(value: ffi::GtkStateFlags) -> StateFlags {
        skip_assert_initialized!();
        StateFlags::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub flags StyleContextPrintFlags: u32 {
        const STYLE_CONTEXT_PRINT_NONE = 0,
        const STYLE_CONTEXT_PRINT_RECURSE = 1,
        const STYLE_CONTEXT_PRINT_SHOW_STYLE = 2,
    }
}

#[doc(hidden)]
impl ToGlib for StyleContextPrintFlags {
    type GlibType = ffi::GtkStyleContextPrintFlags;

    fn to_glib(&self) -> ffi::GtkStyleContextPrintFlags {
        ffi::GtkStyleContextPrintFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkStyleContextPrintFlags> for StyleContextPrintFlags {
    fn from_glib(value: ffi::GtkStyleContextPrintFlags) -> StyleContextPrintFlags {
        skip_assert_initialized!();
        StyleContextPrintFlags::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub flags TargetFlags: u32 {
        const TARGET_SAME_APP = 1,
        const TARGET_SAME_WIDGET = 2,
        const TARGET_OTHER_APP = 4,
        const TARGET_OTHER_WIDGET = 8,
    }
}

#[doc(hidden)]
impl ToGlib for TargetFlags {
    type GlibType = ffi::GtkTargetFlags;

    fn to_glib(&self) -> ffi::GtkTargetFlags {
        ffi::GtkTargetFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkTargetFlags> for TargetFlags {
    fn from_glib(value: ffi::GtkTargetFlags) -> TargetFlags {
        skip_assert_initialized!();
        TargetFlags::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub flags TextSearchFlags: u32 {
        const TEXT_SEARCH_VISIBLE_ONLY = 1,
        const TEXT_SEARCH_TEXT_ONLY = 2,
        const TEXT_SEARCH_CASE_INSENSITIVE = 4,
    }
}

#[doc(hidden)]
impl ToGlib for TextSearchFlags {
    type GlibType = ffi::GtkTextSearchFlags;

    fn to_glib(&self) -> ffi::GtkTextSearchFlags {
        ffi::GtkTextSearchFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkTextSearchFlags> for TextSearchFlags {
    fn from_glib(value: ffi::GtkTextSearchFlags) -> TextSearchFlags {
        skip_assert_initialized!();
        TextSearchFlags::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub flags ToolPaletteDragTargets: u32 {
        const TOOL_PALETTE_DRAG_ITEMS = 1,
        const TOOL_PALETTE_DRAG_GROUPS = 2,
    }
}

#[doc(hidden)]
impl ToGlib for ToolPaletteDragTargets {
    type GlibType = ffi::GtkToolPaletteDragTargets;

    fn to_glib(&self) -> ffi::GtkToolPaletteDragTargets {
        ffi::GtkToolPaletteDragTargets::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkToolPaletteDragTargets> for ToolPaletteDragTargets {
    fn from_glib(value: ffi::GtkToolPaletteDragTargets) -> ToolPaletteDragTargets {
        skip_assert_initialized!();
        ToolPaletteDragTargets::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub flags TreeModelFlags: u32 {
        const TREE_MODEL_ITERS_PERSIST = 1,
        const TREE_MODEL_LIST_ONLY = 2,
    }
}

#[doc(hidden)]
impl ToGlib for TreeModelFlags {
    type GlibType = ffi::GtkTreeModelFlags;

    fn to_glib(&self) -> ffi::GtkTreeModelFlags {
        ffi::GtkTreeModelFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkTreeModelFlags> for TreeModelFlags {
    fn from_glib(value: ffi::GtkTreeModelFlags) -> TreeModelFlags {
        skip_assert_initialized!();
        TreeModelFlags::from_bits_truncate(value.bits())
    }
}

