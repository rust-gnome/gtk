// This file was generated by gir (d50d839) from gir-files (469db10)
// DO NOT EDIT

use AccelGroup;
use Error;
use IconSource;
use Orientation;
use PageSetup;
use PositionType;
use PrintSettings;
use SelectionData;
use SpinButton;
use StyleContext;
use TextBuffer;
#[cfg(any(feature = "v3_12", feature = "dox"))]
use TextDirection;
use TreeModel;
use TreePath;
use Widget;
use Window;
use cairo;
use ffi;
use gdk;
use gdk_pixbuf;
use glib;
use glib::object::IsA;
use glib::translate::*;
use pango;
use std::mem;
use std::ptr;


pub fn accel_groups_activate<P: IsA<glib::Object>>(object: &P, accel_key: u32, accel_mods: gdk::ModifierType) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gtk_accel_groups_activate(object.to_glib_none().0, accel_key, accel_mods.to_glib()))
    }
}

pub fn accel_groups_from_object<P: IsA<glib::Object>>(object: &P) -> Vec<AccelGroup> {
    assert_initialized_main_thread!();
    unsafe {
        FromGlibPtrContainer::from_glib_none(ffi::gtk_accel_groups_from_object(object.to_glib_none().0))
    }
}

pub fn accelerator_get_default_mod_mask() -> gdk::ModifierType {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gtk_accelerator_get_default_mod_mask())
    }
}

pub fn accelerator_get_label(accelerator_key: u32, accelerator_mods: gdk::ModifierType) -> Option<String> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gtk_accelerator_get_label(accelerator_key, accelerator_mods.to_glib()))
    }
}

pub fn accelerator_get_label_with_keycode<'a, P: Into<Option<&'a gdk::Display>>>(display: P, accelerator_key: u32, keycode: u32, accelerator_mods: gdk::ModifierType) -> Option<String> {
    assert_initialized_main_thread!();
    let display = display.into();
    let display = display.to_glib_none();
    unsafe {
        from_glib_full(ffi::gtk_accelerator_get_label_with_keycode(display.0, accelerator_key, keycode, accelerator_mods.to_glib()))
    }
}

pub fn accelerator_name(accelerator_key: u32, accelerator_mods: gdk::ModifierType) -> Option<String> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gtk_accelerator_name(accelerator_key, accelerator_mods.to_glib()))
    }
}

pub fn accelerator_name_with_keycode<'a, P: Into<Option<&'a gdk::Display>>>(display: P, accelerator_key: u32, keycode: u32, accelerator_mods: gdk::ModifierType) -> Option<String> {
    assert_initialized_main_thread!();
    let display = display.into();
    let display = display.to_glib_none();
    unsafe {
        from_glib_full(ffi::gtk_accelerator_name_with_keycode(display.0, accelerator_key, keycode, accelerator_mods.to_glib()))
    }
}

pub fn accelerator_parse(accelerator: &str) -> (u32, gdk::ModifierType) {
    assert_initialized_main_thread!();
    unsafe {
        let mut accelerator_key = mem::uninitialized();
        let mut accelerator_mods = mem::uninitialized();
        ffi::gtk_accelerator_parse(accelerator.to_glib_none().0, &mut accelerator_key, &mut accelerator_mods);
        (accelerator_key, from_glib(accelerator_mods))
    }
}

//pub fn accelerator_parse_with_keycode(accelerator: &str, accelerator_codes: Vec<u32>) -> (u32, gdk::ModifierType) {
//    unsafe { TODO: call ffi::gtk_accelerator_parse_with_keycode() }
//}

pub fn accelerator_set_default_mod_mask(default_mod_mask: gdk::ModifierType) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_accelerator_set_default_mod_mask(default_mod_mask.to_glib());
    }
}

pub fn accelerator_valid(keyval: u32, modifiers: gdk::ModifierType) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gtk_accelerator_valid(keyval, modifiers.to_glib()))
    }
}

pub fn alternative_dialog_button_order<'a, P: Into<Option<&'a gdk::Screen>>>(screen: P) -> bool {
    assert_initialized_main_thread!();
    let screen = screen.into();
    let screen = screen.to_glib_none();
    unsafe {
        from_glib(ffi::gtk_alternative_dialog_button_order(screen.0))
    }
}

