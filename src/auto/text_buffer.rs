// This file was generated by gir (28183c8) from gir-files (71d73f0)
// DO NOT EDIT

use Clipboard;
use TargetList;
use TextChildAnchor;
use TextIter;
use TextMark;
use TextTag;
use TextTagTable;
use ffi;
use gdk;
use gdk_pixbuf;
use gdk_pixbuf_ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct TextBuffer(Object<ffi::GtkTextBuffer>);

    match fn {
        get_type => || ffi::gtk_text_buffer_get_type(),
    }
}

impl TextBuffer {
    pub fn new<'a, P: Into<Option<&'a TextTagTable>>>(table: P) -> TextBuffer {
        assert_initialized_main_thread!();
        let table = table.into();
        let table = table.to_glib_none();
        unsafe {
            from_glib_full(ffi::gtk_text_buffer_new(table.0))
        }
    }
}

pub trait TextBufferExt {
    fn add_mark(&self, mark: &TextMark, where_: &TextIter);

    fn add_selection_clipboard(&self, clipboard: &Clipboard);

    fn apply_tag(&self, tag: &TextTag, start: &TextIter, end: &TextIter);

    fn apply_tag_by_name(&self, name: &str, start: &TextIter, end: &TextIter);

    fn backspace(&self, iter: &mut TextIter, interactive: bool, default_editable: bool) -> bool;

    fn begin_user_action(&self);

    fn copy_clipboard(&self, clipboard: &Clipboard);

    fn create_child_anchor(&self, iter: &mut TextIter) -> Option<TextChildAnchor>;

    fn create_mark<'a, P: Into<Option<&'a str>>>(&self, mark_name: P, where_: &TextIter, left_gravity: bool) -> Option<TextMark>;

    //fn create_tag<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(&self, tag_name: P, first_property_name: Q, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<TextTag>;

    fn cut_clipboard(&self, clipboard: &Clipboard, default_editable: bool);

    fn delete(&self, start: &mut TextIter, end: &mut TextIter);

    fn delete_interactive(&self, start_iter: &mut TextIter, end_iter: &mut TextIter, default_editable: bool) -> bool;

    fn delete_mark(&self, mark: &TextMark);

    fn delete_mark_by_name(&self, name: &str);

    fn delete_selection(&self, interactive: bool, default_editable: bool) -> bool;

    //fn deserialize(&self, content_buffer: &TextBuffer, format: &gdk::Atom, iter: &mut TextIter, data: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 3 }, length: /*Unimplemented*/Fundamental: Size) -> Result<(), Error>;

    fn deserialize_get_can_create_tags(&self, format: &gdk::Atom) -> bool;

    fn deserialize_set_can_create_tags(&self, format: &gdk::Atom, can_create_tags: bool);

    fn end_user_action(&self);

    fn get_bounds(&self) -> (TextIter, TextIter);

    fn get_char_count(&self) -> i32;

    fn get_copy_target_list(&self) -> Option<TargetList>;

    fn get_end_iter(&self) -> TextIter;

    fn get_has_selection(&self) -> bool;

    fn get_insert(&self) -> Option<TextMark>;

    fn get_iter_at_child_anchor(&self, anchor: &TextChildAnchor) -> TextIter;

    fn get_iter_at_line(&self, line_number: i32) -> TextIter;

    fn get_iter_at_line_index(&self, line_number: i32, byte_index: i32) -> TextIter;

    fn get_iter_at_line_offset(&self, line_number: i32, char_offset: i32) -> TextIter;

    fn get_iter_at_mark(&self, mark: &TextMark) -> TextIter;

    fn get_iter_at_offset(&self, char_offset: i32) -> TextIter;

    fn get_line_count(&self) -> i32;

    fn get_mark(&self, name: &str) -> Option<TextMark>;

    fn get_modified(&self) -> bool;

    fn get_paste_target_list(&self) -> Option<TargetList>;

    fn get_selection_bound(&self) -> Option<TextMark>;

