// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use Widget;
use ffi;
use gdk;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct CellEditable(Interface<ffi::GtkCellEditable>) @requires Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_cell_editable_get_type(),
    }
}

pub const NONE_CELL_EDITABLE: Option<&CellEditable> = None;

pub trait CellEditableExt: 'static {
    fn editing_done(&self);

    fn remove_widget(&self);

    fn start_editing<'a, P: Into<Option<&'a gdk::Event>>>(&self, event: P);

    fn get_property_editing_canceled(&self) -> bool;

    fn set_property_editing_canceled(&self, editing_canceled: bool);

    fn connect_editing_done<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_remove_widget<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_editing_canceled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CellEditable>> CellEditableExt for O {
    fn editing_done(&self) {
        unsafe {
            ffi::gtk_cell_editable_editing_done(self.as_ref().to_glib_none().0);
        }
    }

    fn remove_widget(&self) {
        unsafe {
            ffi::gtk_cell_editable_remove_widget(self.as_ref().to_glib_none().0);
        }
    }

    fn start_editing<'a, P: Into<Option<&'a gdk::Event>>>(&self, event: P) {
        let event = event.into();
        unsafe {
            ffi::gtk_cell_editable_start_editing(self.as_ref().to_glib_none().0, mut_override(event.to_glib_none().0));
        }
    }

    fn get_property_editing_canceled(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"editing-canceled\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_editing_canceled(&self, editing_canceled: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"editing-canceled\0".as_ptr() as *const _, Value::from(&editing_canceled).to_glib_none().0);
        }
    }

    fn connect_editing_done<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"editing-done\0".as_ptr() as *const _,
                Some(transmute(editing_done_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_remove_widget<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"remove-widget\0".as_ptr() as *const _,
                Some(transmute(remove_widget_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_editing_canceled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::editing-canceled\0".as_ptr() as *const _,
                Some(transmute(notify_editing_canceled_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn editing_done_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellEditable, f: glib_ffi::gpointer)
where P: IsA<CellEditable> {
    let f: &F = transmute(f);
    f(&CellEditable::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn remove_widget_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellEditable, f: glib_ffi::gpointer)
where P: IsA<CellEditable> {
    let f: &F = transmute(f);
    f(&CellEditable::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_editing_canceled_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellEditable, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellEditable> {
    let f: &F = transmute(f);
    f(&CellEditable::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for CellEditable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CellEditable")
    }
}
