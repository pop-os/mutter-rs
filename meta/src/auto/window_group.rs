// Generated by gir (https://github.com/gtk-rs/gir @ 45cd7bc)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 8e47c67)
// from mutter-gir-files
// DO NOT EDIT

use std::fmt;

glib::wrapper! {
    #[doc(alias = "MetaWindowGroup")]
    pub struct WindowGroup(Object<ffi::MetaWindowGroup, ffi::MetaWindowGroupClass>) @extends clutter::Actor, @implements clutter::Animatable, clutter::Container, clutter::Scriptable;

    match fn {
        type_ => || ffi::meta_window_group_get_type(),
    }
}

impl WindowGroup {}

impl fmt::Display for WindowGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WindowGroup")
    }
}
