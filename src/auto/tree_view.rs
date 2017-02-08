// This file was generated by gir (5232053) from gir-files (71d73f0)
// DO NOT EDIT

use CellRenderer;
use Container;
use Entry;
use MovementStep;
use Scrollable;
use Tooltip;
use TreeIter;
use TreeModel;
use TreePath;
use TreeSelection;
use TreeViewColumn;
use TreeViewDropPosition;
use TreeViewGridLines;
use Widget;
use cairo;
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
use signal::Inhibit;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct TreeView(Object<ffi::GtkTreeView>): Container, Widget, Scrollable;

    match fn {
        get_type => || ffi::gtk_tree_view_get_type(),
    }
}

impl TreeView {
    pub fn new() -> TreeView {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_tree_view_new()).downcast_unchecked()
        }
    }

    pub fn new_with_model<T: IsA<TreeModel>>(model: &T) -> TreeView {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_tree_view_new_with_model(model.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn append_column(&self, column: &TreeViewColumn) -> i32 {
        unsafe {
            ffi::gtk_tree_view_append_column(self.to_glib_none().0, column.to_glib_none().0)
        }
    }

    pub fn collapse_all(&self) {
        unsafe {
            ffi::gtk_tree_view_collapse_all(self.to_glib_none().0);
        }
    }

    pub fn collapse_row(&self, path: &TreePath) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_collapse_row(self.to_glib_none().0, mut_override(path.to_glib_none().0)))
        }
    }

    pub fn columns_autosize(&self) {
        unsafe {
            ffi::gtk_tree_view_columns_autosize(self.to_glib_none().0);
        }
    }

    pub fn convert_bin_window_to_tree_coords(&self, bx: i32, by: i32) -> (i32, i32) {
        unsafe {
            let mut tx = mem::uninitialized();
            let mut ty = mem::uninitialized();
            ffi::gtk_tree_view_convert_bin_window_to_tree_coords(self.to_glib_none().0, bx, by, &mut tx, &mut ty);
            (tx, ty)
        }
    }

    pub fn convert_bin_window_to_widget_coords(&self, bx: i32, by: i32) -> (i32, i32) {
        unsafe {
            let mut wx = mem::uninitialized();
            let mut wy = mem::uninitialized();
            ffi::gtk_tree_view_convert_bin_window_to_widget_coords(self.to_glib_none().0, bx, by, &mut wx, &mut wy);
            (wx, wy)
        }
    }

    pub fn convert_tree_to_bin_window_coords(&self, tx: i32, ty: i32) -> (i32, i32) {
        unsafe {
            let mut bx = mem::uninitialized();
            let mut by = mem::uninitialized();
            ffi::gtk_tree_view_convert_tree_to_bin_window_coords(self.to_glib_none().0, tx, ty, &mut bx, &mut by);
            (bx, by)
        }
    }

    pub fn convert_tree_to_widget_coords(&self, tx: i32, ty: i32) -> (i32, i32) {
        unsafe {
            let mut wx = mem::uninitialized();
            let mut wy = mem::uninitialized();
            ffi::gtk_tree_view_convert_tree_to_widget_coords(self.to_glib_none().0, tx, ty, &mut wx, &mut wy);
            (wx, wy)
        }
    }

    pub fn convert_widget_to_bin_window_coords(&self, wx: i32, wy: i32) -> (i32, i32) {
        unsafe {
            let mut bx = mem::uninitialized();
            let mut by = mem::uninitialized();
            ffi::gtk_tree_view_convert_widget_to_bin_window_coords(self.to_glib_none().0, wx, wy, &mut bx, &mut by);
            (bx, by)
        }
    }

    pub fn convert_widget_to_tree_coords(&self, wx: i32, wy: i32) -> (i32, i32) {
        unsafe {
            let mut tx = mem::uninitialized();
            let mut ty = mem::uninitialized();
            ffi::gtk_tree_view_convert_widget_to_tree_coords(self.to_glib_none().0, wx, wy, &mut tx, &mut ty);
            (tx, ty)
        }
    }

    pub fn create_row_drag_icon(&self, path: &TreePath) -> Option<cairo::Surface> {
        unsafe {
            from_glib_full(ffi::gtk_tree_view_create_row_drag_icon(self.to_glib_none().0, mut_override(path.to_glib_none().0)))
        }
    }

    //pub fn enable_model_drag_dest(&self, targets: /*Ignored*/&[&TargetEntry], n_targets: i32, actions: /*Ignored*/gdk::DragAction) {
    //    unsafe { TODO: call ffi::gtk_tree_view_enable_model_drag_dest() }
    //}

    //pub fn enable_model_drag_source(&self, start_button_mask: gdk::ModifierType, targets: /*Ignored*/&[&TargetEntry], n_targets: i32, actions: /*Ignored*/gdk::DragAction) {
    //    unsafe { TODO: call ffi::gtk_tree_view_enable_model_drag_source() }
    //}

    pub fn expand_all(&self) {
        unsafe {
            ffi::gtk_tree_view_expand_all(self.to_glib_none().0);
        }
    }

    pub fn expand_row(&self, path: &TreePath, open_all: bool) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_expand_row(self.to_glib_none().0, mut_override(path.to_glib_none().0), open_all.to_glib()))
        }
    }

    pub fn expand_to_path(&self, path: &TreePath) {
        unsafe {
            ffi::gtk_tree_view_expand_to_path(self.to_glib_none().0, mut_override(path.to_glib_none().0));
        }
    }

    #[cfg(feature = "v3_8")]
    pub fn get_activate_on_single_click(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_get_activate_on_single_click(self.to_glib_none().0))
        }
    }

    pub fn get_background_area(&self, path: Option<&TreePath>, column: Option<&TreeViewColumn>) -> gdk::Rectangle {
        unsafe {
            let mut rect = gdk::Rectangle::uninitialized();
            ffi::gtk_tree_view_get_background_area(self.to_glib_none().0, mut_override(path.to_glib_none().0), column.to_glib_none().0, rect.to_glib_none_mut().0);
            rect
        }
    }

    pub fn get_bin_window(&self) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_tree_view_get_bin_window(self.to_glib_none().0))
        }
    }

    pub fn get_cell_area(&self, path: Option<&TreePath>, column: Option<&TreeViewColumn>) -> gdk::Rectangle {
        unsafe {
            let mut rect = gdk::Rectangle::uninitialized();
            ffi::gtk_tree_view_get_cell_area(self.to_glib_none().0, mut_override(path.to_glib_none().0), column.to_glib_none().0, rect.to_glib_none_mut().0);
            rect
        }
    }

    pub fn get_column(&self, n: i32) -> Option<TreeViewColumn> {
        unsafe {
            from_glib_none(ffi::gtk_tree_view_get_column(self.to_glib_none().0, n))
        }
    }

    pub fn get_columns(&self) -> Vec<TreeViewColumn> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_tree_view_get_columns(self.to_glib_none().0))
        }
    }

    pub fn get_cursor(&self) -> (Option<TreePath>, Option<TreeViewColumn>) {
        unsafe {
            let mut path = ptr::null_mut();
            let mut focus_column = ptr::null_mut();
            ffi::gtk_tree_view_get_cursor(self.to_glib_none().0, &mut path, &mut focus_column);
            (from_glib_full(path), from_glib_none(focus_column))
        }
    }

    pub fn get_dest_row_at_pos(&self, drag_x: i32, drag_y: i32) -> Option<(Option<TreePath>, TreeViewDropPosition)> {
        unsafe {
            let mut path = ptr::null_mut();
            let mut pos = mem::uninitialized();
            let ret = from_glib(ffi::gtk_tree_view_get_dest_row_at_pos(self.to_glib_none().0, drag_x, drag_y, &mut path, &mut pos));
            if ret { Some((from_glib_full(path), from_glib(pos))) } else { None }
        }
    }

    pub fn get_drag_dest_row(&self) -> (Option<TreePath>, TreeViewDropPosition) {
        unsafe {
            let mut path = ptr::null_mut();
            let mut pos = mem::uninitialized();
            ffi::gtk_tree_view_get_drag_dest_row(self.to_glib_none().0, &mut path, &mut pos);
            (from_glib_full(path), from_glib(pos))
        }
    }

    pub fn get_enable_search(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_get_enable_search(self.to_glib_none().0))
        }
    }

    pub fn get_enable_tree_lines(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_get_enable_tree_lines(self.to_glib_none().0))
        }
    }

    pub fn get_expander_column(&self) -> Option<TreeViewColumn> {
        unsafe {
            from_glib_none(ffi::gtk_tree_view_get_expander_column(self.to_glib_none().0))
        }
    }

    pub fn get_fixed_height_mode(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_get_fixed_height_mode(self.to_glib_none().0))
        }
    }

    pub fn get_grid_lines(&self) -> TreeViewGridLines {
        unsafe {
            from_glib(ffi::gtk_tree_view_get_grid_lines(self.to_glib_none().0))
        }
    }

    pub fn get_headers_clickable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_get_headers_clickable(self.to_glib_none().0))
        }
    }

    pub fn get_headers_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_get_headers_visible(self.to_glib_none().0))
        }
    }

    pub fn get_hover_expand(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_get_hover_expand(self.to_glib_none().0))
        }
    }

    pub fn get_hover_selection(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_get_hover_selection(self.to_glib_none().0))
        }
    }

    pub fn get_level_indentation(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_get_level_indentation(self.to_glib_none().0)
        }
    }

    pub fn get_model(&self) -> Option<TreeModel> {
        unsafe {
            from_glib_none(ffi::gtk_tree_view_get_model(self.to_glib_none().0))
        }
    }

    pub fn get_n_columns(&self) -> u32 {
        unsafe {
            ffi::gtk_tree_view_get_n_columns(self.to_glib_none().0)
        }
    }

    pub fn get_path_at_pos(&self, x: i32, y: i32) -> Option<(Option<TreePath>, Option<TreeViewColumn>, i32, i32)> {
        unsafe {
            let mut path = ptr::null_mut();
            let mut column = ptr::null_mut();
            let mut cell_x = mem::uninitialized();
            let mut cell_y = mem::uninitialized();
            let ret = from_glib(ffi::gtk_tree_view_get_path_at_pos(self.to_glib_none().0, x, y, &mut path, &mut column, &mut cell_x, &mut cell_y));
            if ret { Some((from_glib_full(path), from_glib_none(column), cell_x, cell_y)) } else { None }
        }
    }

    pub fn get_reorderable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_get_reorderable(self.to_glib_none().0))
        }
    }

    //pub fn get_row_separator_func(&self) -> /*Unknown conversion*//*Unimplemented*/TreeViewRowSeparatorFunc {
    //    unsafe { TODO: call ffi::gtk_tree_view_get_row_separator_func() }
    //}

    pub fn get_rubber_banding(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_get_rubber_banding(self.to_glib_none().0))
        }
    }

    pub fn get_rules_hint(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_get_rules_hint(self.to_glib_none().0))
        }
    }

    pub fn get_search_column(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_get_search_column(self.to_glib_none().0)
        }
    }

    pub fn get_search_entry(&self) -> Option<Entry> {
        unsafe {
            from_glib_none(ffi::gtk_tree_view_get_search_entry(self.to_glib_none().0))
        }
    }

    //pub fn get_search_equal_func(&self) -> /*Unknown conversion*//*Unimplemented*/TreeViewSearchEqualFunc {
    //    unsafe { TODO: call ffi::gtk_tree_view_get_search_equal_func() }
    //}

    //pub fn get_search_position_func(&self) -> /*Unknown conversion*//*Unimplemented*/TreeViewSearchPositionFunc {
    //    unsafe { TODO: call ffi::gtk_tree_view_get_search_position_func() }
    //}

    pub fn get_selection(&self) -> TreeSelection {
        unsafe {
            from_glib_none(ffi::gtk_tree_view_get_selection(self.to_glib_none().0))
        }
    }

    pub fn get_show_expanders(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_get_show_expanders(self.to_glib_none().0))
        }
    }

    pub fn get_tooltip_column(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_get_tooltip_column(self.to_glib_none().0)
        }
    }

    pub fn get_tooltip_context(&self, x: &mut i32, y: &mut i32, keyboard_tip: bool) -> Option<(Option<TreeModel>, TreePath, TreeIter)> {
        unsafe {
            let mut model = ptr::null_mut();
            let mut path = ptr::null_mut();
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_view_get_tooltip_context(self.to_glib_none().0, x, y, keyboard_tip.to_glib(), &mut model, &mut path, iter.to_glib_none_mut().0));
            if ret { Some((from_glib_none(model), from_glib_full(path), iter)) } else { None }
        }
    }

    pub fn get_visible_range(&self) -> Option<(TreePath, TreePath)> {
        unsafe {
            let mut start_path = ptr::null_mut();
            let mut end_path = ptr::null_mut();
            let ret = from_glib(ffi::gtk_tree_view_get_visible_range(self.to_glib_none().0, &mut start_path, &mut end_path));
            if ret { Some((from_glib_full(start_path), from_glib_full(end_path))) } else { None }
        }
    }

    pub fn get_visible_rect(&self) -> gdk::Rectangle {
        unsafe {
            let mut visible_rect = gdk::Rectangle::uninitialized();
            ffi::gtk_tree_view_get_visible_rect(self.to_glib_none().0, visible_rect.to_glib_none_mut().0);
            visible_rect
        }
    }

    pub fn insert_column(&self, column: &TreeViewColumn, position: i32) -> i32 {
        unsafe {
            ffi::gtk_tree_view_insert_column(self.to_glib_none().0, column.to_glib_none().0, position)
        }
    }

    //pub fn insert_column_with_attributes<T: IsA<CellRenderer>>(&self, position: i32, title: &str, cell: &T, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> i32 {
    //    unsafe { TODO: call ffi::gtk_tree_view_insert_column_with_attributes() }
    //}

    //pub fn insert_column_with_data_func<T: IsA<CellRenderer>>(&self, position: i32, title: &str, cell: &T, func: /*Unknown conversion*//*Unimplemented*/TreeCellDataFunc, data: /*Unimplemented*/Option<Fundamental: Pointer>, dnotify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) -> i32 {
    //    unsafe { TODO: call ffi::gtk_tree_view_insert_column_with_data_func() }
    //}

    pub fn is_blank_at_pos(&self, x: i32, y: i32) -> Option<(TreePath, TreeViewColumn, i32, i32)> {
        unsafe {
            let mut path = ptr::null_mut();
            let mut column = ptr::null_mut();
            let mut cell_x = mem::uninitialized();
            let mut cell_y = mem::uninitialized();
            let ret = from_glib(ffi::gtk_tree_view_is_blank_at_pos(self.to_glib_none().0, x, y, &mut path, &mut column, &mut cell_x, &mut cell_y));
            if ret { Some((from_glib_full(path), from_glib_full(column), cell_x, cell_y)) } else { None }
        }
    }

    pub fn is_rubber_banding_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_is_rubber_banding_active(self.to_glib_none().0))
        }
    }

    //pub fn map_expanded_rows(&self, func: /*Unknown conversion*//*Unimplemented*/TreeViewMappingFunc, data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi::gtk_tree_view_map_expanded_rows() }
    //}

    pub fn move_column_after(&self, column: &TreeViewColumn, base_column: Option<&TreeViewColumn>) {
        unsafe {
            ffi::gtk_tree_view_move_column_after(self.to_glib_none().0, column.to_glib_none().0, base_column.to_glib_none().0);
        }
    }

    pub fn remove_column(&self, column: &TreeViewColumn) -> i32 {
        unsafe {
            ffi::gtk_tree_view_remove_column(self.to_glib_none().0, column.to_glib_none().0)
        }
    }

    pub fn row_activated(&self, path: &TreePath, column: &TreeViewColumn) {
        unsafe {
            ffi::gtk_tree_view_row_activated(self.to_glib_none().0, mut_override(path.to_glib_none().0), column.to_glib_none().0);
        }
    }

    pub fn row_expanded(&self, path: &TreePath) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_row_expanded(self.to_glib_none().0, mut_override(path.to_glib_none().0)))
        }
    }

    pub fn scroll_to_cell(&self, path: Option<&TreePath>, column: Option<&TreeViewColumn>, use_align: bool, row_align: f32, col_align: f32) {
        unsafe {
            ffi::gtk_tree_view_scroll_to_cell(self.to_glib_none().0, mut_override(path.to_glib_none().0), column.to_glib_none().0, use_align.to_glib(), row_align, col_align);
        }
    }

    pub fn scroll_to_point(&self, tree_x: i32, tree_y: i32) {
        unsafe {
            ffi::gtk_tree_view_scroll_to_point(self.to_glib_none().0, tree_x, tree_y);
        }
    }

    #[cfg(feature = "v3_8")]
    pub fn set_activate_on_single_click(&self, single: bool) {
        unsafe {
            ffi::gtk_tree_view_set_activate_on_single_click(self.to_glib_none().0, single.to_glib());
        }
    }

    //pub fn set_column_drag_function(&self, func: /*Unknown conversion*//*Unimplemented*/TreeViewColumnDropFunc, user_data: /*Unimplemented*/Option<Fundamental: Pointer>, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_tree_view_set_column_drag_function() }
    //}

    pub fn set_cursor(&self, path: &TreePath, focus_column: Option<&TreeViewColumn>, start_editing: bool) {
        unsafe {
            ffi::gtk_tree_view_set_cursor(self.to_glib_none().0, mut_override(path.to_glib_none().0), focus_column.to_glib_none().0, start_editing.to_glib());
        }
    }

    pub fn set_cursor_on_cell<T: IsA<CellRenderer>>(&self, path: &TreePath, focus_column: Option<&TreeViewColumn>, focus_cell: Option<&T>, start_editing: bool) {
        unsafe {
            ffi::gtk_tree_view_set_cursor_on_cell(self.to_glib_none().0, mut_override(path.to_glib_none().0), focus_column.to_glib_none().0, focus_cell.to_glib_none().0, start_editing.to_glib());
        }
    }

    pub fn set_drag_dest_row(&self, path: Option<&TreePath>, pos: TreeViewDropPosition) {
        unsafe {
            ffi::gtk_tree_view_set_drag_dest_row(self.to_glib_none().0, mut_override(path.to_glib_none().0), pos.to_glib());
        }
    }

    pub fn set_enable_search(&self, enable_search: bool) {
        unsafe {
            ffi::gtk_tree_view_set_enable_search(self.to_glib_none().0, enable_search.to_glib());
        }
    }

    pub fn set_enable_tree_lines(&self, enabled: bool) {
        unsafe {
            ffi::gtk_tree_view_set_enable_tree_lines(self.to_glib_none().0, enabled.to_glib());
        }
    }

    pub fn set_expander_column(&self, column: &TreeViewColumn) {
        unsafe {
            ffi::gtk_tree_view_set_expander_column(self.to_glib_none().0, column.to_glib_none().0);
        }
    }

    pub fn set_fixed_height_mode(&self, enable: bool) {
        unsafe {
            ffi::gtk_tree_view_set_fixed_height_mode(self.to_glib_none().0, enable.to_glib());
        }
    }

    pub fn set_grid_lines(&self, grid_lines: TreeViewGridLines) {
        unsafe {
            ffi::gtk_tree_view_set_grid_lines(self.to_glib_none().0, grid_lines.to_glib());
        }
    }

    pub fn set_headers_clickable(&self, setting: bool) {
        unsafe {
            ffi::gtk_tree_view_set_headers_clickable(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_headers_visible(&self, headers_visible: bool) {
        unsafe {
            ffi::gtk_tree_view_set_headers_visible(self.to_glib_none().0, headers_visible.to_glib());
        }
    }

    pub fn set_hover_expand(&self, expand: bool) {
        unsafe {
            ffi::gtk_tree_view_set_hover_expand(self.to_glib_none().0, expand.to_glib());
        }
    }

    pub fn set_hover_selection(&self, hover: bool) {
        unsafe {
            ffi::gtk_tree_view_set_hover_selection(self.to_glib_none().0, hover.to_glib());
        }
    }

    pub fn set_level_indentation(&self, indentation: i32) {
        unsafe {
            ffi::gtk_tree_view_set_level_indentation(self.to_glib_none().0, indentation);
        }
    }

    pub fn set_model<T: IsA<TreeModel>>(&self, model: Option<&T>) {
        unsafe {
            ffi::gtk_tree_view_set_model(self.to_glib_none().0, model.to_glib_none().0);
        }
    }

    pub fn set_reorderable(&self, reorderable: bool) {
        unsafe {
            ffi::gtk_tree_view_set_reorderable(self.to_glib_none().0, reorderable.to_glib());
        }
    }

    //pub fn set_row_separator_func(&self, func: /*Unknown conversion*//*Unimplemented*/TreeViewRowSeparatorFunc, data: /*Unimplemented*/Option<Fundamental: Pointer>, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_tree_view_set_row_separator_func() }
    //}

    pub fn set_rubber_banding(&self, enable: bool) {
        unsafe {
            ffi::gtk_tree_view_set_rubber_banding(self.to_glib_none().0, enable.to_glib());
        }
    }

    pub fn set_rules_hint(&self, setting: bool) {
        unsafe {
            ffi::gtk_tree_view_set_rules_hint(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_search_column(&self, column: i32) {
        unsafe {
            ffi::gtk_tree_view_set_search_column(self.to_glib_none().0, column);
        }
    }

    pub fn set_search_entry<T: IsA<Entry>>(&self, entry: Option<&T>) {
        unsafe {
            ffi::gtk_tree_view_set_search_entry(self.to_glib_none().0, entry.to_glib_none().0);
        }
    }

    //pub fn set_search_equal_func(&self, search_equal_func: /*Unknown conversion*//*Unimplemented*/TreeViewSearchEqualFunc, search_user_data: /*Unimplemented*/Option<Fundamental: Pointer>, search_destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_tree_view_set_search_equal_func() }
    //}

    //pub fn set_search_position_func(&self, func: /*Unknown conversion*//*Unimplemented*/TreeViewSearchPositionFunc, data: /*Unimplemented*/Option<Fundamental: Pointer>, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_tree_view_set_search_position_func() }
    //}

    pub fn set_show_expanders(&self, enabled: bool) {
        unsafe {
            ffi::gtk_tree_view_set_show_expanders(self.to_glib_none().0, enabled.to_glib());
        }
    }

    pub fn set_tooltip_cell<T: IsA<CellRenderer>>(&self, tooltip: &Tooltip, path: Option<&TreePath>, column: Option<&TreeViewColumn>, cell: Option<&T>) {
        unsafe {
            ffi::gtk_tree_view_set_tooltip_cell(self.to_glib_none().0, tooltip.to_glib_none().0, mut_override(path.to_glib_none().0), column.to_glib_none().0, cell.to_glib_none().0);
        }
    }

    pub fn set_tooltip_column(&self, column: i32) {
        unsafe {
            ffi::gtk_tree_view_set_tooltip_column(self.to_glib_none().0, column);
        }
    }

    pub fn set_tooltip_row(&self, tooltip: &Tooltip, path: &TreePath) {
        unsafe {
            ffi::gtk_tree_view_set_tooltip_row(self.to_glib_none().0, tooltip.to_glib_none().0, mut_override(path.to_glib_none().0));
        }
    }

    pub fn unset_rows_drag_dest(&self) {
        unsafe {
            ffi::gtk_tree_view_unset_rows_drag_dest(self.to_glib_none().0);
        }
    }

    pub fn unset_rows_drag_source(&self) {
        unsafe {
            ffi::gtk_tree_view_unset_rows_drag_source(self.to_glib_none().0);
        }
    }

    pub fn get_property_enable_grid_lines(&self) -> TreeViewGridLines {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "enable-grid-lines".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    pub fn set_property_enable_grid_lines(&self, enable_grid_lines: TreeViewGridLines) {
        let enable_grid_lines = enable_grid_lines.to_glib() as i32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "enable-grid-lines".to_glib_none().0, Value::from(&enable_grid_lines).to_glib_none().0);
        }
    }

    pub fn set_property_ubuntu_almost_fixed_height_mode(&self, ubuntu_almost_fixed_height_mode: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "ubuntu-almost-fixed-height-mode".to_glib_none().0, Value::from(&ubuntu_almost_fixed_height_mode).to_glib_none().0);
        }
    }

    pub fn connect_columns_changed<F: Fn(&TreeView) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TreeView) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "columns-changed",
                transmute(columns_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_cursor_changed<F: Fn(&TreeView) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TreeView) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "cursor-changed",
                transmute(cursor_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_expand_collapse_cursor_row<F: Fn(&TreeView, bool, bool, bool) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TreeView, bool, bool, bool) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "expand-collapse-cursor-row",
                transmute(expand_collapse_cursor_row_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_move_cursor<F: Fn(&TreeView, MovementStep, i32) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TreeView, MovementStep, i32) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-cursor",
                transmute(move_cursor_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_row_activated<F: Fn(&TreeView, &TreePath, &TreeViewColumn) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TreeView, &TreePath, &TreeViewColumn) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "row-activated",
                transmute(row_activated_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_row_collapsed<F: Fn(&TreeView, &TreeIter, &TreePath) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TreeView, &TreeIter, &TreePath) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "row-collapsed",
                transmute(row_collapsed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_row_expanded<F: Fn(&TreeView, &TreeIter, &TreePath) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TreeView, &TreeIter, &TreePath) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "row-expanded",
                transmute(row_expanded_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_select_all<F: Fn(&TreeView) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TreeView) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "select-all",
                transmute(select_all_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_select_cursor_parent<F: Fn(&TreeView) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TreeView) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "select-cursor-parent",
                transmute(select_cursor_parent_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_select_cursor_row<F: Fn(&TreeView, bool) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TreeView, bool) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "select-cursor-row",
                transmute(select_cursor_row_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_start_interactive_search<F: Fn(&TreeView) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TreeView) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "start-interactive-search",
                transmute(start_interactive_search_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_test_collapse_row<F: Fn(&TreeView, &TreeIter, &TreePath) -> Inhibit + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TreeView, &TreeIter, &TreePath) -> Inhibit + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "test-collapse-row",
                transmute(test_collapse_row_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_test_expand_row<F: Fn(&TreeView, &TreeIter, &TreePath) -> Inhibit + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TreeView, &TreeIter, &TreePath) -> Inhibit + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "test-expand-row",
                transmute(test_expand_row_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_toggle_cursor_row<F: Fn(&TreeView) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TreeView) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "toggle-cursor-row",
                transmute(toggle_cursor_row_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_unselect_all<F: Fn(&TreeView) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TreeView) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "unselect-all",
                transmute(unselect_all_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn columns_changed_trampoline(this: *mut ffi::GtkTreeView, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TreeView) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn cursor_changed_trampoline(this: *mut ffi::GtkTreeView, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TreeView) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn expand_collapse_cursor_row_trampoline(this: *mut ffi::GtkTreeView, object: glib_ffi::gboolean, p0: glib_ffi::gboolean, p1: glib_ffi::gboolean, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&TreeView, bool, bool, bool) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this), from_glib(object), from_glib(p0), from_glib(p1)).to_glib()
}

