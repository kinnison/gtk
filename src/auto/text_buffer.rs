// This file was generated by gir (b3fbe4b) from gir-files (11e0e6d)
// DO NOT EDIT

use TextChildAnchor;
use TextIter;
use TextMark;
use TextTag;
use TextTagTable;
use ffi;
use gdk_pixbuf;
use glib::translate::*;

glib_wrapper! {
    pub struct TextBuffer(Object<ffi::GtkTextBuffer>);

    match fn {
        get_type => || ffi::gtk_text_buffer_get_type(),
    }
}

impl TextBuffer {
    pub fn new(table: Option<&TextTagTable>) -> TextBuffer {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_text_buffer_new(table.to_glib_none().0))
        }
    }

    pub fn add_mark(&self, mark: &TextMark, where_: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_add_mark(self.to_glib_none().0, mark.to_glib_none().0, where_.to_glib_none().0);
        }
    }

    //pub fn add_selection_clipboard(&self, clipboard: /*Ignored*/&Clipboard) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_add_selection_clipboard() }
    //}

    pub fn apply_tag(&self, tag: &TextTag, start: &TextIter, end: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_apply_tag(self.to_glib_none().0, tag.to_glib_none().0, start.to_glib_none().0, end.to_glib_none().0);
        }
    }

    pub fn apply_tag_by_name(&self, name: &str, start: &TextIter, end: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_apply_tag_by_name(self.to_glib_none().0, name.to_glib_none().0, start.to_glib_none().0, end.to_glib_none().0);
        }
    }

    pub fn backspace(&self, iter: &mut TextIter, interactive: bool, default_editable: bool) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_buffer_backspace(self.to_glib_none().0, iter.to_glib_none_mut().0, interactive.to_glib(), default_editable.to_glib()))
        }
    }

    pub fn begin_user_action(&self) {
        unsafe {
            ffi::gtk_text_buffer_begin_user_action(self.to_glib_none().0);
        }
    }

    //pub fn copy_clipboard(&self, clipboard: /*Ignored*/&Clipboard) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_copy_clipboard() }
    //}

    pub fn create_child_anchor(&self, iter: &mut TextIter) -> Option<TextChildAnchor> {
        unsafe {
            from_glib_none(ffi::gtk_text_buffer_create_child_anchor(self.to_glib_none().0, iter.to_glib_none_mut().0))
        }
    }

    pub fn create_mark(&self, mark_name: Option<&str>, where_: &TextIter, left_gravity: bool) -> Option<TextMark> {
        unsafe {
            from_glib_none(ffi::gtk_text_buffer_create_mark(self.to_glib_none().0, mark_name.to_glib_none().0, where_.to_glib_none().0, left_gravity.to_glib()))
        }
    }

    //pub fn create_tag(&self, tag_name: Option<&str>, first_property_name: Option<&str>, : /*Unknown conversion*/Fundamental: VarArgs) -> Option<TextTag> {
    //    unsafe { TODO: call ffi::gtk_text_buffer_create_tag() }
    //}

    //pub fn cut_clipboard(&self, clipboard: /*Ignored*/&Clipboard, default_editable: bool) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_cut_clipboard() }
    //}

    pub fn delete(&self, start: &mut TextIter, end: &mut TextIter) {
        unsafe {
            ffi::gtk_text_buffer_delete(self.to_glib_none().0, start.to_glib_none_mut().0, end.to_glib_none_mut().0);
        }
    }

    pub fn delete_interactive(&self, start_iter: &mut TextIter, end_iter: &mut TextIter, default_editable: bool) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_buffer_delete_interactive(self.to_glib_none().0, start_iter.to_glib_none_mut().0, end_iter.to_glib_none_mut().0, default_editable.to_glib()))
        }
    }

    pub fn delete_mark(&self, mark: &TextMark) {
        unsafe {
            ffi::gtk_text_buffer_delete_mark(self.to_glib_none().0, mark.to_glib_none().0);
        }
    }

    pub fn delete_mark_by_name(&self, name: &str) {
        unsafe {
            ffi::gtk_text_buffer_delete_mark_by_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    pub fn delete_selection(&self, interactive: bool, default_editable: bool) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_buffer_delete_selection(self.to_glib_none().0, interactive.to_glib(), default_editable.to_glib()))
        }
    }

    //pub fn deserialize(&self, content_buffer: &TextBuffer, format: &Atom, iter: &mut TextIter, data: &Unknown rust type: "CArray TypeId { ns_id: 0, id: 3 }", length: Fundamental: Size, error: Option<glib::Error>) -> bool {
    //    unsafe { TODO: call ffi::gtk_text_buffer_deserialize() }
    //}

    //pub fn deserialize_get_can_create_tags(&self, format: &Atom) -> bool {
    //    unsafe { TODO: call ffi::gtk_text_buffer_deserialize_get_can_create_tags() }
    //}

    //pub fn deserialize_set_can_create_tags(&self, format: &Atom, can_create_tags: bool) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_deserialize_set_can_create_tags() }
    //}

    pub fn end_user_action(&self) {
        unsafe {
            ffi::gtk_text_buffer_end_user_action(self.to_glib_none().0);
        }
    }

    pub fn get_bounds(&self) -> (TextIter, TextIter) {
        unsafe {
            let mut start = TextIter::uninitialized();
            let mut end = TextIter::uninitialized();
            ffi::gtk_text_buffer_get_bounds(self.to_glib_none().0, start.to_glib_none_mut().0, end.to_glib_none_mut().0);
            (start, end)
        }
    }

    pub fn get_char_count(&self) -> i32 {
        unsafe {
            ffi::gtk_text_buffer_get_char_count(self.to_glib_none().0)
        }
    }

    //pub fn get_copy_target_list(&self) -> /*Ignored*/TargetList {
    //    unsafe { TODO: call ffi::gtk_text_buffer_get_copy_target_list() }
    //}

    //pub fn get_deserialize_formats(&self) -> (Vec<Atom>, i32) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_get_deserialize_formats() }
    //}

    pub fn get_end_iter(&self) -> TextIter {
        unsafe {
            let mut iter = TextIter::uninitialized();
            ffi::gtk_text_buffer_get_end_iter(self.to_glib_none().0, iter.to_glib_none_mut().0);
            iter
        }
    }

    pub fn get_has_selection(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_buffer_get_has_selection(self.to_glib_none().0))
        }
    }

    pub fn get_insert(&self) -> Option<TextMark> {
        unsafe {
            from_glib_none(ffi::gtk_text_buffer_get_insert(self.to_glib_none().0))
        }
    }

    pub fn get_iter_at_child_anchor(&self, anchor: &TextChildAnchor) -> TextIter {
        unsafe {
            let mut iter = TextIter::uninitialized();
            ffi::gtk_text_buffer_get_iter_at_child_anchor(self.to_glib_none().0, iter.to_glib_none_mut().0, anchor.to_glib_none().0);
            iter
        }
    }

    pub fn get_iter_at_line(&self, line_number: i32) -> TextIter {
        unsafe {
            let mut iter = TextIter::uninitialized();
            ffi::gtk_text_buffer_get_iter_at_line(self.to_glib_none().0, iter.to_glib_none_mut().0, line_number);
            iter
        }
    }

    pub fn get_iter_at_line_index(&self, line_number: i32, byte_index: i32) -> TextIter {
        unsafe {
            let mut iter = TextIter::uninitialized();
            ffi::gtk_text_buffer_get_iter_at_line_index(self.to_glib_none().0, iter.to_glib_none_mut().0, line_number, byte_index);
            iter
        }
    }

    pub fn get_iter_at_line_offset(&self, line_number: i32, char_offset: i32) -> TextIter {
        unsafe {
            let mut iter = TextIter::uninitialized();
            ffi::gtk_text_buffer_get_iter_at_line_offset(self.to_glib_none().0, iter.to_glib_none_mut().0, line_number, char_offset);
            iter
        }
    }

    pub fn get_iter_at_mark(&self, mark: &TextMark) -> TextIter {
        unsafe {
            let mut iter = TextIter::uninitialized();
            ffi::gtk_text_buffer_get_iter_at_mark(self.to_glib_none().0, iter.to_glib_none_mut().0, mark.to_glib_none().0);
            iter
        }
    }

    pub fn get_iter_at_offset(&self, char_offset: i32) -> TextIter {
        unsafe {
            let mut iter = TextIter::uninitialized();
            ffi::gtk_text_buffer_get_iter_at_offset(self.to_glib_none().0, iter.to_glib_none_mut().0, char_offset);
            iter
        }
    }

    pub fn get_line_count(&self) -> i32 {
        unsafe {
            ffi::gtk_text_buffer_get_line_count(self.to_glib_none().0)
        }
    }

    pub fn get_mark(&self, name: &str) -> Option<TextMark> {
        unsafe {
            from_glib_none(ffi::gtk_text_buffer_get_mark(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    pub fn get_modified(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_buffer_get_modified(self.to_glib_none().0))
        }
    }

    //pub fn get_paste_target_list(&self) -> /*Ignored*/TargetList {
    //    unsafe { TODO: call ffi::gtk_text_buffer_get_paste_target_list() }
    //}

    pub fn get_selection_bound(&self) -> Option<TextMark> {
        unsafe {
            from_glib_none(ffi::gtk_text_buffer_get_selection_bound(self.to_glib_none().0))
        }
    }

    pub fn get_selection_bounds(&self) -> Option<(TextIter, TextIter)> {
        unsafe {
            let mut start = TextIter::uninitialized();
            let mut end = TextIter::uninitialized();
            let ret = from_glib(ffi::gtk_text_buffer_get_selection_bounds(self.to_glib_none().0, start.to_glib_none_mut().0, end.to_glib_none_mut().0));
            if ret { Some((start, end)) } else { None }
        }
    }

    //pub fn get_serialize_formats(&self) -> (Vec<Atom>, i32) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_get_serialize_formats() }
    //}

    pub fn get_slice(&self, start: &TextIter, end: &TextIter, include_hidden_chars: bool) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_text_buffer_get_slice(self.to_glib_none().0, start.to_glib_none().0, end.to_glib_none().0, include_hidden_chars.to_glib()))
        }
    }

    pub fn get_start_iter(&self) -> TextIter {
        unsafe {
            let mut iter = TextIter::uninitialized();
            ffi::gtk_text_buffer_get_start_iter(self.to_glib_none().0, iter.to_glib_none_mut().0);
            iter
        }
    }

    pub fn get_tag_table(&self) -> Option<TextTagTable> {
        unsafe {
            from_glib_none(ffi::gtk_text_buffer_get_tag_table(self.to_glib_none().0))
        }
    }

    pub fn get_text(&self, start: &TextIter, end: &TextIter, include_hidden_chars: bool) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_text_buffer_get_text(self.to_glib_none().0, start.to_glib_none().0, end.to_glib_none().0, include_hidden_chars.to_glib()))
        }
    }

    pub fn insert(&self, iter: &mut TextIter, text: &str, len: i32) {
        unsafe {
            ffi::gtk_text_buffer_insert(self.to_glib_none().0, iter.to_glib_none_mut().0, text.to_glib_none().0, len);
        }
    }

    pub fn insert_at_cursor(&self, text: &str, len: i32) {
        unsafe {
            ffi::gtk_text_buffer_insert_at_cursor(self.to_glib_none().0, text.to_glib_none().0, len);
        }
    }

    pub fn insert_child_anchor(&self, iter: &mut TextIter, anchor: &TextChildAnchor) {
        unsafe {
            ffi::gtk_text_buffer_insert_child_anchor(self.to_glib_none().0, iter.to_glib_none_mut().0, anchor.to_glib_none().0);
        }
    }

    pub fn insert_interactive(&self, iter: &mut TextIter, text: &str, len: i32, default_editable: bool) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_buffer_insert_interactive(self.to_glib_none().0, iter.to_glib_none_mut().0, text.to_glib_none().0, len, default_editable.to_glib()))
        }
    }

    pub fn insert_interactive_at_cursor(&self, text: &str, len: i32, default_editable: bool) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_buffer_insert_interactive_at_cursor(self.to_glib_none().0, text.to_glib_none().0, len, default_editable.to_glib()))
        }
    }

    #[cfg(gtk_3_16)]
    pub fn insert_markup(&self, iter: &mut TextIter, markup: &str, len: i32) {
        unsafe {
            ffi::gtk_text_buffer_insert_markup(self.to_glib_none().0, iter.to_glib_none_mut().0, markup.to_glib_none().0, len);
        }
    }

    pub fn insert_pixbuf(&self, iter: &mut TextIter, pixbuf: &gdk_pixbuf::Pixbuf) {
        unsafe {
            ffi::gtk_text_buffer_insert_pixbuf(self.to_glib_none().0, iter.to_glib_none_mut().0, pixbuf.to_glib_none().0);
        }
    }

    pub fn insert_range(&self, iter: &mut TextIter, start: &TextIter, end: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_insert_range(self.to_glib_none().0, iter.to_glib_none_mut().0, start.to_glib_none().0, end.to_glib_none().0);
        }
    }

    pub fn insert_range_interactive(&self, iter: &mut TextIter, start: &TextIter, end: &TextIter, default_editable: bool) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_buffer_insert_range_interactive(self.to_glib_none().0, iter.to_glib_none_mut().0, start.to_glib_none().0, end.to_glib_none().0, default_editable.to_glib()))
        }
    }

    //pub fn insert_with_tags(&self, iter: &mut TextIter, text: &str, len: i32, first_tag: &TextTag, : /*Unknown conversion*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_insert_with_tags() }
    //}

    //pub fn insert_with_tags_by_name(&self, iter: &mut TextIter, text: &str, len: i32, first_tag_name: &str, : /*Unknown conversion*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_insert_with_tags_by_name() }
    //}

    pub fn move_mark(&self, mark: &TextMark, where_: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_move_mark(self.to_glib_none().0, mark.to_glib_none().0, where_.to_glib_none().0);
        }
    }

    pub fn move_mark_by_name(&self, name: &str, where_: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_move_mark_by_name(self.to_glib_none().0, name.to_glib_none().0, where_.to_glib_none().0);
        }
    }

    //pub fn paste_clipboard(&self, clipboard: /*Ignored*/&Clipboard, override_location: Option<&mut TextIter>, default_editable: bool) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_paste_clipboard() }
    //}

    pub fn place_cursor(&self, where_: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_place_cursor(self.to_glib_none().0, where_.to_glib_none().0);
        }
    }

    //pub fn register_deserialize_format(&self, mime_type: &str, function: /*Unknown conversion*/Unknown rust type: "TextBufferDeserializeFunc", user_data: Fundamental: Pointer, user_data_destroy: /*Unknown conversion*/Unknown rust type: "DestroyNotify") -> Atom {
    //    unsafe { TODO: call ffi::gtk_text_buffer_register_deserialize_format() }
    //}

    //pub fn register_deserialize_tagset(&self, tagset_name: Option<&str>) -> Atom {
    //    unsafe { TODO: call ffi::gtk_text_buffer_register_deserialize_tagset() }
    //}

    //pub fn register_serialize_format(&self, mime_type: &str, function: /*Unknown conversion*/Unknown rust type: "TextBufferSerializeFunc", user_data: Fundamental: Pointer, user_data_destroy: /*Unknown conversion*/Unknown rust type: "DestroyNotify") -> Atom {
    //    unsafe { TODO: call ffi::gtk_text_buffer_register_serialize_format() }
    //}

    //pub fn register_serialize_tagset(&self, tagset_name: Option<&str>) -> Atom {
    //    unsafe { TODO: call ffi::gtk_text_buffer_register_serialize_tagset() }
    //}

    pub fn remove_all_tags(&self, start: &TextIter, end: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_remove_all_tags(self.to_glib_none().0, start.to_glib_none().0, end.to_glib_none().0);
        }
    }

    //pub fn remove_selection_clipboard(&self, clipboard: /*Ignored*/&Clipboard) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_remove_selection_clipboard() }
    //}

    pub fn remove_tag(&self, tag: &TextTag, start: &TextIter, end: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_remove_tag(self.to_glib_none().0, tag.to_glib_none().0, start.to_glib_none().0, end.to_glib_none().0);
        }
    }

    pub fn remove_tag_by_name(&self, name: &str, start: &TextIter, end: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_remove_tag_by_name(self.to_glib_none().0, name.to_glib_none().0, start.to_glib_none().0, end.to_glib_none().0);
        }
    }

    pub fn select_range(&self, ins: &TextIter, bound: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_select_range(self.to_glib_none().0, ins.to_glib_none().0, bound.to_glib_none().0);
        }
    }

    //pub fn serialize(&self, content_buffer: &TextBuffer, format: &Atom, start: &TextIter, end: &TextIter) -> (Unknown rust type: "CArray TypeId { ns_id: 0, id: 3 }", Fundamental: Size) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_serialize() }
    //}

    pub fn set_modified(&self, setting: bool) {
        unsafe {
            ffi::gtk_text_buffer_set_modified(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_text(&self, text: &str, len: i32) {
        unsafe {
            ffi::gtk_text_buffer_set_text(self.to_glib_none().0, text.to_glib_none().0, len);
        }
    }

    //pub fn unregister_deserialize_format(&self, format: &Atom) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_unregister_deserialize_format() }
    //}

    //pub fn unregister_serialize_format(&self, format: &Atom) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_unregister_serialize_format() }
    //}

}
