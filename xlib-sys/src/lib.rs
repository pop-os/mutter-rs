// Generated by gir (https://github.com/gtk-rs/gir @ 5bbf6cb)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 8e47c67)
// from mutter-gir-files
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(clippy::approx_constant, clippy::type_complexity, clippy::unreadable_literal, clippy::upper_case_acronyms)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

use glib_sys as glib;

#[allow(unused_imports)]
use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong,
    c_void, size_t, ssize_t, intptr_t, uintptr_t, time_t, FILE};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Aliases
pub type Atom = c_ulong;
pub type Colormap = c_ulong;
pub type Cursor = c_ulong;
pub type Drawable = c_ulong;
pub type GC = gpointer;
pub type KeyCode = u8;
pub type KeySym = c_ulong;
pub type Picture = c_ulong;
pub type Pixmap = c_ulong;
pub type Time = c_ulong;
pub type VisualID = c_ulong;
pub type Window = c_ulong;
pub type XID = c_ulong;

// Unions
#[repr(C)]
pub struct XEvent(c_void);

impl ::std::fmt::Debug for XEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("XEvent @ {:p}", self))
         .finish()
    }
}

// Records
#[repr(C)]
pub struct Display(c_void);

impl ::std::fmt::Debug for Display {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("Display @ {:p}", self))
         .finish()
    }
}

#[repr(C)]
pub struct Screen(c_void);

impl ::std::fmt::Debug for Screen {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("Screen @ {:p}", self))
         .finish()
    }
}

#[repr(C)]
pub struct Visual(c_void);

impl ::std::fmt::Debug for Visual {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("Visual @ {:p}", self))
         .finish()
    }
}

#[repr(C)]
pub struct XConfigureEvent(c_void);

impl ::std::fmt::Debug for XConfigureEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("XConfigureEvent @ {:p}", self))
         .finish()
    }
}

#[repr(C)]
pub struct XFontStruct(c_void);

impl ::std::fmt::Debug for XFontStruct {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("XFontStruct @ {:p}", self))
         .finish()
    }
}

#[repr(C)]
pub struct XImage(c_void);

impl ::std::fmt::Debug for XImage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("XImage @ {:p}", self))
         .finish()
    }
}

#[repr(C)]
pub struct XTrapezoid(c_void);

impl ::std::fmt::Debug for XTrapezoid {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("XTrapezoid @ {:p}", self))
         .finish()
    }
}

#[repr(C)]
pub struct XVisualInfo(c_void);

impl ::std::fmt::Debug for XVisualInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("XVisualInfo @ {:p}", self))
         .finish()
    }
}

#[repr(C)]
pub struct XWindowAttributes(c_void);

impl ::std::fmt::Debug for XWindowAttributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("XWindowAttributes @ {:p}", self))
         .finish()
    }
}

#[link(name = "X11")]
extern "C" {

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn XOpenDisplay();

}