unsafe extern "C" fn move_cursor_trampoline(this: *mut ffi::GtkTreeView, step: ffi::GtkMovementStep, direction: libc::c_int, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&TreeView, MovementStep, i32) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this), from_glib(step), direction).to_glib()
}

unsafe extern "C" fn row_activated_trampoline(this: *mut ffi::GtkTreeView, path: *mut ffi::GtkTreePath, column: *mut ffi::GtkTreeViewColumn, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TreeView, &TreePath, &TreeViewColumn) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_borrow(path), &from_glib_none(column))
}

unsafe extern "C" fn row_collapsed_trampoline(this: *mut ffi::GtkTreeView, iter: *mut ffi::GtkTreeIter, path: *mut ffi::GtkTreePath, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TreeView, &TreeIter, &TreePath) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_borrow(iter), &from_glib_borrow(path))
}

unsafe extern "C" fn row_expanded_trampoline(this: *mut ffi::GtkTreeView, iter: *mut ffi::GtkTreeIter, path: *mut ffi::GtkTreePath, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TreeView, &TreeIter, &TreePath) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_borrow(iter), &from_glib_borrow(path))
}

unsafe extern "C" fn select_all_trampoline(this: *mut ffi::GtkTreeView, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&TreeView) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this)).to_glib()
}