pub fn bindings_activate<P: IsA<glib::Object>>(object: &P, keyval: u32, modifiers: gdk::ModifierType) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gtk_bindings_activate(object.to_glib_none().0, keyval, modifiers.to_glib()))
    }
}

pub fn bindings_activate_event<P: IsA<glib::Object>>(object: &P, event: &mut gdk::EventKey) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gtk_bindings_activate_event(object.to_glib_none().0, event.to_glib_none_mut().0))
    }
}

pub fn cairo_should_draw_window(cr: &cairo::Context, window: &gdk::Window) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gtk_cairo_should_draw_window(mut_override(cr.to_glib_none().0), window.to_glib_none().0))
    }
}

pub fn cairo_transform_to_window<P: IsA<Widget>>(cr: &cairo::Context, widget: &P, window: &gdk::Window) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_cairo_transform_to_window(mut_override(cr.to_glib_none().0), widget.to_glib_none().0, window.to_glib_none().0);
    }
}

pub fn device_grab_add<P: IsA<Widget>, Q: IsA<gdk::Device>>(widget: &P, device: &Q, block_others: bool) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_device_grab_add(widget.to_glib_none().0, device.to_glib_none().0, block_others.to_glib());
    }
}

pub fn device_grab_remove<P: IsA<Widget>, Q: IsA<gdk::Device>>(widget: &P, device: &Q) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_device_grab_remove(widget.to_glib_none().0, device.to_glib_none().0);
    }
}

pub fn disable_setlocale() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_disable_setlocale();
    }
}

//pub fn distribute_natural_allocation(extra_space: i32, n_requested_sizes: u32, sizes: /*Ignored*/&mut RequestedSize) -> i32 {
//    unsafe { TODO: call ffi::gtk_distribute_natural_allocation() }
//}

pub fn events_pending() -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gtk_events_pending())
    }
}

pub fn false_() -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gtk_false())
    }
}

pub fn get_current_event() -> Option<gdk::Event> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gtk_get_current_event())
    }
}

pub fn get_current_event_device() -> Option<gdk::Device> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_none(ffi::gtk_get_current_event_device())
    }
}

pub fn get_current_event_state() -> Option<gdk::ModifierType> {
    assert_initialized_main_thread!();
    unsafe {
        let mut state = mem::uninitialized();
        let ret = from_glib(ffi::gtk_get_current_event_state(&mut state));
        if ret { Some(from_glib(state)) } else { None }
    }
}

pub fn get_current_event_time() -> u32 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_get_current_event_time()
    }
}

pub fn get_debug_flags() -> u32 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_get_debug_flags()
    }
}

pub fn get_default_language() -> Option<pango::Language> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_none(ffi::gtk_get_default_language())
    }
}

pub fn get_event_widget(event: &mut gdk::Event) -> Option<Widget> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_none(ffi::gtk_get_event_widget(event.to_glib_none_mut().0))
    }
}

#[cfg(any(feature = "v3_12", feature = "dox"))]
pub fn get_locale_direction() -> TextDirection {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gtk_get_locale_direction())
    }
}

//pub fn get_option_group(open_default_display: bool) -> /*Ignored*/Option<glib::OptionGroup> {
//    unsafe { TODO: call ffi::gtk_get_option_group() }
//}

pub fn grab_get_current() -> Option<Widget> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_none(ffi::gtk_grab_get_current())
    }
}

//pub fn init_check(argv: /*Unimplemented*/Vec<String>) -> bool {
//    unsafe { TODO: call ffi::gtk_init_check() }
//}

//pub fn init_with_args<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(argv: /*Unimplemented*/Vec<String>, parameter_string: P, entries: /*Ignored*/&[&glib::OptionEntry], translation_domain: Q) -> Result<(), Error> {
//    unsafe { TODO: call ffi::gtk_init_with_args() }
//}

pub fn main() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_main();
    }
}

pub fn main_do_event(event: &mut gdk::Event) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_main_do_event(event.to_glib_none_mut().0);
    }
}

pub fn main_iteration() -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gtk_main_iteration())
    }
}

pub fn main_iteration_do(blocking: bool) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gtk_main_iteration_do(blocking.to_glib()))
    }
}

pub fn main_level() -> u32 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_main_level()
    }
}

