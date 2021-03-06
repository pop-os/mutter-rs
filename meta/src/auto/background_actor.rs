// Generated by gir (https://github.com/gtk-rs/gir @ 45cd7bc)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 8e47c67)
// from mutter-gir-files
// DO NOT EDIT

use crate::Display;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::translate::*;
use glib::StaticType;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "MetaBackgroundActor")]
    pub struct BackgroundActor(Object<ffi::MetaBackgroundActor, ffi::MetaBackgroundActorClass>) @extends clutter::Actor, @implements clutter::Animatable, clutter::Container, clutter::Scriptable;

    match fn {
        type_ => || ffi::meta_background_actor_get_type(),
    }
}

impl BackgroundActor {
    /// Creates a new actor to draw the background for the given monitor.
    /// ## `monitor`
    /// Index of the monitor for which to draw the background
    ///
    /// # Returns
    ///
    /// the newly created background actor
    #[doc(alias = "meta_background_actor_new")]
    pub fn new(display: &Display, monitor: i32) -> BackgroundActor {
        unsafe {
            clutter::Actor::from_glib_none(ffi::meta_background_actor_new(display.to_glib_none().0, monitor)).unsafe_cast()
        }
    }

    #[doc(alias = "meta-display")]
    pub fn meta_display(&self) -> Option<Display> {
        unsafe {
            let mut value = glib::Value::from_type(<Display as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.as_ptr() as *mut glib::gobject_ffi::GObject, b"meta-display\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `meta-display` getter")
        }
    }

    pub fn monitor(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.as_ptr() as *mut glib::gobject_ffi::GObject, b"monitor\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `monitor` getter")
        }
    }
}

impl fmt::Display for BackgroundActor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("BackgroundActor")
    }
}
