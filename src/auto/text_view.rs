// This file was generated by gir (5232053) from gir-files (71d73f0)
// DO NOT EDIT

use Container;
use DeleteType;
#[cfg(feature = "v3_6")]
use InputHints;
#[cfg(feature = "v3_6")]
use InputPurpose;
use Justification;
use MovementStep;
use ScrollStep;
use Scrollable;
use TextAttributes;
use TextBuffer;
use TextChildAnchor;
#[cfg(feature = "v3_16")]
use TextExtendSelection;
use TextIter;
use TextMark;
use TextWindowType;
use Widget;
use WrapMode;
use ffi;
use gdk;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
#[cfg(feature = "v3_16")]
use signal::Inhibit;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;

glib_wrapper! {
    pub struct TextView(Object<ffi::GtkTextView>): Container, Widget, Scrollable;

    match fn {
        get_type => || ffi::gtk_text_view_get_type(),
    }
}

impl TextView {
    pub fn new() -> TextView {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_text_view_new()).downcast_unchecked()
        }
    }

    pub fn new_with_buffer(buffer: &TextBuffer) -> TextView {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_text_view_new_with_buffer(buffer.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn add_child_at_anchor<T: IsA<Widget>>(&self, child: &T, anchor: &TextChildAnchor) {
        unsafe {
            ffi::gtk_text_view_add_child_at_anchor(self.to_glib_none().0, child.to_glib_none().0, anchor.to_glib_none().0);
        }
    }

    pub fn add_child_in_window<T: IsA<Widget>>(&self, child: &T, which_window: TextWindowType, xpos: i32, ypos: i32) {
        unsafe {
            ffi::gtk_text_view_add_child_in_window(self.to_glib_none().0, child.to_glib_none().0, which_window.to_glib(), xpos, ypos);
        }
    }

    pub fn backward_display_line(&self, iter: &mut TextIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_view_backward_display_line(self.to_glib_none().0, iter.to_glib_none_mut().0))
        }
    }

    pub fn backward_display_line_start(&self, iter: &mut TextIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_view_backward_display_line_start(self.to_glib_none().0, iter.to_glib_none_mut().0))
        }
    }

    pub fn buffer_to_window_coords(&self, win: TextWindowType, buffer_x: i32, buffer_y: i32) -> (i32, i32) {
        unsafe {
            let mut window_x = mem::uninitialized();
            let mut window_y = mem::uninitialized();
            ffi::gtk_text_view_buffer_to_window_coords(self.to_glib_none().0, win.to_glib(), buffer_x, buffer_y, &mut window_x, &mut window_y);
            (window_x, window_y)
        }
    }

    pub fn forward_display_line(&self, iter: &mut TextIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_view_forward_display_line(self.to_glib_none().0, iter.to_glib_none_mut().0))
        }
    }

    pub fn forward_display_line_end(&self, iter: &mut TextIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_view_forward_display_line_end(self.to_glib_none().0, iter.to_glib_none_mut().0))
        }
    }

    pub fn get_accepts_tab(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_view_get_accepts_tab(self.to_glib_none().0))
        }
    }

    pub fn get_border_window_size(&self, type_: TextWindowType) -> i32 {
        unsafe {
            ffi::gtk_text_view_get_border_window_size(self.to_glib_none().0, type_.to_glib())
        }
    }

    #[cfg(feature = "v3_18")]
    pub fn get_bottom_margin(&self) -> i32 {
        unsafe {
            ffi::gtk_text_view_get_bottom_margin(self.to_glib_none().0)
        }
    }

    pub fn get_buffer(&self) -> Option<TextBuffer> {
        unsafe {
            from_glib_none(ffi::gtk_text_view_get_buffer(self.to_glib_none().0))
        }
    }

    pub fn get_cursor_locations(&self, iter: Option<&TextIter>) -> (gdk::Rectangle, gdk::Rectangle) {
        unsafe {
            let mut strong = gdk::Rectangle::uninitialized();
            let mut weak = gdk::Rectangle::uninitialized();
            ffi::gtk_text_view_get_cursor_locations(self.to_glib_none().0, iter.to_glib_none().0, strong.to_glib_none_mut().0, weak.to_glib_none_mut().0);
            (strong, weak)
        }
    }

    pub fn get_cursor_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_view_get_cursor_visible(self.to_glib_none().0))
        }
    }

    pub fn get_default_attributes(&self) -> TextAttributes {
        unsafe {
            from_glib_full(ffi::gtk_text_view_get_default_attributes(self.to_glib_none().0))
        }
    }

    pub fn get_editable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_view_get_editable(self.to_glib_none().0))
        }
    }

    pub fn get_indent(&self) -> i32 {
        unsafe {
            ffi::gtk_text_view_get_indent(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn get_input_hints(&self) -> InputHints {
        unsafe {
            from_glib(ffi::gtk_text_view_get_input_hints(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn get_input_purpose(&self) -> InputPurpose {
        unsafe {
            from_glib(ffi::gtk_text_view_get_input_purpose(self.to_glib_none().0))
        }
    }

    pub fn get_iter_at_location(&self, x: i32, y: i32) -> Option<TextIter> {
        unsafe {
            let mut iter = TextIter::uninitialized();
            let ret = from_glib(ffi::gtk_text_view_get_iter_at_location(self.to_glib_none().0, iter.to_glib_none_mut().0, x, y));
            if ret { Some(iter) } else { None }
        }
    }

    pub fn get_iter_at_position(&self, x: i32, y: i32) -> Option<(TextIter, i32)> {
        unsafe {
            let mut iter = TextIter::uninitialized();
            let mut trailing = mem::uninitialized();
            let ret = from_glib(ffi::gtk_text_view_get_iter_at_position(self.to_glib_none().0, iter.to_glib_none_mut().0, &mut trailing, x, y));
            if ret { Some((iter, trailing)) } else { None }
        }
    }

    pub fn get_iter_location(&self, iter: &TextIter) -> gdk::Rectangle {
        unsafe {
            let mut location = gdk::Rectangle::uninitialized();
            ffi::gtk_text_view_get_iter_location(self.to_glib_none().0, iter.to_glib_none().0, location.to_glib_none_mut().0);
            location
        }
    }

    pub fn get_justification(&self) -> Justification {
        unsafe {
            from_glib(ffi::gtk_text_view_get_justification(self.to_glib_none().0))
        }
    }

    pub fn get_left_margin(&self) -> i32 {
        unsafe {
            ffi::gtk_text_view_get_left_margin(self.to_glib_none().0)
        }
    }

    pub fn get_line_at_y(&self, y: i32) -> (TextIter, i32) {
        unsafe {
            let mut target_iter = TextIter::uninitialized();
            let mut line_top = mem::uninitialized();
            ffi::gtk_text_view_get_line_at_y(self.to_glib_none().0, target_iter.to_glib_none_mut().0, y, &mut line_top);
            (target_iter, line_top)
        }
    }

    pub fn get_line_yrange(&self, iter: &TextIter) -> (i32, i32) {
        unsafe {
            let mut y = mem::uninitialized();
            let mut height = mem::uninitialized();
            ffi::gtk_text_view_get_line_yrange(self.to_glib_none().0, iter.to_glib_none().0, &mut y, &mut height);
            (y, height)
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_monospace(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_view_get_monospace(self.to_glib_none().0))
        }
    }

    pub fn get_overwrite(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_view_get_overwrite(self.to_glib_none().0))
        }
    }

    pub fn get_pixels_above_lines(&self) -> i32 {
        unsafe {
            ffi::gtk_text_view_get_pixels_above_lines(self.to_glib_none().0)
        }
    }

    pub fn get_pixels_below_lines(&self) -> i32 {
        unsafe {
            ffi::gtk_text_view_get_pixels_below_lines(self.to_glib_none().0)
        }
    }

    pub fn get_pixels_inside_wrap(&self) -> i32 {
        unsafe {
            ffi::gtk_text_view_get_pixels_inside_wrap(self.to_glib_none().0)
        }
    }

    pub fn get_right_margin(&self) -> i32 {
        unsafe {
            ffi::gtk_text_view_get_right_margin(self.to_glib_none().0)
        }
    }

    //pub fn get_tabs(&self) -> /*Ignored*/Option<pango::TabArray> {
    //    unsafe { TODO: call ffi::gtk_text_view_get_tabs() }
    //}

    #[cfg(feature = "v3_18")]
    pub fn get_top_margin(&self) -> i32 {
        unsafe {
            ffi::gtk_text_view_get_top_margin(self.to_glib_none().0)
        }
    }

    pub fn get_visible_rect(&self) -> gdk::Rectangle {
        unsafe {
            let mut visible_rect = gdk::Rectangle::uninitialized();
            ffi::gtk_text_view_get_visible_rect(self.to_glib_none().0, visible_rect.to_glib_none_mut().0);
            visible_rect
        }
    }

    pub fn get_window(&self, win: TextWindowType) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_text_view_get_window(self.to_glib_none().0, win.to_glib()))
        }
    }

    pub fn get_window_type(&self, window: &gdk::Window) -> TextWindowType {
        unsafe {
            from_glib(ffi::gtk_text_view_get_window_type(self.to_glib_none().0, window.to_glib_none().0))
        }
    }

    pub fn get_wrap_mode(&self) -> WrapMode {
        unsafe {
            from_glib(ffi::gtk_text_view_get_wrap_mode(self.to_glib_none().0))
        }
    }

    pub fn im_context_filter_keypress(&self, event: &gdk::EventKey) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_view_im_context_filter_keypress(self.to_glib_none().0, mut_override(event.to_glib_none().0)))
        }
    }

    pub fn move_child<T: IsA<Widget>>(&self, child: &T, xpos: i32, ypos: i32) {
        unsafe {
            ffi::gtk_text_view_move_child(self.to_glib_none().0, child.to_glib_none().0, xpos, ypos);
        }
    }

    pub fn move_mark_onscreen(&self, mark: &TextMark) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_view_move_mark_onscreen(self.to_glib_none().0, mark.to_glib_none().0))
        }
    }

    pub fn move_visually(&self, iter: &mut TextIter, count: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_view_move_visually(self.to_glib_none().0, iter.to_glib_none_mut().0, count))
        }
    }

    pub fn place_cursor_onscreen(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_view_place_cursor_onscreen(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_20")]
    pub fn reset_cursor_blink(&self) {
        unsafe {
            ffi::gtk_text_view_reset_cursor_blink(self.to_glib_none().0);
        }
    }

    pub fn reset_im_context(&self) {
        unsafe {
            ffi::gtk_text_view_reset_im_context(self.to_glib_none().0);
        }
    }

    pub fn scroll_mark_onscreen(&self, mark: &TextMark) {
        unsafe {
            ffi::gtk_text_view_scroll_mark_onscreen(self.to_glib_none().0, mark.to_glib_none().0);
        }
    }

    pub fn scroll_to_iter(&self, iter: &mut TextIter, within_margin: f64, use_align: bool, xalign: f64, yalign: f64) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_view_scroll_to_iter(self.to_glib_none().0, iter.to_glib_none_mut().0, within_margin, use_align.to_glib(), xalign, yalign))
        }
    }

    pub fn scroll_to_mark(&self, mark: &TextMark, within_margin: f64, use_align: bool, xalign: f64, yalign: f64) {
        unsafe {
            ffi::gtk_text_view_scroll_to_mark(self.to_glib_none().0, mark.to_glib_none().0, within_margin, use_align.to_glib(), xalign, yalign);
        }
    }

    pub fn set_accepts_tab(&self, accepts_tab: bool) {
        unsafe {
            ffi::gtk_text_view_set_accepts_tab(self.to_glib_none().0, accepts_tab.to_glib());
        }
    }

    pub fn set_border_window_size(&self, type_: TextWindowType, size: i32) {
        unsafe {
            ffi::gtk_text_view_set_border_window_size(self.to_glib_none().0, type_.to_glib(), size);
        }
    }

    #[cfg(feature = "v3_18")]
    pub fn set_bottom_margin(&self, bottom_margin: i32) {
        unsafe {
            ffi::gtk_text_view_set_bottom_margin(self.to_glib_none().0, bottom_margin);
        }
    }

    pub fn set_buffer(&self, buffer: Option<&TextBuffer>) {
        unsafe {
            ffi::gtk_text_view_set_buffer(self.to_glib_none().0, buffer.to_glib_none().0);
        }
    }

    pub fn set_cursor_visible(&self, setting: bool) {
        unsafe {
            ffi::gtk_text_view_set_cursor_visible(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_editable(&self, setting: bool) {
        unsafe {
            ffi::gtk_text_view_set_editable(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_indent(&self, indent: i32) {
        unsafe {
            ffi::gtk_text_view_set_indent(self.to_glib_none().0, indent);
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn set_input_hints(&self, hints: InputHints) {
        unsafe {
            ffi::gtk_text_view_set_input_hints(self.to_glib_none().0, hints.to_glib());
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn set_input_purpose(&self, purpose: InputPurpose) {
        unsafe {
            ffi::gtk_text_view_set_input_purpose(self.to_glib_none().0, purpose.to_glib());
        }
    }

    pub fn set_justification(&self, justification: Justification) {
        unsafe {
            ffi::gtk_text_view_set_justification(self.to_glib_none().0, justification.to_glib());
        }
    }

    pub fn set_left_margin(&self, left_margin: i32) {
        unsafe {
            ffi::gtk_text_view_set_left_margin(self.to_glib_none().0, left_margin);
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn set_monospace(&self, monospace: bool) {
        unsafe {
            ffi::gtk_text_view_set_monospace(self.to_glib_none().0, monospace.to_glib());
        }
    }

    pub fn set_overwrite(&self, overwrite: bool) {
        unsafe {
            ffi::gtk_text_view_set_overwrite(self.to_glib_none().0, overwrite.to_glib());
        }
    }

    pub fn set_pixels_above_lines(&self, pixels_above_lines: i32) {
        unsafe {
            ffi::gtk_text_view_set_pixels_above_lines(self.to_glib_none().0, pixels_above_lines);
        }
    }

    pub fn set_pixels_below_lines(&self, pixels_below_lines: i32) {
        unsafe {
            ffi::gtk_text_view_set_pixels_below_lines(self.to_glib_none().0, pixels_below_lines);
        }
    }

    pub fn set_pixels_inside_wrap(&self, pixels_inside_wrap: i32) {
        unsafe {
            ffi::gtk_text_view_set_pixels_inside_wrap(self.to_glib_none().0, pixels_inside_wrap);
        }
    }

    pub fn set_right_margin(&self, right_margin: i32) {
        unsafe {
            ffi::gtk_text_view_set_right_margin(self.to_glib_none().0, right_margin);
        }
    }

    //pub fn set_tabs(&self, tabs: /*Ignored*/&mut pango::TabArray) {
    //    unsafe { TODO: call ffi::gtk_text_view_set_tabs() }
    //}

    #[cfg(feature = "v3_18")]
    pub fn set_top_margin(&self, top_margin: i32) {
        unsafe {
            ffi::gtk_text_view_set_top_margin(self.to_glib_none().0, top_margin);
        }
    }

    pub fn set_wrap_mode(&self, wrap_mode: WrapMode) {
        unsafe {
            ffi::gtk_text_view_set_wrap_mode(self.to_glib_none().0, wrap_mode.to_glib());
        }
    }

    pub fn starts_display_line(&self, iter: &TextIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_view_starts_display_line(self.to_glib_none().0, iter.to_glib_none().0))
        }
    }

    pub fn window_to_buffer_coords(&self, win: TextWindowType, window_x: i32, window_y: i32) -> (i32, i32) {
        unsafe {
            let mut buffer_x = mem::uninitialized();
            let mut buffer_y = mem::uninitialized();
            ffi::gtk_text_view_window_to_buffer_coords(self.to_glib_none().0, win.to_glib(), window_x, window_y, &mut buffer_x, &mut buffer_y);
            (buffer_x, buffer_y)
        }
    }

    pub fn get_property_im_module(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "im-module".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn set_property_im_module(&self, im_module: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "im-module".to_glib_none().0, Value::from(im_module).to_glib_none().0);
        }
    }

    pub fn get_property_monospace(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "monospace".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_property_monospace(&self, monospace: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "monospace".to_glib_none().0, Value::from(&monospace).to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_8")]
    pub fn get_property_populate_all(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "populate-all".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    #[cfg(feature = "v3_8")]
    pub fn set_property_populate_all(&self, populate_all: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "populate-all".to_glib_none().0, Value::from(&populate_all).to_glib_none().0);
        }
    }

    pub fn connect_backspace<F: Fn(&TextView) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextView) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "backspace",
                transmute(backspace_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_copy_clipboard<F: Fn(&TextView) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextView) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "copy-clipboard",
                transmute(copy_clipboard_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_cut_clipboard<F: Fn(&TextView) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextView) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "cut-clipboard",
                transmute(cut_clipboard_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_delete_from_cursor<F: Fn(&TextView, DeleteType, i32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextView, DeleteType, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "delete-from-cursor",
                transmute(delete_from_cursor_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn connect_extend_selection<F: Fn(&TextView, TextExtendSelection, &TextIter, &TextIter, &TextIter) -> Inhibit + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextView, TextExtendSelection, &TextIter, &TextIter, &TextIter) -> Inhibit + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "extend-selection",
                transmute(extend_selection_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_insert_at_cursor<F: Fn(&TextView, &str) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextView, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "insert-at-cursor",
                transmute(insert_at_cursor_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_move_cursor<F: Fn(&TextView, MovementStep, i32, bool) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextView, MovementStep, i32, bool) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-cursor",
                transmute(move_cursor_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_move_viewport<F: Fn(&TextView, ScrollStep, i32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextView, ScrollStep, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-viewport",
                transmute(move_viewport_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_paste_clipboard<F: Fn(&TextView) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextView) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "paste-clipboard",
                transmute(paste_clipboard_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_populate_popup<F: Fn(&TextView, &Widget) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextView, &Widget) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "populate-popup",
                transmute(populate_popup_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_preedit_changed<F: Fn(&TextView, &str) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextView, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "preedit-changed",
                transmute(preedit_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_select_all<F: Fn(&TextView, bool) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextView, bool) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "select-all",
                transmute(select_all_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_set_anchor<F: Fn(&TextView) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextView) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "set-anchor",
                transmute(set_anchor_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_toggle_cursor_visible<F: Fn(&TextView) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextView) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "toggle-cursor-visible",
                transmute(toggle_cursor_visible_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_toggle_overwrite<F: Fn(&TextView) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextView) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "toggle-overwrite",
                transmute(toggle_overwrite_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn backspace_trampoline(this: *mut ffi::GtkTextView, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextView) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn copy_clipboard_trampoline(this: *mut ffi::GtkTextView, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextView) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn cut_clipboard_trampoline(this: *mut ffi::GtkTextView, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextView) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn delete_from_cursor_trampoline(this: *mut ffi::GtkTextView, type_: ffi::GtkDeleteType, count: libc::c_int, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextView, DeleteType, i32) + 'static> = transmute(f);
    f(&from_glib_none(this), from_glib(type_), count)
}

#[cfg(feature = "v3_16")]
unsafe extern "C" fn extend_selection_trampoline(this: *mut ffi::GtkTextView, granularity: ffi::GtkTextExtendSelection, location: *mut ffi::GtkTextIter, start: *mut ffi::GtkTextIter, end: *mut ffi::GtkTextIter, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&TextView, TextExtendSelection, &TextIter, &TextIter, &TextIter) -> Inhibit + 'static> = transmute(f);
    f(&from_glib_none(this), from_glib(granularity), &from_glib_none(location), &from_glib_none(start), &from_glib_none(end)).to_glib()
}

unsafe extern "C" fn insert_at_cursor_trampoline(this: *mut ffi::GtkTextView, string: *mut libc::c_char, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextView, &str) + 'static> = transmute(f);
    f(&from_glib_none(this), &String::from_glib_none(string))
}

unsafe extern "C" fn move_cursor_trampoline(this: *mut ffi::GtkTextView, step: ffi::GtkMovementStep, count: libc::c_int, extend_selection: glib_ffi::gboolean, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextView, MovementStep, i32, bool) + 'static> = transmute(f);
    f(&from_glib_none(this), from_glib(step), count, from_glib(extend_selection))
}

unsafe extern "C" fn move_viewport_trampoline(this: *mut ffi::GtkTextView, step: ffi::GtkScrollStep, count: libc::c_int, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextView, ScrollStep, i32) + 'static> = transmute(f);
    f(&from_glib_none(this), from_glib(step), count)
}

unsafe extern "C" fn paste_clipboard_trampoline(this: *mut ffi::GtkTextView, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextView) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn populate_popup_trampoline(this: *mut ffi::GtkTextView, popup: *mut ffi::GtkWidget, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextView, &Widget) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(popup))
}

unsafe extern "C" fn preedit_changed_trampoline(this: *mut ffi::GtkTextView, preedit: *mut libc::c_char, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextView, &str) + 'static> = transmute(f);
    f(&from_glib_none(this), &String::from_glib_none(preedit))
}

unsafe extern "C" fn select_all_trampoline(this: *mut ffi::GtkTextView, select: glib_ffi::gboolean, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextView, bool) + 'static> = transmute(f);
    f(&from_glib_none(this), from_glib(select))
}

unsafe extern "C" fn set_anchor_trampoline(this: *mut ffi::GtkTextView, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextView) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn toggle_cursor_visible_trampoline(this: *mut ffi::GtkTextView, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextView) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn toggle_overwrite_trampoline(this: *mut ffi::GtkTextView, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextView) + 'static> = transmute(f);
    f(&from_glib_none(this))
}
