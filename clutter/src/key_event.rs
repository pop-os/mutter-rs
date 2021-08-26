use glib::translate::{
    Borrowed,
    FromGlib,
    FromGlibPtrBorrow,
    FromGlibPtrNone,
};
use libc::c_uint;

use crate::{
    Actor,
    EventFlags,
    EventType,
    InputDevice,
    ModifierType,
    Stage,
    ffi::ClutterKeyEvent,
};

/// Key event
pub struct KeyEvent {
    pub type_: EventType,
    pub time: u32,
    pub flags: EventFlags,
    pub stage: Stage,
    pub source: Actor,
    pub modifier_state: ModifierType,
    pub keyval: c_uint,
    pub hardware_keycode: u16,
    pub unicode_value: u32,
    pub evdev_code: u32,
    pub device: InputDevice,
}

impl FromGlibPtrBorrow<*mut ClutterKeyEvent> for KeyEvent {
    unsafe fn from_glib_borrow(ptr: *mut ClutterKeyEvent) -> Borrowed<Self> {
        let glib = *ptr;
        Borrowed::new(KeyEvent {
            type_: EventType::from_glib(glib.type_),
            time: glib.time,
            flags: EventFlags::from_glib(glib.flags),
            stage: Stage::from_glib_none(glib.stage),
            source: Actor::from_glib_none(glib.source),
            modifier_state: ModifierType::from_glib(glib.modifier_state),
            keyval: glib.keyval,
            hardware_keycode: glib.hardware_keycode,
            unicode_value: glib.unicode_value,
            evdev_code: glib.evdev_code,
            device: InputDevice::from_glib_none(glib.device),
        })
    }
}
