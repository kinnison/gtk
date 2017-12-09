// This file was generated by gir (8080733) from gir-files (469db10)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Container;
use ToolButton;
use ToolItem;
use Widget;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct ToggleToolButton(Object<ffi::GtkToggleToolButton, ffi::GtkToggleToolButtonClass>): ToolButton, ToolItem, Bin, Container, Widget, Buildable, Actionable;

    match fn {
        get_type => || ffi::gtk_toggle_tool_button_get_type(),
    }
}

impl ToggleToolButton {
    pub fn new() -> ToggleToolButton {
        assert_initialized_main_thread!();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_toggle_tool_button_new()).downcast_unchecked()
        }
    }

    pub fn new_from_stock(stock_id: &str) -> ToggleToolButton {
        assert_initialized_main_thread!();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_toggle_tool_button_new_from_stock(stock_id.to_glib_none().0)).downcast_unchecked()
        }
    }
}

impl Default for ToggleToolButton {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ToggleToolButtonExt {
    fn get_active(&self) -> bool;

    fn set_active(&self, is_active: bool);

    fn connect_toggled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ToggleToolButton> + IsA<glib::object::Object>> ToggleToolButtonExt for O {
    fn get_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_toggle_tool_button_get_active(self.to_glib_none().0))
        }
    }

    fn set_active(&self, is_active: bool) {
        unsafe {
            ffi::gtk_toggle_tool_button_set_active(self.to_glib_none().0, is_active.to_glib());
        }
    }

    fn connect_toggled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "toggled",
                transmute(toggled_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::active",
                transmute(notify_active_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn toggled_trampoline<P>(this: *mut ffi::GtkToggleToolButton, f: glib_ffi::gpointer)
where P: IsA<ToggleToolButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ToggleToolButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_active_trampoline<P>(this: *mut ffi::GtkToggleToolButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ToggleToolButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ToggleToolButton::from_glib_borrow(this).downcast_unchecked())
}
