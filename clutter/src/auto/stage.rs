// Generated by gir (https://github.com/gtk-rs/gir @ b193568)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 8e47c67)
// from mutter-gir-files
// DO NOT EDIT

use crate::Actor;
use crate::Animatable;
use crate::Container;
use crate::InputDevice;
use crate::Scriptable;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v1_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "ClutterStage")]
    pub struct Stage(Object<ffi::ClutterStage, ffi::ClutterStageClass>) @extends Actor, @implements Animatable, Container, Scriptable;

    match fn {
        type_ => || ffi::clutter_stage_get_type(),
    }
}

pub const NONE_STAGE: Option<&Stage> = None;

/// Trait containing all [`struct@Stage`] methods.
///
/// # Implementors
///
/// [`Stage`][struct@crate::Stage]
pub trait StageExt: 'static {
    //#[doc(alias = "clutter_stage_capture_into")]
    //fn capture_into(&self, rect: /*Ignored*/&mut cairo::RectangleInt, scale: f32, data: u8, stride: i32);

    //#[doc(alias = "clutter_stage_capture_view_into")]
    //fn capture_view_into(&self, view: /*Ignored*/&StageView, rect: /*Ignored*/&mut cairo::RectangleInt, data: u8, stride: i32);

    #[doc(alias = "clutter_stage_clear_stage_views")]
    fn clear_stage_views(&self);

    /// Ensures that the GL viewport is updated with the current
    /// stage window size.
    ///
    /// This function will queue a redraw of `self`.
    ///
    /// This function should not be called by applications; it is used
    /// when embedding a [`Stage`][crate::Stage] into a toolkit with another
    /// windowing system, like GTK+.
    #[cfg(any(feature = "v1_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_0")))]
    #[doc(alias = "clutter_stage_ensure_viewport")]
    fn ensure_viewport(&self);

    //#[doc(alias = "clutter_stage_event")]
    //fn event(&self, event: /*Ignored*/&mut Event) -> bool;

    //#[cfg(any(feature = "v1_0", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_0")))]
    //#[doc(alias = "clutter_stage_get_actor_at_pos")]
    //#[doc(alias = "get_actor_at_pos")]
    //fn actor_at_pos(&self, pick_mode: /*Ignored*/PickMode, x: f32, y: f32) -> Option<Actor>;

    //#[doc(alias = "clutter_stage_get_capture_final_size")]
    //#[doc(alias = "get_capture_final_size")]
    //fn capture_final_size(&self, rect: /*Ignored*/&mut cairo::RectangleInt) -> Option<(i32, i32, f32)>;

    //#[cfg(any(feature = "v1_12", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    //#[doc(alias = "clutter_stage_get_device_actor")]
    //#[doc(alias = "get_device_actor")]
    //fn device_actor(&self, device: &InputDevice, sequence: /*Ignored*/Option<&mut EventSequence>) -> Option<Actor>;

    //#[cfg(any(feature = "v1_12", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    //#[doc(alias = "clutter_stage_get_device_coords")]
    //#[doc(alias = "get_device_coords")]
    //fn device_coords(&self, device: &InputDevice, sequence: /*Ignored*/&mut EventSequence, coords: /*Ignored*/&mut graphene::Point);

    #[doc(alias = "clutter_stage_get_frame_counter")]
    #[doc(alias = "get_frame_counter")]
    fn frame_counter(&self) -> i64;

    /// Retrieves the actor that is currently under key focus.
    ///
    /// # Returns
    ///
    /// the actor with key focus, or the stage
    #[cfg(any(feature = "v0_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_6")))]
    #[doc(alias = "clutter_stage_get_key_focus")]
    #[doc(alias = "get_key_focus")]
    fn key_focus(&self) -> Option<Actor>;

    /// Retrieves the minimum size for a stage window as set using
    /// [`set_minimum_size()`][Self::set_minimum_size()].
    ///
    /// The returned size may not correspond to the actual minimum size and
    /// it is specific to the [`Stage`][crate::Stage] implementation inside the
    /// Clutter backend
    ///
    /// # Returns
    ///
    ///
    /// ## `width`
    /// return location for the minimum width, in pixels,
    ///  or [`None`]
    ///
    /// ## `height`
    /// return location for the minimum height, in pixels,
    ///  or [`None`]
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "clutter_stage_get_minimum_size")]
    #[doc(alias = "get_minimum_size")]
    fn minimum_size(&self) -> (u32, u32);

    /// Retrieves the value set using [`set_motion_events_enabled()`][Self::set_motion_events_enabled()].
    ///
    /// # Returns
    ///
    /// [`true`] if the per-actor motion event delivery is enabled
    ///  and [`false`] otherwise
    #[cfg(any(feature = "v1_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_8")))]
    #[doc(alias = "clutter_stage_get_motion_events_enabled")]
    #[doc(alias = "get_motion_events_enabled")]
    fn is_motion_events_enabled(&self) -> bool;

    //#[cfg(any(feature = "v0_4", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4")))]
    //#[doc(alias = "clutter_stage_get_perspective")]
    //#[doc(alias = "get_perspective")]
    //fn perspective(&self, perspective: /*Ignored*/Perspective);

    /// Retrieves the value set with [`set_throttle_motion_events()`][Self::set_throttle_motion_events()]
    ///
    /// # Returns
    ///
    /// [`true`] if the motion events are being throttled,
    ///  and [`false`] otherwise
    #[cfg(any(feature = "v1_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_0")))]
    #[doc(alias = "clutter_stage_get_throttle_motion_events")]
    #[doc(alias = "get_throttle_motion_events")]
    fn is_throttle_motion_events(&self) -> bool;

    /// Gets the stage title.
    ///
    /// # Returns
    ///
    /// pointer to the title string for the stage. The
    /// returned string is owned by the actor and should not
    /// be modified or freed.
    #[cfg(any(feature = "v0_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4")))]
    #[doc(alias = "clutter_stage_get_title")]
    #[doc(alias = "get_title")]
    fn title(&self) -> Option<glib::GString>;

    #[doc(alias = "clutter_stage_get_use_alpha")]
    #[doc(alias = "get_use_alpha")]
    fn uses_alpha(&self) -> bool;

    //#[doc(alias = "clutter_stage_get_view_at")]
    //#[doc(alias = "get_view_at")]
    //fn view_at(&self, x: f32, y: f32) -> /*Ignored*/Option<StageView>;

    //#[doc(alias = "clutter_stage_is_redraw_queued_on_view")]
    //fn is_redraw_queued_on_view(&self, view: /*Ignored*/&StageView) -> bool;

    //#[doc(alias = "clutter_stage_paint_to_buffer")]
    //fn paint_to_buffer(&self, rect: /*Ignored*/&cairo::RectangleInt, scale: f32, data: /*Unimplemented*/Vec<u8>, stride: i32, format: /*Ignored*/cogl::PixelFormat, paint_flags: /*Ignored*/PaintFlag) -> Result<(), glib::Error>;

    //#[doc(alias = "clutter_stage_paint_to_framebuffer")]
    //fn paint_to_framebuffer(&self, framebuffer: /*Ignored*/&cogl::Framebuffer, rect: /*Ignored*/&cairo::RectangleInt, scale: f32, paint_flags: /*Ignored*/PaintFlag);

    //#[doc(alias = "clutter_stage_peek_stage_views")]
    //fn peek_stage_views(&self) -> /*Unimplemented*/Vec<Fundamental: Pointer>;

    #[doc(alias = "clutter_stage_repick_device")]
    fn repick_device(&self, device: &InputDevice);

    /// Schedules a redraw of the [`Stage`][crate::Stage] at the next optimal timestamp.
    #[doc(alias = "clutter_stage_schedule_update")]
    fn schedule_update(&self);

    /// Sets the key focus on `actor`. An actor with key focus will receive
    /// all the key events. If `actor` is [`None`], the stage will receive
    /// focus.
    /// ## `actor`
    /// the actor to set key focus to, or [`None`]
    #[cfg(any(feature = "v0_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_6")))]
    #[doc(alias = "clutter_stage_set_key_focus")]
    fn set_key_focus<P: IsA<Actor>>(&self, actor: Option<&P>);

    /// Sets the minimum size for a stage window, if the default backend
    /// uses [`Stage`][crate::Stage] inside a window
    ///
    /// This is a convenience function, and it is equivalent to setting the
    /// `property::Actor::min-width` and `property::Actor::min-height` on `self`
    ///
    /// If the current size of `self` is smaller than the minimum size, the
    /// `self` will be resized to the new `width` and `height`
    /// ## `width`
    /// width, in pixels
    /// ## `height`
    /// height, in pixels
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "clutter_stage_set_minimum_size")]
    fn set_minimum_size(&self, width: u32, height: u32);

    /// Sets whether per-actor motion events (and relative crossing
    /// events) should be disabled or not.
    ///
    /// The default is [`true`].
    ///
    /// If `enable` is [`false`] the following signals will not be emitted
    /// by the actors children of `self`:
    ///
    ///  - `signal::Actor::motion-event`
    ///  - `signal::Actor::enter-event`
    ///  - `signal::Actor::leave-event`
    ///
    /// The events will still be delivered to the [`Stage`][crate::Stage].
    ///
    /// The main side effect of this function is that disabling the motion
    /// events will disable picking to detect the [`Actor`][crate::Actor] underneath
    /// the pointer for each motion event. This is useful, for instance,
    /// when dragging a [`Actor`][crate::Actor] across the `self`: the actor underneath
    /// the pointer is not going to change, so it's meaningless to perform
    /// a pick.
    /// ## `enabled`
    /// [`true`] to enable the motion events delivery, and [`false`]
    ///  otherwise
    #[cfg(any(feature = "v1_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_8")))]
    #[doc(alias = "clutter_stage_set_motion_events_enabled")]
    fn set_motion_events_enabled(&self, enabled: bool);

    /// Sets whether motion events received between redraws should
    /// be throttled or not. If motion events are throttled, those
    /// events received by the windowing system between redraws will
    /// be compressed so that only the last event will be propagated
    /// to the `self` and its actors.
    ///
    /// This function should only be used if you want to have all
    /// the motion events delivered to your application code.
    /// ## `throttle`
    /// [`true`] to throttle motion events
    #[cfg(any(feature = "v1_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_0")))]
    #[doc(alias = "clutter_stage_set_throttle_motion_events")]
    fn set_throttle_motion_events(&self, throttle: bool);

    /// Sets the stage title.
    /// ## `title`
    /// A utf8 string for the stage windows title.
    #[cfg(any(feature = "v0_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4")))]
    #[doc(alias = "clutter_stage_set_title")]
    fn set_title(&self, title: &str);

    #[doc(alias = "clutter_stage_set_use_alpha")]
    fn set_use_alpha(&self, use_alpha: bool);

    //#[cfg(any(feature = "v1_12", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    //#[doc(alias = "clutter_stage_update_device")]
    //fn update_device<P: IsA<Actor>>(&self, device: &InputDevice, sequence: /*Ignored*/&mut EventSequence, point: /*Ignored*/&graphene::Point, time: u32, new_actor: &P, emit_crossing: bool);

    /// The ::activate signal is emitted when the stage receives key focus
    /// from the underlying window system.
    #[cfg(any(feature = "v0_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_6")))]
    #[doc(alias = "activate")]
    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    //#[cfg(any(feature = "v1_20", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    //#[doc(alias = "after-paint")]
    //fn connect_after_paint<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    //#[doc(alias = "after-update")]
    //fn connect_after_update<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    //#[doc(alias = "before-paint")]
    //fn connect_before_paint<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    //#[doc(alias = "before-update")]
    //fn connect_before_update<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    /// The ::deactivate signal is emitted when the stage loses key focus
    /// from the underlying window system.
    #[cfg(any(feature = "v0_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_6")))]
    #[doc(alias = "deactivate")]
    fn connect_deactivate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    /// Signals that the underlying GL driver has had its texture memory purged
    /// so anything presently held in texture memory is now invalidated, and
    /// likely corrupt. It needs redrawing.
    #[doc(alias = "gl-video-memory-purged")]
    fn connect_gl_video_memory_purged<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    //#[doc(alias = "paint-view")]
    //fn connect_paint_view<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    //#[doc(alias = "presented")]
    //fn connect_presented<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "key-focus")]
    fn connect_key_focus_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v0_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8")))]
    #[doc(alias = "perspective")]
    fn connect_perspective_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v0_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4")))]
    #[doc(alias = "title")]
    fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Stage>> StageExt for O {
    //fn capture_into(&self, rect: /*Ignored*/&mut cairo::RectangleInt, scale: f32, data: u8, stride: i32) {
    //    unsafe { TODO: call ffi:clutter_stage_capture_into() }
    //}

    //fn capture_view_into(&self, view: /*Ignored*/&StageView, rect: /*Ignored*/&mut cairo::RectangleInt, data: u8, stride: i32) {
    //    unsafe { TODO: call ffi:clutter_stage_capture_view_into() }
    //}

    fn clear_stage_views(&self) {
        unsafe {
            ffi::clutter_stage_clear_stage_views(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_0")))]
    fn ensure_viewport(&self) {
        unsafe {
            ffi::clutter_stage_ensure_viewport(self.as_ref().to_glib_none().0);
        }
    }

    //fn event(&self, event: /*Ignored*/&mut Event) -> bool {
    //    unsafe { TODO: call ffi:clutter_stage_event() }
    //}

    //#[cfg(any(feature = "v1_0", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_0")))]
    //fn actor_at_pos(&self, pick_mode: /*Ignored*/PickMode, x: f32, y: f32) -> Option<Actor> {
    //    unsafe { TODO: call ffi:clutter_stage_get_actor_at_pos() }
    //}

    //fn capture_final_size(&self, rect: /*Ignored*/&mut cairo::RectangleInt) -> Option<(i32, i32, f32)> {
    //    unsafe { TODO: call ffi:clutter_stage_get_capture_final_size() }
    //}

    //#[cfg(any(feature = "v1_12", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    //fn device_actor(&self, device: &InputDevice, sequence: /*Ignored*/Option<&mut EventSequence>) -> Option<Actor> {
    //    unsafe { TODO: call ffi:clutter_stage_get_device_actor() }
    //}

    //#[cfg(any(feature = "v1_12", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    //fn device_coords(&self, device: &InputDevice, sequence: /*Ignored*/&mut EventSequence, coords: /*Ignored*/&mut graphene::Point) {
    //    unsafe { TODO: call ffi:clutter_stage_get_device_coords() }
    //}

    fn frame_counter(&self) -> i64 {
        unsafe {
            ffi::clutter_stage_get_frame_counter(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v0_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_6")))]
    fn key_focus(&self) -> Option<Actor> {
        unsafe {
            from_glib_none(ffi::clutter_stage_get_key_focus(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    fn minimum_size(&self) -> (u32, u32) {
        unsafe {
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            ffi::clutter_stage_get_minimum_size(self.as_ref().to_glib_none().0, width.as_mut_ptr(), height.as_mut_ptr());
            let width = width.assume_init();
            let height = height.assume_init();
            (width, height)
        }
    }

    #[cfg(any(feature = "v1_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_8")))]
    fn is_motion_events_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::clutter_stage_get_motion_events_enabled(self.as_ref().to_glib_none().0))
        }
    }

    //#[cfg(any(feature = "v0_4", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4")))]
    //fn perspective(&self, perspective: /*Ignored*/Perspective) {
    //    unsafe { TODO: call ffi:clutter_stage_get_perspective() }
    //}

    #[cfg(any(feature = "v1_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_0")))]
    fn is_throttle_motion_events(&self) -> bool {
        unsafe {
            from_glib(ffi::clutter_stage_get_throttle_motion_events(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v0_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4")))]
    fn title(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::clutter_stage_get_title(self.as_ref().to_glib_none().0))
        }
    }

    fn uses_alpha(&self) -> bool {
        unsafe {
            from_glib(ffi::clutter_stage_get_use_alpha(self.as_ref().to_glib_none().0))
        }
    }

    //fn view_at(&self, x: f32, y: f32) -> /*Ignored*/Option<StageView> {
    //    unsafe { TODO: call ffi:clutter_stage_get_view_at() }
    //}

    //fn is_redraw_queued_on_view(&self, view: /*Ignored*/&StageView) -> bool {
    //    unsafe { TODO: call ffi:clutter_stage_is_redraw_queued_on_view() }
    //}

    //fn paint_to_buffer(&self, rect: /*Ignored*/&cairo::RectangleInt, scale: f32, data: /*Unimplemented*/Vec<u8>, stride: i32, format: /*Ignored*/cogl::PixelFormat, paint_flags: /*Ignored*/PaintFlag) -> Result<(), glib::Error> {
    //    unsafe { TODO: call ffi:clutter_stage_paint_to_buffer() }
    //}

    //fn paint_to_framebuffer(&self, framebuffer: /*Ignored*/&cogl::Framebuffer, rect: /*Ignored*/&cairo::RectangleInt, scale: f32, paint_flags: /*Ignored*/PaintFlag) {
    //    unsafe { TODO: call ffi:clutter_stage_paint_to_framebuffer() }
    //}

    //fn peek_stage_views(&self) -> /*Unimplemented*/Vec<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi:clutter_stage_peek_stage_views() }
    //}

    fn repick_device(&self, device: &InputDevice) {
        unsafe {
            ffi::clutter_stage_repick_device(self.as_ref().to_glib_none().0, device.to_glib_none().0);
        }
    }

    fn schedule_update(&self) {
        unsafe {
            ffi::clutter_stage_schedule_update(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v0_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_6")))]
    fn set_key_focus<P: IsA<Actor>>(&self, actor: Option<&P>) {
        unsafe {
            ffi::clutter_stage_set_key_focus(self.as_ref().to_glib_none().0, actor.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    fn set_minimum_size(&self, width: u32, height: u32) {
        unsafe {
            ffi::clutter_stage_set_minimum_size(self.as_ref().to_glib_none().0, width, height);
        }
    }

    #[cfg(any(feature = "v1_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_8")))]
    fn set_motion_events_enabled(&self, enabled: bool) {
        unsafe {
            ffi::clutter_stage_set_motion_events_enabled(self.as_ref().to_glib_none().0, enabled.into_glib());
        }
    }

    #[cfg(any(feature = "v1_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_0")))]
    fn set_throttle_motion_events(&self, throttle: bool) {
        unsafe {
            ffi::clutter_stage_set_throttle_motion_events(self.as_ref().to_glib_none().0, throttle.into_glib());
        }
    }

    #[cfg(any(feature = "v0_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4")))]
    fn set_title(&self, title: &str) {
        unsafe {
            ffi::clutter_stage_set_title(self.as_ref().to_glib_none().0, title.to_glib_none().0);
        }
    }

    fn set_use_alpha(&self, use_alpha: bool) {
        unsafe {
            ffi::clutter_stage_set_use_alpha(self.as_ref().to_glib_none().0, use_alpha.into_glib());
        }
    }

    //#[cfg(any(feature = "v1_12", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    //fn update_device<P: IsA<Actor>>(&self, device: &InputDevice, sequence: /*Ignored*/&mut EventSequence, point: /*Ignored*/&graphene::Point, time: u32, new_actor: &P, emit_crossing: bool) {
    //    unsafe { TODO: call ffi:clutter_stage_update_device() }
    //}

    #[cfg(any(feature = "v0_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_6")))]
    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<P: IsA<Stage>, F: Fn(&P) + 'static>(this: *mut ffi::ClutterStage, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Stage::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"activate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(activate_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    //#[cfg(any(feature = "v1_20", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    //fn connect_after_paint<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored view: Clutter.StageView
    //}

    //fn connect_after_update<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored view: Clutter.StageView
    //}

    //fn connect_before_paint<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored view: Clutter.StageView
    //}

    //fn connect_before_update<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored view: Clutter.StageView
    //}

    #[cfg(any(feature = "v0_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_6")))]
    fn connect_deactivate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn deactivate_trampoline<P: IsA<Stage>, F: Fn(&P) + 'static>(this: *mut ffi::ClutterStage, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Stage::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"deactivate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(deactivate_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_gl_video_memory_purged<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn gl_video_memory_purged_trampoline<P: IsA<Stage>, F: Fn(&P) + 'static>(this: *mut ffi::ClutterStage, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Stage::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"gl-video-memory-purged\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(gl_video_memory_purged_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    //fn connect_paint_view<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored view: Clutter.StageView
    //    Ignored redraw_clip: cairo.Region
    //}

    //fn connect_presented<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored view: Clutter.StageView
    //    Unimplemented frame_info: *.Pointer
    //}

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    fn connect_key_focus_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_key_focus_trampoline<P: IsA<Stage>, F: Fn(&P) + 'static>(this: *mut ffi::ClutterStage, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Stage::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::key-focus\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_key_focus_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v0_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8")))]
    fn connect_perspective_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_perspective_trampoline<P: IsA<Stage>, F: Fn(&P) + 'static>(this: *mut ffi::ClutterStage, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Stage::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::perspective\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_perspective_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v0_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4")))]
    fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<P: IsA<Stage>, F: Fn(&P) + 'static>(this: *mut ffi::ClutterStage, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Stage::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::title\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_title_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Stage")
    }
}