    fn get_selection_bounds(&self) -> Option<(TextIter, TextIter)>;

    fn get_slice(&self, start: &TextIter, end: &TextIter, include_hidden_chars: bool) -> Option<String>;

    fn get_start_iter(&self) -> TextIter;

    fn get_tag_table(&self) -> Option<TextTagTable>;

    fn get_text(&self, start: &TextIter, end: &TextIter, include_hidden_chars: bool) -> Option<String>;

    fn insert_child_anchor(&self, iter: &mut TextIter, anchor: &TextChildAnchor);

    fn insert_pixbuf(&self, iter: &mut TextIter, pixbuf: &gdk_pixbuf::Pixbuf);

    fn insert_range(&self, iter: &mut TextIter, start: &TextIter, end: &TextIter);

    fn insert_range_interactive(&self, iter: &mut TextIter, start: &TextIter, end: &TextIter, default_editable: bool) -> bool;

    //fn insert_with_tags(&self, iter: &mut TextIter, text: &str, len: i32, first_tag: &TextTag, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn insert_with_tags_by_name(&self, iter: &mut TextIter, text: &str, len: i32, first_tag_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn move_mark(&self, mark: &TextMark, where_: &TextIter);

    fn move_mark_by_name(&self, name: &str, where_: &TextIter);

    fn paste_clipboard<'a, P: Into<Option<&'a TextIter>>>(&self, clipboard: &Clipboard, override_location: P, default_editable: bool);

    fn place_cursor(&self, where_: &TextIter);

    //fn register_deserialize_format<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, mime_type: &str, function: /*Unknown conversion*//*Unimplemented*/TextBufferDeserializeFunc, user_data: P, user_data_destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) -> Option<gdk::Atom>;

    fn register_deserialize_tagset<'a, P: Into<Option<&'a str>>>(&self, tagset_name: P) -> gdk::Atom;

    //fn register_serialize_format<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, mime_type: &str, function: /*Unknown conversion*//*Unimplemented*/TextBufferSerializeFunc, user_data: P, user_data_destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) -> Option<gdk::Atom>;

    fn register_serialize_tagset<'a, P: Into<Option<&'a str>>>(&self, tagset_name: P) -> gdk::Atom;

    fn remove_all_tags(&self, start: &TextIter, end: &TextIter);

    fn remove_selection_clipboard(&self, clipboard: &Clipboard);

    fn remove_tag(&self, tag: &TextTag, start: &TextIter, end: &TextIter);

    fn remove_tag_by_name(&self, name: &str, start: &TextIter, end: &TextIter);

    fn select_range(&self, ins: &TextIter, bound: &TextIter);

    //fn serialize(&self, content_buffer: &TextBuffer, format: &gdk::Atom, start: &TextIter, end: &TextIter) -> (/*Unimplemented*/CArray TypeId { ns_id: 0, id: 3 }, /*Unimplemented*/Fundamental: Size);

    fn set_modified(&self, setting: bool);

    fn unregister_deserialize_format(&self, format: &gdk::Atom);

    fn unregister_serialize_format(&self, format: &gdk::Atom);

    fn get_property_cursor_position(&self) -> i32;

    fn set_property_text(&self, text: Option<&str>);

    fn connect_apply_tag<F: Fn(&Self, &TextTag, &TextIter, &TextIter) + 'static>(&self, f: F) -> u64;

    fn connect_begin_user_action<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_delete_range<F: Fn(&Self, &TextIter, &TextIter) + 'static>(&self, f: F) -> u64;

    fn connect_end_user_action<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_insert_child_anchor<F: Fn(&Self, &TextIter, &TextChildAnchor) + 'static>(&self, f: F) -> u64;

    fn connect_insert_pixbuf<F: Fn(&Self, &TextIter, &gdk_pixbuf::Pixbuf) + 'static>(&self, f: F) -> u64;