//pub fn parse_args(argv: /*Unimplemented*/Vec<String>) -> bool {
//    unsafe { TODO: call ffi::gtk_parse_args() }
//}

pub fn print_run_page_setup_dialog<'a, 'b, P: IsA<Window> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b PageSetup>>>(parent: Q, page_setup: R, settings: &PrintSettings) -> Option<PageSetup> {
    skip_assert_initialized!();
    let parent = parent.into();
    let parent = parent.to_glib_none();
    let page_setup = page_setup.into();
    let page_setup = page_setup.to_glib_none();
    unsafe {
        from_glib_full(ffi::gtk_print_run_page_setup_dialog(parent.0, page_setup.0, settings.to_glib_none().0))
    }
}

//pub fn print_run_page_setup_dialog_async<'a, 'b, P: IsA<Window> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b PageSetup>>, S: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(parent: Q, page_setup: R, settings: &PrintSettings, done_cb: /*Unknown conversion*//*Unimplemented*/PageSetupDoneFunc, data: S) {
//    unsafe { TODO: call ffi::gtk_print_run_page_setup_dialog_async() }
//}

pub fn propagate_event<P: IsA<Widget>>(widget: &P, event: &mut gdk::Event) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_propagate_event(widget.to_glib_none().0, event.to_glib_none_mut().0);
    }
}

pub fn render_activity(context: &StyleContext, cr: &cairo::Context, x: f64, y: f64, width: f64, height: f64) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_activity(context.to_glib_none().0, mut_override(cr.to_glib_none().0), x, y, width, height);
    }
}

pub fn render_arrow(context: &StyleContext, cr: &cairo::Context, angle: f64, x: f64, y: f64, size: f64) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_arrow(context.to_glib_none().0, mut_override(cr.to_glib_none().0), angle, x, y, size);
    }
}

pub fn render_background(context: &StyleContext, cr: &cairo::Context, x: f64, y: f64, width: f64, height: f64) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_background(context.to_glib_none().0, mut_override(cr.to_glib_none().0), x, y, width, height);
    }
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
pub fn render_background_get_clip(context: &StyleContext, x: f64, y: f64, width: f64, height: f64) -> gdk::Rectangle {
    skip_assert_initialized!();
    unsafe {
        let mut out_clip = gdk::Rectangle::uninitialized();
        ffi::gtk_render_background_get_clip(context.to_glib_none().0, x, y, width, height, out_clip.to_glib_none_mut().0);
        out_clip
    }
}

pub fn render_check(context: &StyleContext, cr: &cairo::Context, x: f64, y: f64, width: f64, height: f64) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_check(context.to_glib_none().0, mut_override(cr.to_glib_none().0), x, y, width, height);
    }
}

pub fn render_expander(context: &StyleContext, cr: &cairo::Context, x: f64, y: f64, width: f64, height: f64) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_expander(context.to_glib_none().0, mut_override(cr.to_glib_none().0), x, y, width, height);
    }
}

pub fn render_extension(context: &StyleContext, cr: &cairo::Context, x: f64, y: f64, width: f64, height: f64, gap_side: PositionType) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_extension(context.to_glib_none().0, mut_override(cr.to_glib_none().0), x, y, width, height, gap_side.to_glib());
    }
}

pub fn render_focus(context: &StyleContext, cr: &cairo::Context, x: f64, y: f64, width: f64, height: f64) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_focus(context.to_glib_none().0, mut_override(cr.to_glib_none().0), x, y, width, height);
    }
}

pub fn render_frame(context: &StyleContext, cr: &cairo::Context, x: f64, y: f64, width: f64, height: f64) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_frame(context.to_glib_none().0, mut_override(cr.to_glib_none().0), x, y, width, height);
    }
}

pub fn render_frame_gap(context: &StyleContext, cr: &cairo::Context, x: f64, y: f64, width: f64, height: f64, gap_side: PositionType, xy0_gap: f64, xy1_gap: f64) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_frame_gap(context.to_glib_none().0, mut_override(cr.to_glib_none().0), x, y, width, height, gap_side.to_glib(), xy0_gap, xy1_gap);
    }
}

