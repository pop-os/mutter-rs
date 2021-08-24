// Generated by gir (https://github.com/gtk-rs/gir @ 5bbf6cb)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 8e47c67)
// from mutter-gir-files
// DO NOT EDIT

use crate::Background;
use crate::Display;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "MetaBackgroundContent")]
    pub struct BackgroundContent(Object<ffi::MetaBackgroundContent, ffi::MetaBackgroundContentClass>) @implements clutter::Content;

    match fn {
        type_ => || ffi::meta_background_content_get_type(),
    }
}

impl BackgroundContent {
    #[doc(alias = "meta_background_content_set_background")]
    pub fn set_background(&self, background: &Background) {
        unsafe {
            ffi::meta_background_content_set_background(self.to_glib_none().0, background.to_glib_none().0);
        }
    }

    #[doc(alias = "meta_background_content_set_gradient")]
    pub fn set_gradient(&self, enabled: bool, height: i32, tone_start: f64) {
        unsafe {
            ffi::meta_background_content_set_gradient(self.to_glib_none().0, enabled.into_glib(), height, tone_start);
        }
    }

    //#[doc(alias = "meta_background_content_set_rounded_clip_bounds")]
    //pub fn set_rounded_clip_bounds(&self, bounds: /*Ignored*/Option<&graphene::Rect>) {
    //    unsafe { TODO: call ffi:meta_background_content_set_rounded_clip_bounds() }
    //}

    #[doc(alias = "meta_background_content_set_rounded_clip_radius")]
    pub fn set_rounded_clip_radius(&self, radius: f32) {
        unsafe {
            ffi::meta_background_content_set_rounded_clip_radius(self.to_glib_none().0, radius);
        }
    }

    #[doc(alias = "meta_background_content_set_vignette")]
    pub fn set_vignette(&self, enabled: bool, brightness: f64, sharpness: f64) {
        unsafe {
            ffi::meta_background_content_set_vignette(self.to_glib_none().0, enabled.into_glib(), brightness, sharpness);
        }
    }

    pub fn background(&self) -> Option<Background> {
        unsafe {
            let mut value = glib::Value::from_type(<Background as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.as_ptr() as *mut glib::gobject_ffi::GObject, b"background\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `background` getter")
        }
    }

    pub fn brightness(&self) -> f64 {
        unsafe {
            let mut value = glib::Value::from_type(<f64 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.as_ptr() as *mut glib::gobject_ffi::GObject, b"brightness\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `brightness` getter")
        }
    }

    pub fn set_brightness(&self, brightness: f64) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(self.as_ptr() as *mut glib::gobject_ffi::GObject, b"brightness\0".as_ptr() as *const _, brightness.to_value().to_glib_none().0);
        }
    }

    pub fn is_gradient(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.as_ptr() as *mut glib::gobject_ffi::GObject, b"gradient\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `gradient` getter")
        }
    }

    #[doc(alias = "gradient-height")]
    pub fn gradient_height(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.as_ptr() as *mut glib::gobject_ffi::GObject, b"gradient-height\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `gradient-height` getter")
        }
    }

    #[doc(alias = "gradient-height")]
    pub fn set_gradient_height(&self, gradient_height: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(self.as_ptr() as *mut glib::gobject_ffi::GObject, b"gradient-height\0".as_ptr() as *const _, gradient_height.to_value().to_glib_none().0);
        }
    }

    #[doc(alias = "gradient-max-darkness")]
    pub fn gradient_max_darkness(&self) -> f64 {
        unsafe {
            let mut value = glib::Value::from_type(<f64 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.as_ptr() as *mut glib::gobject_ffi::GObject, b"gradient-max-darkness\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `gradient-max-darkness` getter")
        }
    }

    #[doc(alias = "gradient-max-darkness")]
    pub fn set_gradient_max_darkness(&self, gradient_max_darkness: f64) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(self.as_ptr() as *mut glib::gobject_ffi::GObject, b"gradient-max-darkness\0".as_ptr() as *const _, gradient_max_darkness.to_value().to_glib_none().0);
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

    #[doc(alias = "rounded-clip-radius")]
    pub fn rounded_clip_radius(&self) -> f32 {
        unsafe {
            let mut value = glib::Value::from_type(<f32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.as_ptr() as *mut glib::gobject_ffi::GObject, b"rounded-clip-radius\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `rounded-clip-radius` getter")
        }
    }

    pub fn is_vignette(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.as_ptr() as *mut glib::gobject_ffi::GObject, b"vignette\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `vignette` getter")
        }
    }

    #[doc(alias = "vignette-sharpness")]
    pub fn vignette_sharpness(&self) -> f64 {
        unsafe {
            let mut value = glib::Value::from_type(<f64 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.as_ptr() as *mut glib::gobject_ffi::GObject, b"vignette-sharpness\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `vignette-sharpness` getter")
        }
    }

    #[doc(alias = "vignette-sharpness")]
    pub fn set_vignette_sharpness(&self, vignette_sharpness: f64) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(self.as_ptr() as *mut glib::gobject_ffi::GObject, b"vignette-sharpness\0".as_ptr() as *const _, vignette_sharpness.to_value().to_glib_none().0);
        }
    }

    /// Creates a new actor to draw the background for the given monitor.
    /// ## `monitor`
    /// Index of the monitor for which to draw the background
    ///
    /// # Returns
    ///
    /// the newly created background actor
    #[doc(alias = "meta_background_content_new")]
    pub fn new(display: &Display, monitor: i32) -> Option<clutter::Content> {
        unsafe {
            from_glib_full(ffi::meta_background_content_new(display.to_glib_none().0, monitor))
        }
    }

    #[doc(alias = "background")]
    pub fn connect_background_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_background_trampoline<F: Fn(&BackgroundContent) + 'static>(this: *mut ffi::MetaBackgroundContent, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::background\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_background_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "brightness")]
    pub fn connect_brightness_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_brightness_trampoline<F: Fn(&BackgroundContent) + 'static>(this: *mut ffi::MetaBackgroundContent, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::brightness\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_brightness_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "gradient")]
    pub fn connect_gradient_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_gradient_trampoline<F: Fn(&BackgroundContent) + 'static>(this: *mut ffi::MetaBackgroundContent, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::gradient\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_gradient_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "gradient-height")]
    pub fn connect_gradient_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_gradient_height_trampoline<F: Fn(&BackgroundContent) + 'static>(this: *mut ffi::MetaBackgroundContent, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::gradient-height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_gradient_height_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "gradient-max-darkness")]
    pub fn connect_gradient_max_darkness_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_gradient_max_darkness_trampoline<F: Fn(&BackgroundContent) + 'static>(this: *mut ffi::MetaBackgroundContent, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::gradient-max-darkness\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_gradient_max_darkness_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "rounded-clip-radius")]
    pub fn connect_rounded_clip_radius_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_rounded_clip_radius_trampoline<F: Fn(&BackgroundContent) + 'static>(this: *mut ffi::MetaBackgroundContent, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::rounded-clip-radius\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_rounded_clip_radius_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "vignette")]
    pub fn connect_vignette_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_vignette_trampoline<F: Fn(&BackgroundContent) + 'static>(this: *mut ffi::MetaBackgroundContent, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::vignette\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_vignette_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "vignette-sharpness")]
    pub fn connect_vignette_sharpness_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_vignette_sharpness_trampoline<F: Fn(&BackgroundContent) + 'static>(this: *mut ffi::MetaBackgroundContent, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::vignette-sharpness\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_vignette_sharpness_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for BackgroundContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("BackgroundContent")
    }
}