    fn connect_mark_deleted<F: Fn(&Self, &TextMark) + 'static>(&self, f: F) -> u64;

    fn connect_mark_set<F: Fn(&Self, &TextIter, &TextMark) + 'static>(&self, f: F) -> u64;

    fn connect_modified_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_paste_done<F: Fn(&Self, &Clipboard) + 'static>(&self, f: F) -> u64;

    fn connect_remove_tag<F: Fn(&Self, &TextTag, &TextIter, &TextIter) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<TextBuffer> + IsA<glib::object::Object>> TextBufferExt for O {
    fn add_mark(&self, mark: &TextMark, where_: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_add_mark(self.to_glib_none().0, mark.to_glib_none().0, where_.to_glib_none().0);
        }
    }

    fn add_selection_clipboard(&self, clipboard: &Clipboard) {
        unsafe {
            ffi::gtk_text_buffer_add_selection_clipboard(self.to_glib_none().0, clipboard.to_glib_none().0);
        }
    }

    fn apply_tag(&self, tag: &TextTag, start: &TextIter, end: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_apply_tag(self.to_glib_none().0, tag.to_glib_none().0, start.to_glib_none().0, end.to_glib_none().0);
        }
    }

    fn apply_tag_by_name(&self, name: &str, start: &TextIter, end: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_apply_tag_by_name(self.to_glib_none().0, name.to_glib_none().0, start.to_glib_none().0, end.to_glib_none().0);
        }
    }

    fn backspace(&self, iter: &mut TextIter, interactive: bool, default_editable: bool) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_buffer_backspace(self.to_glib_none().0, iter.to_glib_none_mut().0, interactive.to_glib(), default_editable.to_glib()))
        }
    }

    fn begin_user_action(&self) {
        unsafe {
            ffi::gtk_text_buffer_begin_user_action(self.to_glib_none().0);
        }
    }

    fn copy_clipboard(&self, clipboard: &Clipboard) {
        unsafe {
            ffi::gtk_text_buffer_copy_clipboard(self.to_glib_none().0, clipboard.to_glib_none().0);
        }
    }

    fn create_child_anchor(&self, iter: &mut TextIter) -> Option<TextChildAnchor> {
        unsafe {
            from_glib_none(ffi::gtk_text_buffer_create_child_anchor(self.to_glib_none().0, iter.to_glib_none_mut().0))
        }
    }

    fn create_mark<'a, P: Into<Option<&'a str>>>(&self, mark_name: P, where_: &TextIter, left_gravity: bool) -> Option<TextMark> {
        let mark_name = mark_name.into();
        let mark_name = mark_name.to_glib_none();
        unsafe {
            from_glib_none(ffi::gtk_text_buffer_create_mark(self.to_glib_none().0, mark_name.0, where_.to_glib_none().0, left_gravity.to_glib()))
        }
    }

    //fn create_tag<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(&self, tag_name: P, first_property_name: Q, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<TextTag> {
    //    unsafe { TODO: call ffi::gtk_text_buffer_create_tag() }
    //}

    fn cut_clipboard(&self, clipboard: &Clipboard, default_editable: bool) {
        unsafe {
            ffi::gtk_text_buffer_cut_clipboard(self.to_glib_none().0, clipboard.to_glib_none().0, default_editable.to_glib());
        }
    }

    fn delete(&self, start: &mut TextIter, end: &mut TextIter) {
        unsafe {
            ffi::gtk_text_buffer_delete(self.to_glib_none().0, start.to_glib_none_mut().0, end.to_glib_none_mut().0);
        }
    }

    fn delete_interactive(&self, start_iter: &mut TextIter, end_iter: &mut TextIter, default_editable: bool) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_buffer_delete_interactive(self.to_glib_none().0, start_iter.to_glib_none_mut().0, end_iter.to_glib_none_mut().0, default_editable.to_glib()))
        }
    }

    fn delete_mark(&self, mark: &TextMark) {
        unsafe {
            ffi::gtk_text_buffer_delete_mark(self.to_glib_none().0, mark.to_glib_none().0);
        }
    }

    fn delete_mark_by_name(&self, name: &str) {
        unsafe {
            ffi::gtk_text_buffer_delete_mark_by_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    fn delete_selection(&self, interactive: bool, default_editable: bool) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_buffer_delete_selection(self.to_glib_none().0, interactive.to_glib(), default_editable.to_glib()))
        }
    }

    //fn deserialize(&self, content_buffer: &TextBuffer, format: &gdk::Atom, iter: &mut TextIter, data: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 3 }, length: /*Unimplemented*/Fundamental: Size) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::gtk_text_buffer_deserialize() }
    //}

    fn deserialize_get_can_create_tags(&self, format: &gdk::Atom) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_buffer_deserialize_get_can_create_tags(self.to_glib_none().0, format.to_glib_none().0))
        }
    }

    fn deserialize_set_can_create_tags(&self, format: &gdk::Atom, can_create_tags: bool) {
        unsafe {
            ffi::gtk_text_buffer_deserialize_set_can_create_tags(self.to_glib_none().0, format.to_glib_none().0, can_create_tags.to_glib());
        }
    }

    fn end_user_action(&self) {
        unsafe {
            ffi::gtk_text_buffer_end_user_action(self.to_glib_none().0);
        }
    }

    fn get_bounds(&self) -> (TextIter, TextIter) {
        unsafe {
            let mut start = TextIter::uninitialized();
            let mut end = TextIter::uninitialized();
            ffi::gtk_text_buffer_get_bounds(self.to_glib_none().0, start.to_glib_none_mut().0, end.to_glib_none_mut().0);
            (start, end)
        }
    }

    fn get_char_count(&self) -> i32 {
        unsafe {
            ffi::gtk_text_buffer_get_char_count(self.to_glib_none().0)
        }
    }

    fn get_copy_target_list(&self) -> Option<TargetList> {
        unsafe {
            from_glib_none(ffi::gtk_text_buffer_get_copy_target_list(self.to_glib_none().0))
        }
    }

    fn get_end_iter(&self) -> TextIter {
        unsafe {
            let mut iter = TextIter::uninitialized();
            ffi::gtk_text_buffer_get_end_iter(self.to_glib_none().0, iter.to_glib_none_mut().0);
            iter
        }
    }

    fn get_has_selection(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_buffer_get_has_selection(self.to_glib_none().0))
        }
    }

    fn get_insert(&self) -> Option<TextMark> {
        unsafe {
            from_glib_none(ffi::gtk_text_buffer_get_insert(self.to_glib_none().0))
        }
    }

    fn get_iter_at_child_anchor(&self, anchor: &TextChildAnchor) -> TextIter {
        unsafe {
            let mut iter = TextIter::uninitialized();
            ffi::gtk_text_buffer_get_iter_at_child_anchor(self.to_glib_none().0, iter.to_glib_none_mut().0, anchor.to_glib_none().0);
            iter
        }
    }

    fn get_iter_at_line(&self, line_number: i32) -> TextIter {
        unsafe {
            let mut iter = TextIter::uninitialized();
            ffi::gtk_text_buffer_get_iter_at_line(self.to_glib_none().0, iter.to_glib_none_mut().0, line_number);
            iter
        }
    }

    fn get_iter_at_line_index(&self, line_number: i32, byte_index: i32) -> TextIter {
        unsafe {
            let mut iter = TextIter::uninitialized();
            ffi::gtk_text_buffer_get_iter_at_line_index(self.to_glib_none().0, iter.to_glib_none_mut().0, line_number, byte_index);
            iter
        }
    }

    fn get_iter_at_line_offset(&self, line_number: i32, char_offset: i32) -> TextIter {
        unsafe {
            let mut iter = TextIter::uninitialized();
            ffi::gtk_text_buffer_get_iter_at_line_offset(self.to_glib_none().0, iter.to_glib_none_mut().0, line_number, char_offset);
            iter
        }
    }

    fn get_iter_at_mark(&self, mark: &TextMark) -> TextIter {
        unsafe {
            let mut iter = TextIter::uninitialized();
            ffi::gtk_text_buffer_get_iter_at_mark(self.to_glib_none().0, iter.to_glib_none_mut().0, mark.to_glib_none().0);
            iter
        }
    }

    fn get_iter_at_offset(&self, char_offset: i32) -> TextIter {
        unsafe {
            let mut iter = TextIter::uninitialized();
            ffi::gtk_text_buffer_get_iter_at_offset(self.to_glib_none().0, iter.to_glib_none_mut().0, char_offset);
            iter
        }
    }

    fn get_line_count(&self) -> i32 {
        unsafe {
            ffi::gtk_text_buffer_get_line_count(self.to_glib_none().0)
        }
    }

    fn get_mark(&self, name: &str) -> Option<TextMark> {
        unsafe {
            from_glib_none(ffi::gtk_text_buffer_get_mark(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn get_modified(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_buffer_get_modified(self.to_glib_none().0))
        }
    }

    fn get_paste_target_list(&self) -> Option<TargetList> {
        unsafe {
            from_glib_none(ffi::gtk_text_buffer_get_paste_target_list(self.to_glib_none().0))
        }
    }

    fn get_selection_bound(&self) -> Option<TextMark> {
        unsafe {
            from_glib_none(ffi::gtk_text_buffer_get_selection_bound(self.to_glib_none().0))
        }
    }

    fn get_selection_bounds(&self) -> Option<(TextIter, TextIter)> {
        unsafe {
            let mut start = TextIter::uninitialized();
            let mut end = TextIter::uninitialized();
            let ret = from_glib(ffi::gtk_text_buffer_get_selection_bounds(self.to_glib_none().0, start.to_glib_none_mut().0, end.to_glib_none_mut().0));
            if ret { Some((start, end)) } else { None }
        }
    }

    fn get_slice(&self, start: &TextIter, end: &TextIter, include_hidden_chars: bool) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_text_buffer_get_slice(self.to_glib_none().0, start.to_glib_none().0, end.to_glib_none().0, include_hidden_chars.to_glib()))
        }
    }

    fn get_start_iter(&self) -> TextIter {
        unsafe {
            let mut iter = TextIter::uninitialized();
            ffi::gtk_text_buffer_get_start_iter(self.to_glib_none().0, iter.to_glib_none_mut().0);
            iter
        }
    }

    fn get_tag_table(&self) -> Option<TextTagTable> {
        unsafe {
            from_glib_none(ffi::gtk_text_buffer_get_tag_table(self.to_glib_none().0))
        }
    }

    fn get_text(&self, start: &TextIter, end: &TextIter, include_hidden_chars: bool) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_text_buffer_get_text(self.to_glib_none().0, start.to_glib_none().0, end.to_glib_none().0, include_hidden_chars.to_glib()))
        }
    }

    fn insert_child_anchor(&self, iter: &mut TextIter, anchor: &TextChildAnchor) {
        unsafe {
            ffi::gtk_text_buffer_insert_child_anchor(self.to_glib_none().0, iter.to_glib_none_mut().0, anchor.to_glib_none().0);
        }
    }

    fn insert_pixbuf(&self, iter: &mut TextIter, pixbuf: &gdk_pixbuf::Pixbuf) {
        unsafe {
            ffi::gtk_text_buffer_insert_pixbuf(self.to_glib_none().0, iter.to_glib_none_mut().0, pixbuf.to_glib_none().0);
        }
    }

    fn insert_range(&self, iter: &mut TextIter, start: &TextIter, end: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_insert_range(self.to_glib_none().0, iter.to_glib_none_mut().0, start.to_glib_none().0, end.to_glib_none().0);
        }
    }

    fn insert_range_interactive(&self, iter: &mut TextIter, start: &TextIter, end: &TextIter, default_editable: bool) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_buffer_insert_range_interactive(self.to_glib_none().0, iter.to_glib_none_mut().0, start.to_glib_none().0, end.to_glib_none().0, default_editable.to_glib()))
        }
    }

    //fn insert_with_tags(&self, iter: &mut TextIter, text: &str, len: i32, first_tag: &TextTag, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_insert_with_tags() }
    //}

    //fn insert_with_tags_by_name(&self, iter: &mut TextIter, text: &str, len: i32, first_tag_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_insert_with_tags_by_name() }
    //}

    fn move_mark(&self, mark: &TextMark, where_: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_move_mark(self.to_glib_none().0, mark.to_glib_none().0, where_.to_glib_none().0);
        }
    }

    fn move_mark_by_name(&self, name: &str, where_: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_move_mark_by_name(self.to_glib_none().0, name.to_glib_none().0, where_.to_glib_none().0);
        }
    }

    fn paste_clipboard<'a, P: Into<Option<&'a TextIter>>>(&self, clipboard: &Clipboard, override_location: P, default_editable: bool) {
        let override_location = override_location.into();
        unsafe {
            ffi::gtk_text_buffer_paste_clipboard(self.to_glib_none().0, clipboard.to_glib_none().0, mut_override(override_location.to_glib_none().0), default_editable.to_glib());
        }
    }

    fn place_cursor(&self, where_: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_place_cursor(self.to_glib_none().0, where_.to_glib_none().0);
        }
    }

    //fn register_deserialize_format<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, mime_type: &str, function: /*Unknown conversion*//*Unimplemented*/TextBufferDeserializeFunc, user_data: P, user_data_destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) -> Option<gdk::Atom> {
    //    unsafe { TODO: call ffi::gtk_text_buffer_register_deserialize_format() }
    //}

    fn register_deserialize_tagset<'a, P: Into<Option<&'a str>>>(&self, tagset_name: P) -> gdk::Atom {
        let tagset_name = tagset_name.into();
        let tagset_name = tagset_name.to_glib_none();
        unsafe {
            from_glib_none(ffi::gtk_text_buffer_register_deserialize_tagset(self.to_glib_none().0, tagset_name.0))
        }
    }

    //fn register_serialize_format<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, mime_type: &str, function: /*Unknown conversion*//*Unimplemented*/TextBufferSerializeFunc, user_data: P, user_data_destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) -> Option<gdk::Atom> {
    //    unsafe { TODO: call ffi::gtk_text_buffer_register_serialize_format() }
    //}

    fn register_serialize_tagset<'a, P: Into<Option<&'a str>>>(&self, tagset_name: P) -> gdk::Atom {
        let tagset_name = tagset_name.into();
        let tagset_name = tagset_name.to_glib_none();
        unsafe {
            from_glib_none(ffi::gtk_text_buffer_register_serialize_tagset(self.to_glib_none().0, tagset_name.0))
        }
    }

    fn remove_all_tags(&self, start: &TextIter, end: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_remove_all_tags(self.to_glib_none().0, start.to_glib_none().0, end.to_glib_none().0);
        }
    }

    fn remove_selection_clipboard(&self, clipboard: &Clipboard) {
        unsafe {
            ffi::gtk_text_buffer_remove_selection_clipboard(self.to_glib_none().0, clipboard.to_glib_none().0);
        }
    }

    fn remove_tag(&self, tag: &TextTag, start: &TextIter, end: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_remove_tag(self.to_glib_none().0, tag.to_glib_none().0, start.to_glib_none().0, end.to_glib_none().0);
        }
    }

    fn remove_tag_by_name(&self, name: &str, start: &TextIter, end: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_remove_tag_by_name(self.to_glib_none().0, name.to_glib_none().0, start.to_glib_none().0, end.to_glib_none().0);
        }
    }

    fn select_range(&self, ins: &TextIter, bound: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_select_range(self.to_glib_none().0, ins.to_glib_none().0, bound.to_glib_none().0);
        }
    }

    //fn serialize(&self, content_buffer: &TextBuffer, format: &gdk::Atom, start: &TextIter, end: &TextIter) -> (/*Unimplemented*/CArray TypeId { ns_id: 0, id: 3 }, /*Unimplemented*/Fundamental: Size) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_serialize() }
    //}

    fn set_modified(&self, setting: bool) {
        unsafe {
            ffi::gtk_text_buffer_set_modified(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn unregister_deserialize_format(&self, format: &gdk::Atom) {
        unsafe {
            ffi::gtk_text_buffer_unregister_deserialize_format(self.to_glib_none().0, format.to_glib_none().0);
        }
    }

    fn unregister_serialize_format(&self, format: &gdk::Atom) {
        unsafe {
            ffi::gtk_text_buffer_unregister_serialize_format(self.to_glib_none().0, format.to_glib_none().0);
        }
    }

    fn get_property_cursor_position(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "cursor-position".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_text(&self, text: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "text".to_glib_none().0, Value::from(text).to_glib_none().0);
        }
    }

    fn connect_apply_tag<F: Fn(&Self, &TextTag, &TextIter, &TextIter) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &TextTag, &TextIter, &TextIter) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "apply-tag",
                transmute(apply_tag_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_begin_user_action<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "begin-user-action",
                transmute(begin_user_action_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "changed",
                transmute(changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_delete_range<F: Fn(&Self, &TextIter, &TextIter) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &TextIter, &TextIter) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "delete-range",
                transmute(delete_range_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_end_user_action<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "end-user-action",
                transmute(end_user_action_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_insert_child_anchor<F: Fn(&Self, &TextIter, &TextChildAnchor) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &TextIter, &TextChildAnchor) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "insert-child-anchor",
                transmute(insert_child_anchor_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_insert_pixbuf<F: Fn(&Self, &TextIter, &gdk_pixbuf::Pixbuf) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &TextIter, &gdk_pixbuf::Pixbuf) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "insert-pixbuf",
                transmute(insert_pixbuf_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_mark_deleted<F: Fn(&Self, &TextMark) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &TextMark) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "mark-deleted",
                transmute(mark_deleted_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_mark_set<F: Fn(&Self, &TextIter, &TextMark) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &TextIter, &TextMark) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "mark-set",
                transmute(mark_set_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_modified_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "modified-changed",
                transmute(modified_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_paste_done<F: Fn(&Self, &Clipboard) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Clipboard) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "paste-done",
                transmute(paste_done_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_remove_tag<F: Fn(&Self, &TextTag, &TextIter, &TextIter) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &TextTag, &TextIter, &TextIter) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "remove-tag",
                transmute(remove_tag_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn apply_tag_trampoline<P>(this: *mut ffi::GtkTextBuffer, tag: *mut ffi::GtkTextTag, start: *mut ffi::GtkTextIter, end: *mut ffi::GtkTextIter, f: glib_ffi::gpointer)
where P: IsA<TextBuffer> {
    callback_guard!();
    let f: &Box_<Fn(&P, &TextTag, &TextIter, &TextIter) + 'static> = transmute(f);
    f(&TextBuffer::from_glib_none(this).downcast_unchecked(), &from_glib_none(tag), &from_glib_none(start), &from_glib_none(end))
}

unsafe extern "C" fn begin_user_action_trampoline<P>(this: *mut ffi::GtkTextBuffer, f: glib_ffi::gpointer)
where P: IsA<TextBuffer> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&TextBuffer::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn changed_trampoline<P>(this: *mut ffi::GtkTextBuffer, f: glib_ffi::gpointer)
where P: IsA<TextBuffer> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&TextBuffer::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn delete_range_trampoline<P>(this: *mut ffi::GtkTextBuffer, start: *mut ffi::GtkTextIter, end: *mut ffi::GtkTextIter, f: glib_ffi::gpointer)
where P: IsA<TextBuffer> {
    callback_guard!();
    let f: &Box_<Fn(&P, &TextIter, &TextIter) + 'static> = transmute(f);
    f(&TextBuffer::from_glib_none(this).downcast_unchecked(), &from_glib_none(start), &from_glib_none(end))
}

unsafe extern "C" fn end_user_action_trampoline<P>(this: *mut ffi::GtkTextBuffer, f: glib_ffi::gpointer)
where P: IsA<TextBuffer> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&TextBuffer::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn insert_child_anchor_trampoline<P>(this: *mut ffi::GtkTextBuffer, location: *mut ffi::GtkTextIter, anchor: *mut ffi::GtkTextChildAnchor, f: glib_ffi::gpointer)
where P: IsA<TextBuffer> {
    callback_guard!();
    let f: &Box_<Fn(&P, &TextIter, &TextChildAnchor) + 'static> = transmute(f);
    f(&TextBuffer::from_glib_none(this).downcast_unchecked(), &from_glib_none(location), &from_glib_none(anchor))
}

unsafe extern "C" fn insert_pixbuf_trampoline<P>(this: *mut ffi::GtkTextBuffer, location: *mut ffi::GtkTextIter, pixbuf: *mut gdk_pixbuf_ffi::GdkPixbuf, f: glib_ffi::gpointer)
where P: IsA<TextBuffer> {
    callback_guard!();
    let f: &Box_<Fn(&P, &TextIter, &gdk_pixbuf::Pixbuf) + 'static> = transmute(f);
    f(&TextBuffer::from_glib_none(this).downcast_unchecked(), &from_glib_none(location), &from_glib_none(pixbuf))
}

unsafe extern "C" fn mark_deleted_trampoline<P>(this: *mut ffi::GtkTextBuffer, mark: *mut ffi::GtkTextMark, f: glib_ffi::gpointer)
where P: IsA<TextBuffer> {
    callback_guard!();
    let f: &Box_<Fn(&P, &TextMark) + 'static> = transmute(f);
    f(&TextBuffer::from_glib_none(this).downcast_unchecked(), &from_glib_none(mark))
}

unsafe extern "C" fn mark_set_trampoline<P>(this: *mut ffi::GtkTextBuffer, location: *mut ffi::GtkTextIter, mark: *mut ffi::GtkTextMark, f: glib_ffi::gpointer)
where P: IsA<TextBuffer> {
    callback_guard!();
    let f: &Box_<Fn(&P, &TextIter, &TextMark) + 'static> = transmute(f);
    f(&TextBuffer::from_glib_none(this).downcast_unchecked(), &from_glib_none(location), &from_glib_none(mark))
}

unsafe extern "C" fn modified_changed_trampoline<P>(this: *mut ffi::GtkTextBuffer, f: glib_ffi::gpointer)
where P: IsA<TextBuffer> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&TextBuffer::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn paste_done_trampoline<P>(this: *mut ffi::GtkTextBuffer, clipboard: *mut ffi::GtkClipboard, f: glib_ffi::gpointer)
where P: IsA<TextBuffer> {
    callback_guard!();
    let f: &Box_<Fn(&P, &Clipboard) + 'static> = transmute(f);
    f(&TextBuffer::from_glib_none(this).downcast_unchecked(), &from_glib_none(clipboard))
}

unsafe extern "C" fn remove_tag_trampoline<P>(this: *mut ffi::GtkTextBuffer, tag: *mut ffi::GtkTextTag, start: *mut ffi::GtkTextIter, end: *mut ffi::GtkTextIter, f: glib_ffi::gpointer)
where P: IsA<TextBuffer> {
    callback_guard!();
    let f: &Box_<Fn(&P, &TextTag, &TextIter, &TextIter) + 'static> = transmute(f);
    f(&TextBuffer::from_glib_none(this).downcast_unchecked(), &from_glib_none(tag), &from_glib_none(start), &from_glib_none(end))
}
