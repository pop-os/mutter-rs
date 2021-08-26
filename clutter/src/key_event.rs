use crate::ffi::ClutterKeyEvent;

/// Key event
pub struct KeyEvent(pub ClutterKeyEvent);

impl glib::translate::FromGlibPtrBorrow<*mut ClutterKeyEvent> for KeyEvent {
    unsafe fn from_glib_borrow(ptr: *mut ClutterKeyEvent) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(KeyEvent(*ptr))
    }
}
