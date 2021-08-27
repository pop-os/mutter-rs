// Generated by gir (https://github.com/gtk-rs/gir @ b193568)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 8e47c67)
// from mutter-gir-files
// DO NOT EDIT

use crate::Actor;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "ClutterActorMeta")]
    pub struct ActorMeta(Object<ffi::ClutterActorMeta, ffi::ClutterActorMetaClass>);

    match fn {
        type_ => || ffi::clutter_actor_meta_get_type(),
    }
}

pub const NONE_ACTOR_META: Option<&ActorMeta> = None;

/// Trait containing all [`struct@ActorMeta`] methods.
///
/// # Implementors
///
/// [`Action`][struct@crate::Action], [`ActorMeta`][struct@crate::ActorMeta], [`Constraint`][struct@crate::Constraint]
pub trait ActorMetaExt: 'static {
    /// Retrieves a pointer to the [`Actor`][crate::Actor] that owns `self`
    ///
    /// # Returns
    ///
    /// a pointer to a [`Actor`][crate::Actor] or [`None`]
    #[doc(alias = "clutter_actor_meta_get_actor")]
    #[doc(alias = "get_actor")]
    fn actor(&self) -> Option<Actor>;

    /// Retrieves whether `self` is enabled
    ///
    /// # Returns
    ///
    /// [`true`] if the [`ActorMeta`][crate::ActorMeta] instance is enabled
    #[doc(alias = "clutter_actor_meta_get_enabled")]
    #[doc(alias = "get_enabled")]
    fn is_enabled(&self) -> bool;

    /// Retrieves the name set using [`set_name()`][Self::set_name()]
    ///
    /// # Returns
    ///
    /// the name of the [`ActorMeta`][crate::ActorMeta]
    ///  instance, or [`None`] if none was set. The returned string is owned
    ///  by the [`ActorMeta`][crate::ActorMeta] instance and it should not be modified
    ///  or freed
    #[doc(alias = "clutter_actor_meta_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> Option<glib::GString>;

    /// Sets whether `self` should be enabled or not
    /// ## `is_enabled`
    /// whether `self` is enabled
    #[doc(alias = "clutter_actor_meta_set_enabled")]
    fn set_enabled(&self, is_enabled: bool);

    /// Sets the name of `self`
    ///
    /// The name can be used to identify the [`ActorMeta`][crate::ActorMeta] instance
    /// ## `name`
    /// the name of `self`
    #[doc(alias = "clutter_actor_meta_set_name")]
    fn set_name(&self, name: &str);

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_4")))]
    #[doc(alias = "actor")]
    fn connect_actor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_4")))]
    #[doc(alias = "enabled")]
    fn connect_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_4")))]
    #[doc(alias = "name")]
    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ActorMeta>> ActorMetaExt for O {
    fn actor(&self) -> Option<Actor> {
        unsafe {
            from_glib_none(ffi::clutter_actor_meta_get_actor(self.as_ref().to_glib_none().0))
        }
    }

    fn is_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::clutter_actor_meta_get_enabled(self.as_ref().to_glib_none().0))
        }
    }

    fn name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::clutter_actor_meta_get_name(self.as_ref().to_glib_none().0))
        }
    }

    fn set_enabled(&self, is_enabled: bool) {
        unsafe {
            ffi::clutter_actor_meta_set_enabled(self.as_ref().to_glib_none().0, is_enabled.into_glib());
        }
    }

    fn set_name(&self, name: &str) {
        unsafe {
            ffi::clutter_actor_meta_set_name(self.as_ref().to_glib_none().0, name.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_4")))]
    fn connect_actor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_actor_trampoline<P: IsA<ActorMeta>, F: Fn(&P) + 'static>(this: *mut ffi::ClutterActorMeta, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(ActorMeta::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::actor\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_actor_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_4")))]
    fn connect_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_enabled_trampoline<P: IsA<ActorMeta>, F: Fn(&P) + 'static>(this: *mut ffi::ClutterActorMeta, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(ActorMeta::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::enabled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_enabled_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_4")))]
    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P: IsA<ActorMeta>, F: Fn(&P) + 'static>(this: *mut ffi::ClutterActorMeta, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(ActorMeta::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_name_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for ActorMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ActorMeta")
    }
}
