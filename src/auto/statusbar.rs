// This file was generated by gir (9e3f4cc) from gir-files (71d73f0)
// DO NOT EDIT

use Box;
use Container;
use Orientable;
use Widget;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct Statusbar(Object<ffi::GtkStatusbar>): Box, Container, Widget, Orientable;

    match fn {
        get_type => || ffi::gtk_statusbar_get_type(),
    }
}

impl Statusbar {
    pub fn new() -> Statusbar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_statusbar_new()).downcast_unchecked()
        }
    }
}

pub trait StatusbarExt {
    fn get_context_id(&self, context_description: &str) -> u32;

    fn get_message_area(&self) -> Option<Box>;

    fn pop(&self, context_id: u32);

    fn push(&self, context_id: u32, text: &str) -> u32;

    fn remove(&self, context_id: u32, message_id: u32);

    fn remove_all(&self, context_id: u32);

    fn connect_text_popped<F: Fn(&Self, u32, &str) + 'static>(&self, f: F) -> u64;

    fn connect_text_pushed<F: Fn(&Self, u32, &str) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<Statusbar> + IsA<glib::object::Object>> StatusbarExt for O {
    fn get_context_id(&self, context_description: &str) -> u32 {
        unsafe {
            ffi::gtk_statusbar_get_context_id(self.to_glib_none().0, context_description.to_glib_none().0)
        }
    }

    fn get_message_area(&self) -> Option<Box> {
        unsafe {
            from_glib_none(ffi::gtk_statusbar_get_message_area(self.to_glib_none().0))
        }
    }

    fn pop(&self, context_id: u32) {
        unsafe {
            ffi::gtk_statusbar_pop(self.to_glib_none().0, context_id);
        }
    }

    fn push(&self, context_id: u32, text: &str) -> u32 {
        unsafe {
            ffi::gtk_statusbar_push(self.to_glib_none().0, context_id, text.to_glib_none().0)
        }
    }

    fn remove(&self, context_id: u32, message_id: u32) {
        unsafe {
            ffi::gtk_statusbar_remove(self.to_glib_none().0, context_id, message_id);
        }
    }

    fn remove_all(&self, context_id: u32) {
        unsafe {
            ffi::gtk_statusbar_remove_all(self.to_glib_none().0, context_id);
        }
    }

    fn connect_text_popped<F: Fn(&Self, u32, &str) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, u32, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "text-popped",
                transmute(text_popped_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_text_pushed<F: Fn(&Self, u32, &str) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, u32, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "text-pushed",
                transmute(text_pushed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn text_popped_trampoline<P>(this: *mut ffi::GtkStatusbar, context_id: libc::c_uint, text: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<Statusbar> {
    callback_guard!();
    let f: &Box_<Fn(&P, u32, &str) + 'static> = transmute(f);
    f(&Statusbar::from_glib_none(this).downcast_unchecked(), context_id, &String::from_glib_none(text))
}

unsafe extern "C" fn text_pushed_trampoline<P>(this: *mut ffi::GtkStatusbar, context_id: libc::c_uint, text: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<Statusbar> {
    callback_guard!();
    let f: &Box_<Fn(&P, u32, &str) + 'static> = transmute(f);
    f(&Statusbar::from_glib_none(this).downcast_unchecked(), context_id, &String::from_glib_none(text))
}
