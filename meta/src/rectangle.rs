use glib::translate::Uninitialized;
use std::mem;

use crate::Rectangle;

impl Uninitialized for Rectangle {
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}
