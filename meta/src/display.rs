use glib::{
    object::IsA,
    translate::*,
};
use std::boxed::Box as Box_;

use crate::{
    Display,
    KeyBinding,
    KeyBindingFlags,
    Window,
};

impl Display {
    /// Add a keybinding at runtime. The key `name` in `schema` needs to be of
    /// type `G_VARIANT_TYPE_STRING_ARRAY`, with each string describing a
    /// keybinding in the form of "&lt;Control&gt;a" or "&lt;Shift&gt;&lt;Alt&gt;F1". The parser
    /// is fairly liberal and allows lower or upper case, and also abbreviations
    /// such as "&lt;Ctl&gt;" and "&lt;Ctrl&gt;". If the key is set to the empty list or a
    /// list with a single element of either "" or "disabled", the keybinding is
    /// disabled.
    ///
    /// Use [`remove_keybinding()`][Self::remove_keybinding()] to remove the binding.
    /// ## `name`
    /// the binding's name
    /// ## `settings`
    /// the [`gio::Settings`][crate::gio::Settings] object where `name` is stored
    /// ## `flags`
    /// flags to specify binding details
    /// ## `handler`
    /// function to run when the keybinding is invoked
    /// ## `free_data`
    /// function to free `user_data`
    ///
    /// # Returns
    ///
    /// the corresponding keybinding action if the keybinding was
    ///  added successfully, otherwise `META_KEYBINDING_ACTION_NONE`
    #[doc(alias = "meta_display_add_keybinding")]
    pub fn add_keybinding<P: IsA<gio::Settings>, Q: Fn(&Display, Option<&Window>, Option<&clutter::KeyEvent>, &KeyBinding) + 'static>(&self, name: &str, settings: &P, flags: KeyBindingFlags, handler: Q) -> u32 {
        let handler_data: Box_<Q> = Box_::new(handler);
        unsafe extern "C" fn handler_func<P: IsA<gio::Settings>, Q: Fn(&Display, Option<&Window>, Option<&clutter::KeyEvent>, &KeyBinding) + 'static>(display: *mut ffi::MetaDisplay, window: *mut ffi::MetaWindow, event: *mut clutter::ffi::ClutterKeyEvent, binding: *mut ffi::MetaKeyBinding, user_data: glib::ffi::gpointer) {
            let display = from_glib_borrow(display);
            let window: Borrowed<Option<Window>> = from_glib_borrow(window);
            let event: Borrowed<Option<clutter::KeyEvent>> = from_glib_borrow(event);
            let binding = from_glib_borrow(binding);
            let callback: &Q = &*(user_data as *mut _);
            (*callback)(&display, window.as_ref().as_ref(), event.as_ref().as_ref(), &binding);
        }
        let handler = Some(handler_func::<P, Q> as _);
        unsafe extern "C" fn free_data_func<P: IsA<gio::Settings>, Q: Fn(&Display, Option<&Window>, Option<&clutter::KeyEvent>, &KeyBinding) + 'static>(data: glib::ffi::gpointer) {
            let _callback: Box_<Q> = Box_::from_raw(data as *mut _);
        }
        let destroy_call6 = Some(free_data_func::<P, Q> as _);
        let super_callback0: Box_<Q> = handler_data;
        unsafe {
            ffi::meta_display_add_keybinding(self.to_glib_none().0, name.to_glib_none().0, settings.as_ref().to_glib_none().0, flags.into_glib(), handler, Box_::into_raw(super_callback0) as *mut _, destroy_call6)
        }
    }
}
