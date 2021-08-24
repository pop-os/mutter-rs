use ffi::MetaRectangle;
use glib::translate::{
    FromGlibPtrNone,
    Uninitialized,
    ToGlibPtr,
};

use crate::Rectangle;

impl Rectangle {
    pub fn x(&self) -> libc::c_int {
        unsafe { (*self.to_glib_none().0).x }
    }

    pub fn y(&self) -> libc::c_int {
        unsafe { (*self.to_glib_none().0).y }
    }

    pub fn width(&self) -> libc::c_int {
        unsafe { (*self.to_glib_none().0).width }
    }

    pub fn height(&self) -> libc::c_int {
        unsafe { (*self.to_glib_none().0).height }
    }
}

impl Uninitialized for Rectangle {
    unsafe fn uninitialized() -> Self {
        //TODO: is this implementation correct?
        let rect = MetaRectangle {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        Self::from_glib_none(&rect as *const MetaRectangle)
    }
}