pub fn render_handle(context: &StyleContext, cr: &cairo::Context, x: f64, y: f64, width: f64, height: f64) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_handle(context.to_glib_none().0, mut_override(cr.to_glib_none().0), x, y, width, height);
    }
}

pub fn render_icon(context: &StyleContext, cr: &cairo::Context, pixbuf: &gdk_pixbuf::Pixbuf, x: f64, y: f64) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_icon(context.to_glib_none().0, mut_override(cr.to_glib_none().0), pixbuf.to_glib_none().0, x, y);
    }
}

pub fn render_icon_pixbuf(context: &StyleContext, source: &IconSource, size: i32) -> Option<gdk_pixbuf::Pixbuf> {
    skip_assert_initialized!();
    unsafe {
        from_glib_full(ffi::gtk_render_icon_pixbuf(context.to_glib_none().0, source.to_glib_none().0, size))
    }
}

#[cfg(any(feature = "v3_10", feature = "dox"))]
pub fn render_icon_surface(context: &StyleContext, cr: &cairo::Context, surface: &cairo::Surface, x: f64, y: f64) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_icon_surface(context.to_glib_none().0, mut_override(cr.to_glib_none().0), mut_override(surface.to_glib_none().0), x, y);
    }
}

pub fn render_insertion_cursor(context: &StyleContext, cr: &cairo::Context, x: f64, y: f64, layout: &pango::Layout, index: i32, direction: pango::Direction) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_insertion_cursor(context.to_glib_none().0, mut_override(cr.to_glib_none().0), x, y, layout.to_glib_none().0, index, direction.to_glib());
    }
}

pub fn render_layout(context: &StyleContext, cr: &cairo::Context, x: f64, y: f64, layout: &pango::Layout) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_layout(context.to_glib_none().0, mut_override(cr.to_glib_none().0), x, y, layout.to_glib_none().0);
    }
}

pub fn render_line(context: &StyleContext, cr: &cairo::Context, x0: f64, y0: f64, x1: f64, y1: f64) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_line(context.to_glib_none().0, mut_override(cr.to_glib_none().0), x0, y0, x1, y1);
    }
}

pub fn render_option(context: &StyleContext, cr: &cairo::Context, x: f64, y: f64, width: f64, height: f64) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_option(context.to_glib_none().0, mut_override(cr.to_glib_none().0), x, y, width, height);
    }
}

pub fn render_slider(context: &StyleContext, cr: &cairo::Context, x: f64, y: f64, width: f64, height: f64, orientation: Orientation) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_render_slider(context.to_glib_none().0, mut_override(cr.to_glib_none().0), x, y, width, height, orientation.to_glib());
    }
}

pub fn rgb_to_hsv(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    assert_initialized_main_thread!();
    unsafe {
        let mut h = mem::uninitialized();
        let mut s = mem::uninitialized();
        let mut v = mem::uninitialized();
        ffi::gtk_rgb_to_hsv(r, g, b, &mut h, &mut s, &mut v);
        (h, s, v)
    }
}

pub fn selection_add_target<P: IsA<Widget>>(widget: &P, selection: &gdk::Atom, target: &gdk::Atom, info: u32) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_selection_add_target(widget.to_glib_none().0, selection.to_glib_none().0, target.to_glib_none().0, info);
    }
}

pub fn selection_clear_targets<P: IsA<Widget>>(widget: &P, selection: &gdk::Atom) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_selection_clear_targets(widget.to_glib_none().0, selection.to_glib_none().0);
    }
}

pub fn selection_convert<P: IsA<Widget>>(widget: &P, selection: &gdk::Atom, target: &gdk::Atom, time_: u32) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gtk_selection_convert(widget.to_glib_none().0, selection.to_glib_none().0, target.to_glib_none().0, time_))
    }
}

pub fn selection_owner_set<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(widget: Q, selection: &gdk::Atom, time_: u32) -> bool {
    assert_initialized_main_thread!();
    let widget = widget.into();
    let widget = widget.to_glib_none();
    unsafe {
        from_glib(ffi::gtk_selection_owner_set(widget.0, selection.to_glib_none().0, time_))
    }
}

pub fn selection_owner_set_for_display<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(display: &gdk::Display, widget: Q, selection: &gdk::Atom, time_: u32) -> bool {
    assert_initialized_main_thread!();
    let widget = widget.into();
    let widget = widget.to_glib_none();
    unsafe {
        from_glib(ffi::gtk_selection_owner_set_for_display(display.to_glib_none().0, widget.0, selection.to_glib_none().0, time_))
    }
}

