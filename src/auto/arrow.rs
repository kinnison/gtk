// This file was generated by gir (14876d3) from gir-files (71d73f0)
// DO NOT EDIT

use ArrowType;
use Misc;
use ShadowType;
use Widget;
use ffi;
use glib::Value;
use glib::object::Downcast;
use glib::translate::*;
use gobject_ffi;
use std::mem::transmute;

glib_wrapper! {
    pub struct Arrow(Object<ffi::GtkArrow>): Misc, Widget;

    match fn {
        get_type => || ffi::gtk_arrow_get_type(),
    }
}

impl Arrow {
    pub fn new(arrow_type: ArrowType, shadow_type: ShadowType) -> Arrow {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_arrow_new(arrow_type.to_glib(), shadow_type.to_glib())).downcast_unchecked()
        }
    }

    pub fn set(&self, arrow_type: ArrowType, shadow_type: ShadowType) {
        unsafe {
            ffi::gtk_arrow_set(self.to_glib_none().0, arrow_type.to_glib(), shadow_type.to_glib());
        }
    }

    pub fn get_property_arrow_type(&self) -> ArrowType {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "arrow-type".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    pub fn set_property_arrow_type(&self, arrow_type: ArrowType) {
        let arrow_type = arrow_type.to_glib() as i32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "arrow-type".to_glib_none().0, Value::from(&arrow_type).to_glib_none().0);
        }
    }

    pub fn get_property_shadow_type(&self) -> ShadowType {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "shadow-type".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    pub fn set_property_shadow_type(&self, shadow_type: ShadowType) {
        let shadow_type = shadow_type.to_glib() as i32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "shadow-type".to_glib_none().0, Value::from(&shadow_type).to_glib_none().0);
        }
    }
}