unsafe extern "C" fn select_cursor_parent_trampoline(this: *mut ffi::GtkTreeView, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&TreeView) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this)).to_glib()
}

unsafe extern "C" fn select_cursor_row_trampoline(this: *mut ffi::GtkTreeView, object: glib_ffi::gboolean, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&TreeView, bool) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this), from_glib(object)).to_glib()
}

unsafe extern "C" fn start_interactive_search_trampoline(this: *mut ffi::GtkTreeView, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&TreeView) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this)).to_glib()
}

unsafe extern "C" fn test_collapse_row_trampoline(this: *mut ffi::GtkTreeView, iter: *mut ffi::GtkTreeIter, path: *mut ffi::GtkTreePath, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&TreeView, &TreeIter, &TreePath) -> Inhibit + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_borrow(iter), &from_glib_borrow(path)).to_glib()
}

unsafe extern "C" fn test_expand_row_trampoline(this: *mut ffi::GtkTreeView, iter: *mut ffi::GtkTreeIter, path: *mut ffi::GtkTreePath, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&TreeView, &TreeIter, &TreePath) -> Inhibit + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_borrow(iter), &from_glib_borrow(path)).to_glib()
}

unsafe extern "C" fn toggle_cursor_row_trampoline(this: *mut ffi::GtkTreeView, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&TreeView) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this)).to_glib()
}

unsafe extern "C" fn unselect_all_trampoline(this: *mut ffi::GtkTreeView, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&TreeView) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this)).to_glib()
}