pub fn selection_remove_all<P: IsA<Widget>>(widget: &P) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_selection_remove_all(widget.to_glib_none().0);
    }
}

pub fn set_debug_flags(flags: u32) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_set_debug_flags(flags);
    }
}

//pub fn show_about_dialog<'a, P: IsA<Window> + 'a, Q: Into<Option<&'a P>>>(parent: Q, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call ffi::gtk_show_about_dialog() }
//}

pub fn show_uri<'a, P: Into<Option<&'a gdk::Screen>>>(screen: P, uri: &str, timestamp: u32) -> Result<(), Error> {
    assert_initialized_main_thread!();
    let screen = screen.into();
    let screen = screen.to_glib_none();
    unsafe {
        let mut error = ptr::null_mut();
        let _ = ffi::gtk_show_uri(screen.0, uri.to_glib_none().0, timestamp, &mut error);
        if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
    }
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
pub fn show_uri_on_window<'a, P: IsA<Window> + 'a, Q: Into<Option<&'a P>>>(parent: Q, uri: &str, timestamp: u32) -> Result<(), Error> {
    assert_initialized_main_thread!();
    let parent = parent.into();
    let parent = parent.to_glib_none();
    unsafe {
        let mut error = ptr::null_mut();
        let _ = ffi::gtk_show_uri_on_window(parent.0, uri.to_glib_none().0, timestamp, &mut error);
        if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
    }
}

//pub fn stock_add(items: /*Ignored*/&[&StockItem]) {
//    unsafe { TODO: call ffi::gtk_stock_add() }
//}

//pub fn stock_add_static(items: /*Ignored*/&[&StockItem]) {
//    unsafe { TODO: call ffi::gtk_stock_add_static() }
//}

pub fn stock_list_ids() -> Vec<String> {
    assert_initialized_main_thread!();
    unsafe {
        FromGlibPtrContainer::from_glib_full(ffi::gtk_stock_list_ids())
    }
}

//pub fn stock_lookup(stock_id: &str, item: /*Ignored*/StockItem) -> bool {
//    unsafe { TODO: call ffi::gtk_stock_lookup() }
//}

//pub fn stock_set_translate_func<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(domain: &str, func: /*Unknown conversion*//*Unimplemented*/TranslateFunc, data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
//    unsafe { TODO: call ffi::gtk_stock_set_translate_func() }
//}

pub fn targets_include_image(targets: &[&gdk::Atom], writable: bool) -> bool {
    assert_initialized_main_thread!();
    let n_targets = targets.len() as i32;
    unsafe {
        from_glib(ffi::gtk_targets_include_image(targets.to_glib_none().0, n_targets, writable.to_glib()))
    }
}

pub fn targets_include_rich_text(targets: &[&gdk::Atom], buffer: &TextBuffer) -> bool {
    skip_assert_initialized!();
    let n_targets = targets.len() as i32;
    unsafe {
        from_glib(ffi::gtk_targets_include_rich_text(targets.to_glib_none().0, n_targets, buffer.to_glib_none().0))
    }
}

pub fn targets_include_text(targets: &[&gdk::Atom]) -> bool {
    assert_initialized_main_thread!();
    let n_targets = targets.len() as i32;
    unsafe {
        from_glib(ffi::gtk_targets_include_text(targets.to_glib_none().0, n_targets))
    }
}

pub fn targets_include_uri(targets: &[&gdk::Atom]) -> bool {
    assert_initialized_main_thread!();
    let n_targets = targets.len() as i32;
    unsafe {
        from_glib(ffi::gtk_targets_include_uri(targets.to_glib_none().0, n_targets))
    }
}

pub fn test_create_simple_window(window_title: &str, dialog_text: &str) -> Option<Widget> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_none(ffi::gtk_test_create_simple_window(window_title.to_glib_none().0, dialog_text.to_glib_none().0))
    }
}

//pub fn test_create_widget<'a, P: Into<Option<&'a str>>>(widget_type: glib::types::Type, first_property_name: P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<Widget> {
//    unsafe { TODO: call ffi::gtk_test_create_widget() }
//}

