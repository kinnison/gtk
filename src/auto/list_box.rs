// This file was generated by gir (b3fbe4b) from gir-files (11e0e6d)
// DO NOT EDIT

#[cfg(gtk_3_10)]
use Adjustment;
use Buildable;
use Container;
#[cfg(gtk_3_10)]
use ListBoxRow;
#[cfg(gtk_3_10)]
use SelectionMode;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::Upcast;
use glib::translate::*;

glib_wrapper! {
    pub struct ListBox(Object<ffi::GtkListBox>): Widget, Container, Buildable;

    match fn {
        get_type => || ffi::gtk_list_box_get_type(),
    }
}

impl ListBox {
    #[cfg(gtk_3_10)]
    pub fn new() -> ListBox {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_list_box_new()).downcast_unchecked()
        }
    }

    //#[cfg(gtk_3_16)]
    //pub fn bind_model<T: Upcast</*Ignored*/gio::ListModel>>(&self, model: Option<&T>, create_widget_func: /*Unknown conversion*/Unknown rust type: "ListBoxCreateWidgetFunc", user_data: Fundamental: Pointer, user_data_free_func: /*Unknown conversion*/Unknown rust type: "DestroyNotify") {
    //    unsafe { TODO: call ffi::gtk_list_box_bind_model() }
    //}

    #[cfg(gtk_3_10)]
    pub fn drag_highlight_row(&self, row: &ListBoxRow) {
        unsafe {
            ffi::gtk_list_box_drag_highlight_row(self.to_glib_none().0, row.to_glib_none().0);
        }
    }

    #[cfg(gtk_3_10)]
    pub fn drag_unhighlight_row(&self) {
        unsafe {
            ffi::gtk_list_box_drag_unhighlight_row(self.to_glib_none().0);
        }
    }

    #[cfg(gtk_3_10)]
    pub fn get_activate_on_single_click(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_box_get_activate_on_single_click(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_10)]
    pub fn get_adjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_get_adjustment(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_10)]
    pub fn get_row_at_index(&self, index_: i32) -> Option<ListBoxRow> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_get_row_at_index(self.to_glib_none().0, index_))
        }
    }

    #[cfg(gtk_3_10)]
    pub fn get_row_at_y(&self, y: i32) -> Option<ListBoxRow> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_get_row_at_y(self.to_glib_none().0, y))
        }
    }

    #[cfg(gtk_3_10)]
    pub fn get_selected_row(&self) -> Option<ListBoxRow> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_get_selected_row(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_14)]
    pub fn get_selected_rows(&self) -> Vec<ListBoxRow> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_list_box_get_selected_rows(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_10)]
    pub fn get_selection_mode(&self) -> SelectionMode {
        unsafe {
            ffi::gtk_list_box_get_selection_mode(self.to_glib_none().0)
        }
    }

    #[cfg(gtk_3_10)]
    pub fn insert<T: Upcast<Widget>>(&self, child: &T, position: i32) {
        unsafe {
            ffi::gtk_list_box_insert(self.to_glib_none().0, child.to_glib_none().0, position);
        }
    }

    #[cfg(gtk_3_10)]
    pub fn invalidate_filter(&self) {
        unsafe {
            ffi::gtk_list_box_invalidate_filter(self.to_glib_none().0);
        }
    }

    #[cfg(gtk_3_10)]
    pub fn invalidate_headers(&self) {
        unsafe {
            ffi::gtk_list_box_invalidate_headers(self.to_glib_none().0);
        }
    }

    #[cfg(gtk_3_10)]
    pub fn invalidate_sort(&self) {
        unsafe {
            ffi::gtk_list_box_invalidate_sort(self.to_glib_none().0);
        }
    }

    #[cfg(gtk_3_10)]
    pub fn prepend<T: Upcast<Widget>>(&self, child: &T) {
        unsafe {
            ffi::gtk_list_box_prepend(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    #[cfg(gtk_3_14)]
    pub fn select_all(&self) {
        unsafe {
            ffi::gtk_list_box_select_all(self.to_glib_none().0);
        }
    }

    #[cfg(gtk_3_10)]
    pub fn select_row(&self, row: Option<&ListBoxRow>) {
        unsafe {
            ffi::gtk_list_box_select_row(self.to_glib_none().0, row.to_glib_none().0);
        }
    }

    //#[cfg(gtk_3_14)]
    //pub fn selected_foreach(&self, func: /*Unknown conversion*/Unknown rust type: "ListBoxForeachFunc", data: Fundamental: Pointer) {
    //    unsafe { TODO: call ffi::gtk_list_box_selected_foreach() }
    //}

    #[cfg(gtk_3_10)]
    pub fn set_activate_on_single_click(&self, single: bool) {
        unsafe {
            ffi::gtk_list_box_set_activate_on_single_click(self.to_glib_none().0, single.to_glib());
        }
    }

    #[cfg(gtk_3_10)]
    pub fn set_adjustment(&self, adjustment: Option<&Adjustment>) {
        unsafe {
            ffi::gtk_list_box_set_adjustment(self.to_glib_none().0, adjustment.to_glib_none().0);
        }
    }

    //#[cfg(gtk_3_10)]
    //pub fn set_filter_func(&self, filter_func: /*Unknown conversion*/Unknown rust type: "ListBoxFilterFunc", user_data: Fundamental: Pointer, destroy: /*Unknown conversion*/Unknown rust type: "DestroyNotify") {
    //    unsafe { TODO: call ffi::gtk_list_box_set_filter_func() }
    //}

    //#[cfg(gtk_3_10)]
    //pub fn set_header_func(&self, update_header: /*Unknown conversion*/Unknown rust type: "ListBoxUpdateHeaderFunc", user_data: Fundamental: Pointer, destroy: /*Unknown conversion*/Unknown rust type: "DestroyNotify") {
    //    unsafe { TODO: call ffi::gtk_list_box_set_header_func() }
    //}

    #[cfg(gtk_3_10)]
    pub fn set_placeholder<T: Upcast<Widget>>(&self, placeholder: Option<&T>) {
        unsafe {
            ffi::gtk_list_box_set_placeholder(self.to_glib_none().0, placeholder.to_glib_none().0);
        }
    }

    #[cfg(gtk_3_10)]
    pub fn set_selection_mode(&self, mode: SelectionMode) {
        unsafe {
            ffi::gtk_list_box_set_selection_mode(self.to_glib_none().0, mode);
        }
    }

    //#[cfg(gtk_3_10)]
    //pub fn set_sort_func(&self, sort_func: /*Unknown conversion*/Unknown rust type: "ListBoxSortFunc", user_data: Fundamental: Pointer, destroy: /*Unknown conversion*/Unknown rust type: "DestroyNotify") {
    //    unsafe { TODO: call ffi::gtk_list_box_set_sort_func() }
    //}

    #[cfg(gtk_3_14)]
    pub fn unselect_all(&self) {
        unsafe {
            ffi::gtk_list_box_unselect_all(self.to_glib_none().0);
        }
    }

    #[cfg(gtk_3_14)]
    pub fn unselect_row(&self, row: &ListBoxRow) {
        unsafe {
            ffi::gtk_list_box_unselect_row(self.to_glib_none().0, row.to_glib_none().0);
        }
    }

}
