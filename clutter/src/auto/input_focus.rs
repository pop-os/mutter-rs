// Generated by gir (https://github.com/gtk-rs/gir @ 45cd7bc)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 8e47c67)
// from mutter-gir-files
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "ClutterInputFocus")]
    pub struct InputFocus(Object<ffi::ClutterInputFocus, ffi::ClutterInputFocusClass>);

    match fn {
        type_ => || ffi::clutter_input_focus_get_type(),
    }
}

pub const NONE_INPUT_FOCUS: Option<&InputFocus> = None;

/// Trait containing all [`struct@InputFocus`] methods.
///
/// # Implementors
///
/// [`InputFocus`][struct@crate::InputFocus]
pub trait InputFocusExt: 'static {
    //#[doc(alias = "clutter_input_focus_filter_event")]
    //fn filter_event(&self, event: /*Ignored*/&Event) -> bool;

    #[doc(alias = "clutter_input_focus_is_focused")]
    fn is_focused(&self) -> bool;

    #[doc(alias = "clutter_input_focus_reset")]
    fn reset(&self);

    #[doc(alias = "clutter_input_focus_set_can_show_preedit")]
    fn set_can_show_preedit(&self, can_show_preedit: bool);

    //#[doc(alias = "clutter_input_focus_set_content_hints")]
    //fn set_content_hints(&self, hint: /*Ignored*/InputContentHintFlags);

    //#[doc(alias = "clutter_input_focus_set_content_purpose")]
    //fn set_content_purpose(&self, purpose: /*Ignored*/InputContentPurpose);

    //#[doc(alias = "clutter_input_focus_set_cursor_location")]
    //fn set_cursor_location(&self, rect: /*Ignored*/&graphene::Rect);

    //#[doc(alias = "clutter_input_focus_set_input_panel_state")]
    //fn set_input_panel_state(&self, state: /*Ignored*/InputPanelState);

    #[doc(alias = "clutter_input_focus_set_surrounding")]
    fn set_surrounding(&self, text: &str, cursor: u32, anchor: u32);
}

impl<O: IsA<InputFocus>> InputFocusExt for O {
    //fn filter_event(&self, event: /*Ignored*/&Event) -> bool {
    //    unsafe { TODO: call ffi:clutter_input_focus_filter_event() }
    //}

    fn is_focused(&self) -> bool {
        unsafe {
            from_glib(ffi::clutter_input_focus_is_focused(self.as_ref().to_glib_none().0))
        }
    }

    fn reset(&self) {
        unsafe {
            ffi::clutter_input_focus_reset(self.as_ref().to_glib_none().0);
        }
    }

    fn set_can_show_preedit(&self, can_show_preedit: bool) {
        unsafe {
            ffi::clutter_input_focus_set_can_show_preedit(self.as_ref().to_glib_none().0, can_show_preedit.into_glib());
        }
    }

    //fn set_content_hints(&self, hint: /*Ignored*/InputContentHintFlags) {
    //    unsafe { TODO: call ffi:clutter_input_focus_set_content_hints() }
    //}

    //fn set_content_purpose(&self, purpose: /*Ignored*/InputContentPurpose) {
    //    unsafe { TODO: call ffi:clutter_input_focus_set_content_purpose() }
    //}

    //fn set_cursor_location(&self, rect: /*Ignored*/&graphene::Rect) {
    //    unsafe { TODO: call ffi:clutter_input_focus_set_cursor_location() }
    //}

    //fn set_input_panel_state(&self, state: /*Ignored*/InputPanelState) {
    //    unsafe { TODO: call ffi:clutter_input_focus_set_input_panel_state() }
    //}

    fn set_surrounding(&self, text: &str, cursor: u32, anchor: u32) {
        unsafe {
            ffi::clutter_input_focus_set_surrounding(self.as_ref().to_glib_none().0, text.to_glib_none().0, cursor, anchor);
        }
    }
}

impl fmt::Display for InputFocus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("InputFocus")
    }
}