//pub fn test_display_button_window(window_title: &str, dialog_text: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<Widget> {
//    unsafe { TODO: call ffi::gtk_test_display_button_window() }
//}

pub fn test_find_label<P: IsA<Widget>>(widget: &P, label_pattern: &str) -> Option<Widget> {
    skip_assert_initialized!();
    unsafe {
        from_glib_none(ffi::gtk_test_find_label(widget.to_glib_none().0, label_pattern.to_glib_none().0))
    }
}

pub fn test_find_sibling<P: IsA<Widget>>(base_widget: &P, widget_type: glib::types::Type) -> Option<Widget> {
    skip_assert_initialized!();
    unsafe {
        from_glib_none(ffi::gtk_test_find_sibling(base_widget.to_glib_none().0, widget_type.to_glib()))
    }
}

pub fn test_find_widget<P: IsA<Widget>>(widget: &P, label_pattern: &str, widget_type: glib::types::Type) -> Option<Widget> {
    skip_assert_initialized!();
    unsafe {
        from_glib_none(ffi::gtk_test_find_widget(widget.to_glib_none().0, label_pattern.to_glib_none().0, widget_type.to_glib()))
    }
}

//pub fn test_init(argvp: /*Unimplemented*/Vec<String>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call ffi::gtk_test_init() }
//}

//pub fn test_list_all_types() -> /*Unimplemented*/CArray TypeId { ns_id: 0, id: 30 } {
//    unsafe { TODO: call ffi::gtk_test_list_all_types() }
//}

pub fn test_register_all_types() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_test_register_all_types();
    }
}

pub fn test_slider_get_value<P: IsA<Widget>>(widget: &P) -> f64 {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_test_slider_get_value(widget.to_glib_none().0)
    }
}

pub fn test_slider_set_perc<P: IsA<Widget>>(widget: &P, percentage: f64) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_test_slider_set_perc(widget.to_glib_none().0, percentage);
    }
}

pub fn test_spin_button_click(spinner: &SpinButton, button: u32, upwards: bool) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gtk_test_spin_button_click(spinner.to_glib_none().0, button, upwards.to_glib()))
    }
}

pub fn test_text_get<P: IsA<Widget>>(widget: &P) -> Option<String> {
    skip_assert_initialized!();
    unsafe {
        from_glib_full(ffi::gtk_test_text_get(widget.to_glib_none().0))
    }
}

pub fn test_text_set<P: IsA<Widget>>(widget: &P, string: &str) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_test_text_set(widget.to_glib_none().0, string.to_glib_none().0);
    }
}

pub fn test_widget_click<P: IsA<Widget>>(widget: &P, button: u32, modifiers: gdk::ModifierType) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gtk_test_widget_click(widget.to_glib_none().0, button, modifiers.to_glib()))
    }
}

pub fn test_widget_send_key<P: IsA<Widget>>(widget: &P, keyval: u32, modifiers: gdk::ModifierType) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gtk_test_widget_send_key(widget.to_glib_none().0, keyval, modifiers.to_glib()))
    }
}

#[cfg(any(feature = "v3_10", feature = "dox"))]
pub fn test_widget_wait_for_draw<P: IsA<Widget>>(widget: &P) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_test_widget_wait_for_draw(widget.to_glib_none().0);
    }
}

pub fn tree_get_row_drag_data(selection_data: &SelectionData) -> Option<(Option<TreeModel>, Option<TreePath>)> {
    assert_initialized_main_thread!();
    unsafe {
        let mut tree_model = ptr::null_mut();
        let mut path = ptr::null_mut();
        let ret = from_glib(ffi::gtk_tree_get_row_drag_data(mut_override(selection_data.to_glib_none().0), &mut tree_model, &mut path));
        if ret { Some((from_glib_none(tree_model), from_glib_full(path))) } else { None }
    }
}

pub fn tree_set_row_drag_data<P: IsA<TreeModel>>(selection_data: &SelectionData, tree_model: &P, path: &mut TreePath) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gtk_tree_set_row_drag_data(mut_override(selection_data.to_glib_none().0), tree_model.to_glib_none().0, path.to_glib_none_mut().0))
    }
}

pub fn true_() -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gtk_true())
    }
}
