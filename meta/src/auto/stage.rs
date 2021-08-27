// Generated by gir (https://github.com/gtk-rs/gir @ 45cd7bc)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 8e47c67)
// from mutter-gir-files
// DO NOT EDIT

use crate::Display;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "MetaStage")]
    pub struct Stage(Object<ffi::MetaStage, ffi::MetaStageClass>) @extends clutter::Stage, clutter::Actor, @implements clutter::Animatable, clutter::Container, clutter::Scriptable;

    match fn {
        type_ => || ffi::meta_stage_get_type(),
    }
}

impl Stage {
    #[doc(alias = "meta_stage_is_focused")]
    pub fn is_focused(display: &Display) -> bool {
        unsafe {
            from_glib(ffi::meta_stage_is_focused(display.to_glib_none().0))
        }
    }

    #[doc(alias = "actors-painted")]
    pub fn connect_actors_painted<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn actors_painted_trampoline<F: Fn(&Stage) + 'static>(this: *mut ffi::MetaStage, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"actors-painted\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(actors_painted_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Stage")
    }
}
