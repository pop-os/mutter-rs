// Generated by gir (https://github.com/gtk-rs/gir @ 45cd7bc)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 8e47c67)
// from mutter-gir-files
// DO NOT EDIT

use crate::Display;
use crate::ModalOptions;
use crate::WindowActor;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "MetaPlugin")]
    pub struct Plugin(Object<ffi::MetaPlugin, ffi::MetaPluginClass>);

    match fn {
        type_ => || ffi::meta_plugin_get_type(),
    }
}

impl Plugin {
    #[doc(alias = "meta_plugin_manager_set_plugin_type")]
    pub fn manager_set_plugin_type(gtype: glib::types::Type) {
        unsafe {
            ffi::meta_plugin_manager_set_plugin_type(gtype.into_glib());
        }
    }
}

pub const NONE_PLUGIN: Option<&Plugin> = None;

/// Trait containing all [`struct@Plugin`] methods.
///
/// # Implementors
///
/// [`Plugin`][struct@crate::Plugin]
pub trait PluginExt: 'static {
    /// This function is used to grab the keyboard and mouse for the exclusive
    /// use of the plugin. Correct operation requires that both the keyboard
    /// and mouse are grabbed, or thing will break. (In particular, other
    /// passive X grabs in Meta can trigger but not be handled by the normal
    /// keybinding handling code.) However, the plugin can establish the keyboard
    /// and/or mouse grabs ahead of time and pass in the
    /// [`ModalOptions::POINTER_ALREADY_GRABBED`][crate::ModalOptions::POINTER_ALREADY_GRABBED] and/or [`ModalOptions::KEYBOARD_ALREADY_GRABBED`][crate::ModalOptions::KEYBOARD_ALREADY_GRABBED]
    /// options. This facility is provided for two reasons: first to allow using
    /// this function to establish modality after a passive grab, and second to
    /// allow using obscure features of XGrabPointer() and XGrabKeyboard() without
    /// having to add them to this API.
    /// ## `options`
    /// flags that modify the behavior of the modal grab
    /// ## `timestamp`
    /// the timestamp used for establishing grabs
    ///
    /// # Returns
    ///
    /// whether we successfully grabbed the keyboard and
    ///  mouse and made the plugin modal.
    #[doc(alias = "meta_plugin_begin_modal")]
    fn begin_modal(&self, options: ModalOptions, timestamp: u32) -> bool;

    #[doc(alias = "meta_plugin_complete_display_change")]
    fn complete_display_change(&self, ok: bool);

    #[doc(alias = "meta_plugin_destroy_completed")]
    fn destroy_completed(&self, actor: &WindowActor);

    /// Ends the modal operation begun with [`begin_modal()`][Self::begin_modal()]. This
    /// ungrabs both the mouse and keyboard even when
    /// [`ModalOptions::POINTER_ALREADY_GRABBED`][crate::ModalOptions::POINTER_ALREADY_GRABBED] or
    /// [`ModalOptions::KEYBOARD_ALREADY_GRABBED`][crate::ModalOptions::KEYBOARD_ALREADY_GRABBED] were provided as options
    /// when beginnning the modal operation.
    /// ## `timestamp`
    /// the time used for releasing grabs
    #[doc(alias = "meta_plugin_end_modal")]
    fn end_modal(&self, timestamp: u32);

    /// Gets the [`Display`][crate::Display] corresponding to a plugin.
    ///
    /// # Returns
    ///
    /// the [`Display`][crate::Display] for the plugin
    #[doc(alias = "meta_plugin_get_display")]
    #[doc(alias = "get_display")]
    fn display(&self) -> Option<Display>;

    //#[doc(alias = "meta_plugin_get_info")]
    //#[doc(alias = "get_info")]
    //fn info(&self) -> /*Ignored*/Option<PluginInfo>;

    #[doc(alias = "meta_plugin_map_completed")]
    fn map_completed(&self, actor: &WindowActor);

    #[doc(alias = "meta_plugin_minimize_completed")]
    fn minimize_completed(&self, actor: &WindowActor);

    #[doc(alias = "meta_plugin_size_change_completed")]
    fn size_change_completed(&self, actor: &WindowActor);

    #[doc(alias = "meta_plugin_switch_workspace_completed")]
    fn switch_workspace_completed(&self);

    #[doc(alias = "meta_plugin_unminimize_completed")]
    fn unminimize_completed(&self, actor: &WindowActor);
}

impl<O: IsA<Plugin>> PluginExt for O {
    fn begin_modal(&self, options: ModalOptions, timestamp: u32) -> bool {
        unsafe {
            from_glib(ffi::meta_plugin_begin_modal(self.as_ref().to_glib_none().0, options.into_glib(), timestamp))
        }
    }

    fn complete_display_change(&self, ok: bool) {
        unsafe {
            ffi::meta_plugin_complete_display_change(self.as_ref().to_glib_none().0, ok.into_glib());
        }
    }

    fn destroy_completed(&self, actor: &WindowActor) {
        unsafe {
            ffi::meta_plugin_destroy_completed(self.as_ref().to_glib_none().0, actor.to_glib_none().0);
        }
    }

    fn end_modal(&self, timestamp: u32) {
        unsafe {
            ffi::meta_plugin_end_modal(self.as_ref().to_glib_none().0, timestamp);
        }
    }

    fn display(&self) -> Option<Display> {
        unsafe {
            from_glib_none(ffi::meta_plugin_get_display(self.as_ref().to_glib_none().0))
        }
    }

    //fn info(&self) -> /*Ignored*/Option<PluginInfo> {
    //    unsafe { TODO: call ffi:meta_plugin_get_info() }
    //}

    fn map_completed(&self, actor: &WindowActor) {
        unsafe {
            ffi::meta_plugin_map_completed(self.as_ref().to_glib_none().0, actor.to_glib_none().0);
        }
    }

    fn minimize_completed(&self, actor: &WindowActor) {
        unsafe {
            ffi::meta_plugin_minimize_completed(self.as_ref().to_glib_none().0, actor.to_glib_none().0);
        }
    }

    fn size_change_completed(&self, actor: &WindowActor) {
        unsafe {
            ffi::meta_plugin_size_change_completed(self.as_ref().to_glib_none().0, actor.to_glib_none().0);
        }
    }

    fn switch_workspace_completed(&self) {
        unsafe {
            ffi::meta_plugin_switch_workspace_completed(self.as_ref().to_glib_none().0);
        }
    }

    fn unminimize_completed(&self, actor: &WindowActor) {
        unsafe {
            ffi::meta_plugin_unminimize_completed(self.as_ref().to_glib_none().0, actor.to_glib_none().0);
        }
    }
}

impl fmt::Display for Plugin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Plugin")
    }
}
