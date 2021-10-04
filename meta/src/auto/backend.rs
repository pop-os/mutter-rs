// Generated by gir (https://github.com/gtk-rs/gir @ 45cd7bc)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 8e47c67)
// from mutter-gir-files
// DO NOT EDIT

use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "MetaBackend")]
    pub struct Backend(Object<ffi::MetaBackend, ffi::MetaBackendClass>);

    match fn {
        type_ => || ffi::meta_backend_get_type(),
    }
}

impl Backend {
    //#[doc(alias = "meta_backend_get_dnd")]
    //#[doc(alias = "get_dnd")]
    //pub fn dnd(&self) -> /*Ignored*/Option<Dnd> {
    //    unsafe { TODO: call ffi:meta_backend_get_dnd() }
    //}

    //#[doc(alias = "meta_backend_get_remote_access_controller")]
    //#[doc(alias = "get_remote_access_controller")]
    //pub fn remote_access_controller(&self) -> /*Ignored*/Option<RemoteAccessController> {
    //    unsafe { TODO: call ffi:meta_backend_get_remote_access_controller() }
    //}

    //#[doc(alias = "meta_backend_get_settings")]
    //#[doc(alias = "get_settings")]
    //pub fn settings(&self) -> /*Ignored*/Option<Settings> {
    //    unsafe { TODO: call ffi:meta_backend_get_settings() }
    //}

    /// Gets the global [`clutter::Stage`][crate::clutter::Stage] that's managed by this backend.
    ///
    /// # Returns
    ///
    /// the [`clutter::Stage`][crate::clutter::Stage]
    #[doc(alias = "meta_backend_get_stage")]
    #[doc(alias = "get_stage")]
    pub fn stage(&self) -> Option<clutter::Actor> {
        unsafe {
            from_glib_none(ffi::meta_backend_get_stage(self.to_glib_none().0))
        }
    }

    ///
    /// # Returns
    ///
    /// [`true`] if the rendering is hardware accelerated, otherwise
    /// [`false`].
    #[doc(alias = "meta_backend_is_rendering_hardware_accelerated")]
    pub fn is_rendering_hardware_accelerated(&self) -> bool {
        unsafe {
            from_glib(ffi::meta_backend_is_rendering_hardware_accelerated(self.to_glib_none().0))
        }
    }

    #[doc(alias = "meta_backend_lock_layout_group")]
    pub fn lock_layout_group(&self, idx: u32) {
        unsafe {
            ffi::meta_backend_lock_layout_group(self.to_glib_none().0, idx);
        }
    }

    #[doc(alias = "meta_backend_set_keymap")]
    pub fn set_keymap(&self, layouts: &str, variants: &str, options: &str) {
        unsafe {
            ffi::meta_backend_set_keymap(self.to_glib_none().0, layouts.to_glib_none().0, variants.to_glib_none().0, options.to_glib_none().0);
        }
    }

    #[doc(alias = "meta_backend_set_numlock")]
    pub fn set_numlock(&self, numlock_state: bool) {
        unsafe {
            ffi::meta_backend_set_numlock(self.to_glib_none().0, numlock_state.into_glib());
        }
    }

    //#[doc(alias = "gpu-added")]
    //pub fn connect_gpu_added<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Unimplemented gpu: *.Pointer
    //}

    #[doc(alias = "keymap-changed")]
    pub fn connect_keymap_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn keymap_changed_trampoline<F: Fn(&Backend) + 'static>(this: *mut ffi::MetaBackend, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"keymap-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(keymap_changed_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "keymap-layout-group-changed")]
    pub fn connect_keymap_layout_group_changed<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn keymap_layout_group_changed_trampoline<F: Fn(&Backend, u32) + 'static>(this: *mut ffi::MetaBackend, object: libc::c_uint, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), object)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"keymap-layout-group-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(keymap_layout_group_changed_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "last-device-changed")]
    pub fn connect_last_device_changed<F: Fn(&Self, &clutter::InputDevice) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn last_device_changed_trampoline<F: Fn(&Backend, &clutter::InputDevice) + 'static>(this: *mut ffi::MetaBackend, object: *mut clutter::ffi::ClutterInputDevice, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(object))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"last-device-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(last_device_changed_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "lid-is-closed-changed")]
    pub fn connect_lid_is_closed_changed<F: Fn(&Self, bool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn lid_is_closed_changed_trampoline<F: Fn(&Backend, bool) + 'static>(this: *mut ffi::MetaBackend, object: glib::ffi::gboolean, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(object))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"lid-is-closed-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(lid_is_closed_changed_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "prepare-shutdown")]
    pub fn connect_prepare_shutdown<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn prepare_shutdown_trampoline<F: Fn(&Backend) + 'static>(this: *mut ffi::MetaBackend, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"prepare-shutdown\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(prepare_shutdown_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Backend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Backend")
    }
}
