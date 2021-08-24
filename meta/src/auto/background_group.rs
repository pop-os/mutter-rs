// Generated by gir (https://github.com/gtk-rs/gir @ 5bbf6cb)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 8e47c67)
// from mutter-gir-files
// DO NOT EDIT

use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "MetaBackgroundGroup")]
    pub struct BackgroundGroup(Object<ffi::MetaBackgroundGroup, ffi::MetaBackgroundGroupClass>) @extends clutter::Actor;

    match fn {
        type_ => || ffi::meta_background_group_get_type(),
    }
}

impl BackgroundGroup {
    #[doc(alias = "meta_background_group_new")]
    pub fn new() -> BackgroundGroup {
        unsafe {
            clutter::Actor::from_glib_none(ffi::meta_background_group_new()).unsafe_cast()
        }
    }
}

impl Default for BackgroundGroup {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_BACKGROUND_GROUP: Option<&BackgroundGroup> = None;

impl fmt::Display for BackgroundGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("BackgroundGroup")
    }
}