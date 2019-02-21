// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Container;
use RadioButton;
use ToggleToolButton;
use ToolButton;
use ToolItem;
use Widget;
use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct RadioToolButton(Object<ffi::GtkRadioToolButton, ffi::GtkRadioToolButtonClass, RadioToolButtonClass>) @extends ToggleToolButton, ToolButton, ToolItem, Bin, Container, Widget, @implements Buildable, Actionable;

    match fn {
        get_type => || ffi::gtk_radio_tool_button_get_type(),
    }
}

impl RadioToolButton {
    pub fn new_from_widget<P: IsA<RadioToolButton>>(group: &P) -> RadioToolButton {
        skip_assert_initialized!();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_radio_tool_button_new_from_widget(group.as_ref().to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_RADIO_TOOL_BUTTON: Option<&RadioToolButton> = None;

pub trait RadioToolButtonExt: 'static {
    fn get_group(&self) -> Vec<RadioButton>;
}

impl<O: IsA<RadioToolButton>> RadioToolButtonExt for O {
    fn get_group(&self) -> Vec<RadioButton> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_radio_tool_button_get_group(self.as_ref().to_glib_none().0))
        }
    }
}

impl fmt::Display for RadioToolButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RadioToolButton")
    }
}
