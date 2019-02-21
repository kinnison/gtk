// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use IconSize;
use Orientation;
use ReliefStyle;
use SizeGroup;
use ToolbarStyle;
use Widget;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use pango;
use std::fmt;

glib_wrapper! {
    pub struct ToolShell(Interface<ffi::GtkToolShell>) @requires Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_tool_shell_get_type(),
    }
}

pub const NONE_TOOL_SHELL: Option<&ToolShell> = None;

pub trait ToolShellExt: 'static {
    fn get_ellipsize_mode(&self) -> pango::EllipsizeMode;

    fn get_icon_size(&self) -> IconSize;

    fn get_orientation(&self) -> Orientation;

    fn get_relief_style(&self) -> ReliefStyle;

    fn get_style(&self) -> ToolbarStyle;

    fn get_text_alignment(&self) -> f32;

    fn get_text_orientation(&self) -> Orientation;

    fn get_text_size_group(&self) -> Option<SizeGroup>;

    fn rebuild_menu(&self);
}

impl<O: IsA<ToolShell>> ToolShellExt for O {
    fn get_ellipsize_mode(&self) -> pango::EllipsizeMode {
        unsafe {
            from_glib(ffi::gtk_tool_shell_get_ellipsize_mode(self.as_ref().to_glib_none().0))
        }
    }

    fn get_icon_size(&self) -> IconSize {
        unsafe {
            from_glib(ffi::gtk_tool_shell_get_icon_size(self.as_ref().to_glib_none().0))
        }
    }

    fn get_orientation(&self) -> Orientation {
        unsafe {
            from_glib(ffi::gtk_tool_shell_get_orientation(self.as_ref().to_glib_none().0))
        }
    }

    fn get_relief_style(&self) -> ReliefStyle {
        unsafe {
            from_glib(ffi::gtk_tool_shell_get_relief_style(self.as_ref().to_glib_none().0))
        }
    }

    fn get_style(&self) -> ToolbarStyle {
        unsafe {
            from_glib(ffi::gtk_tool_shell_get_style(self.as_ref().to_glib_none().0))
        }
    }

    fn get_text_alignment(&self) -> f32 {
        unsafe {
            ffi::gtk_tool_shell_get_text_alignment(self.as_ref().to_glib_none().0)
        }
    }

    fn get_text_orientation(&self) -> Orientation {
        unsafe {
            from_glib(ffi::gtk_tool_shell_get_text_orientation(self.as_ref().to_glib_none().0))
        }
    }

    fn get_text_size_group(&self) -> Option<SizeGroup> {
        unsafe {
            from_glib_none(ffi::gtk_tool_shell_get_text_size_group(self.as_ref().to_glib_none().0))
        }
    }

    fn rebuild_menu(&self) {
        unsafe {
            ffi::gtk_tool_shell_rebuild_menu(self.as_ref().to_glib_none().0);
        }
    }
}

impl fmt::Display for ToolShell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ToolShell")
    }
}
