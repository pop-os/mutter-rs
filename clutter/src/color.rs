use ffi::ClutterColor;
use glib::translate::{
    FromGlibPtrNone,
    Uninitialized,
};

use crate::Color;

impl Uninitialized for Color {
    unsafe fn uninitialized() -> Self {
        //TODO: is this implementation correct?
        let color = ClutterColor {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 0,
        };
        Self::from_glib_none(&color as *const ClutterColor)
    }
}
