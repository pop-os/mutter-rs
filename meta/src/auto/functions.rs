// Generated by gir (https://github.com/gtk-rs/gir @ 5bbf6cb)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 8e47c67)
// from mutter-gir-files
// DO NOT EDIT

use crate::Display;
use crate::ExitCode;
use glib::translate::*;
use std::ptr;


//#[doc(alias = "meta_add_clutter_debug_flags")]
//pub fn add_clutter_debug_flags(debug_flags: /*Ignored*/clutter::DebugFlag, draw_flags: /*Ignored*/clutter::DrawDebugFlag, pick_flags: /*Ignored*/clutter::PickDebugFlag) {
//    unsafe { TODO: call ffi:meta_add_clutter_debug_flags() }
//}

//#[doc(alias = "meta_add_debug_paint_flag")]
//pub fn add_debug_paint_flag(flag: /*Ignored*/DebugPaintFlag) {
//    unsafe { TODO: call ffi:meta_add_debug_paint_flag() }
//}

//#[doc(alias = "meta_add_verbose_topic")]
//pub fn add_verbose_topic(topic: /*Ignored*/DebugTopic) {
//    unsafe { TODO: call ffi:meta_add_verbose_topic() }
//}

//#[doc(alias = "meta_bug")]
//pub fn bug(format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call ffi:meta_bug() }
//}

#[doc(alias = "meta_clutter_init")]
pub fn clutter_init() {
    unsafe {
        ffi::meta_clutter_init();
    }
}

/// Disables unredirection, can be useful in situations where having
/// unredirected windows is undesirable like when recording a video.
/// ## `display`
/// a [`Display`][crate::Display]
#[doc(alias = "meta_disable_unredirect_for_display")]
pub fn disable_unredirect_for_display(display: &Display) {
    unsafe {
        ffi::meta_disable_unredirect_for_display(display.to_glib_none().0);
    }
}

/// Enables unredirection which reduces the overhead for apps like games.
/// ## `display`
/// a [`Display`][crate::Display]
#[doc(alias = "meta_enable_unredirect_for_display")]
pub fn enable_unredirect_for_display(display: &Display) {
    unsafe {
        ffi::meta_enable_unredirect_for_display(display.to_glib_none().0);
    }
}

#[doc(alias = "meta_exit")]
pub fn exit(code: ExitCode) {
    unsafe {
        ffi::meta_exit(code.into_glib());
    }
}

#[doc(alias = "meta_external_binding_name_for_action")]
pub fn external_binding_name_for_action(keybinding_action: u32) -> Option<glib::GString> {
    unsafe {
        from_glib_full(ffi::meta_external_binding_name_for_action(keybinding_action))
    }
}

//#[doc(alias = "meta_fatal")]
//pub fn fatal(format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call ffi:meta_fatal() }
//}

#[doc(alias = "meta_finalize")]
pub fn finalize() {
    unsafe {
        ffi::meta_finalize();
    }
}

#[doc(alias = "meta_focus_stage_window")]
pub fn focus_stage_window(display: &Display, timestamp: u32) {
    unsafe {
        ffi::meta_focus_stage_window(display.to_glib_none().0, timestamp);
    }
}

#[doc(alias = "meta_g_utf8_strndup")]
pub fn g_utf8_strndup(src: &str, n: usize) -> Option<glib::GString> {
    unsafe {
        from_glib_full(ffi::meta_g_utf8_strndup(src.to_glib_none().0, n))
    }
}

//#[doc(alias = "meta_get_backend")]
//#[doc(alias = "get_backend")]
//pub fn backend() -> /*Ignored*/Option<Backend> {
//    unsafe { TODO: call ffi:meta_get_backend() }
//}

//#[doc(alias = "meta_get_debug_paint_flags")]
//#[doc(alias = "get_debug_paint_flags")]
//pub fn debug_paint_flags() -> /*Ignored*/DebugPaintFlag {
//    unsafe { TODO: call ffi:meta_get_debug_paint_flags() }
//}

#[doc(alias = "meta_get_exit_code")]
#[doc(alias = "get_exit_code")]
pub fn exit_code() -> ExitCode {
    unsafe {
        from_glib(ffi::meta_get_exit_code())
    }
}

/// ## `display`
/// a [`Display`][crate::Display]
///
/// # Returns
///
/// The feedback group corresponding to `display`
#[doc(alias = "meta_get_feedback_group_for_display")]
#[doc(alias = "get_feedback_group_for_display")]
pub fn feedback_group_for_display(display: &Display) -> Option<clutter::Actor> {
    unsafe {
        from_glib_none(ffi::meta_get_feedback_group_for_display(display.to_glib_none().0))
    }
}

//#[doc(alias = "meta_get_locale_direction")]
//#[doc(alias = "get_locale_direction")]
//pub fn locale_direction() -> /*Ignored*/LocaleDirection {
//    unsafe { TODO: call ffi:meta_get_locale_direction() }
//}

//#[doc(alias = "meta_get_option_context")]
//#[doc(alias = "get_option_context")]
//pub fn option_context() -> /*Ignored*/Option<glib::OptionContext> {
//    unsafe { TODO: call ffi:meta_get_option_context() }
//}

#[doc(alias = "meta_get_replace_current_wm")]
#[doc(alias = "get_replace_current_wm")]
pub fn is_replace_current_wm() -> bool {
    unsafe {
        from_glib(ffi::meta_get_replace_current_wm())
    }
}

/// ## `display`
/// a [`Display`][crate::Display]
///
/// # Returns
///
/// The [`clutter::Stage`][crate::clutter::Stage] for the display
#[doc(alias = "meta_get_stage_for_display")]
#[doc(alias = "get_stage_for_display")]
pub fn stage_for_display(display: &Display) -> Option<clutter::Actor> {
    unsafe {
        from_glib_none(ffi::meta_get_stage_for_display(display.to_glib_none().0))
    }
}

/// ## `display`
/// a [`Display`][crate::Display]
///
/// # Returns
///
/// The top window group corresponding to `display`
#[doc(alias = "meta_get_top_window_group_for_display")]
#[doc(alias = "get_top_window_group_for_display")]
pub fn top_window_group_for_display(display: &Display) -> Option<clutter::Actor> {
    unsafe {
        from_glib_none(ffi::meta_get_top_window_group_for_display(display.to_glib_none().0))
    }
}

/// ## `display`
/// a [`Display`][crate::Display]
///
/// # Returns
///
/// The set of [`WindowActor`][crate::WindowActor] on `display`
#[doc(alias = "meta_get_window_actors")]
#[doc(alias = "get_window_actors")]
pub fn window_actors(display: &Display) -> Vec<clutter::Actor> {
    unsafe {
        FromGlibPtrContainer::from_glib_none(ffi::meta_get_window_actors(display.to_glib_none().0))
    }
}

/// ## `display`
/// a [`Display`][crate::Display]
///
/// # Returns
///
/// The window group corresponding to `display`
#[doc(alias = "meta_get_window_group_for_display")]
#[doc(alias = "get_window_group_for_display")]
pub fn window_group_for_display(display: &Display) -> Option<clutter::Actor> {
    unsafe {
        from_glib_none(ffi::meta_get_window_group_for_display(display.to_glib_none().0))
    }
}

/// Initialize mutter. Call this after `meta_get_option_context()` and
/// [`Plugin::manager_set_plugin_type()`][crate::Plugin::manager_set_plugin_type()], and before [`run()`][crate::run()].
#[doc(alias = "meta_init")]
pub fn init() {
    unsafe {
        ffi::meta_init();
    }
}

/// Returns [`true`] if this instance of Mutter comes from Mutter
/// restarting itself (for example to enable/disable stereo.)
/// See [`restart()`][crate::restart()]. If this is the case, any startup visuals
/// or animations should be suppressed.
#[doc(alias = "meta_is_restart")]
pub fn is_restart() -> bool {
    unsafe {
        from_glib(ffi::meta_is_restart())
    }
}

/// Returns whether X synchronisation is currently enabled.
///
/// FIXME: This is *only* called by `meta_display_open()`, but by that time
/// we have already turned syncing on or off on startup, and we don't
/// have any way to do so while Mutter is running, so it's rather
/// pointless.
///
/// # Returns
///
/// [`true`] if we must wait for events whenever we send X requests;
/// [`false`] otherwise.
#[doc(alias = "meta_is_syncing")]
pub fn is_syncing() -> bool {
    unsafe {
        from_glib(ffi::meta_is_syncing())
    }
}

//#[doc(alias = "meta_is_topic_enabled")]
//pub fn is_topic_enabled(topic: /*Ignored*/DebugTopic) -> bool {
//    unsafe { TODO: call ffi:meta_is_topic_enabled() }
//}

#[doc(alias = "meta_is_verbose")]
pub fn is_verbose() -> bool {
    unsafe {
        from_glib(ffi::meta_is_verbose())
    }
}

#[doc(alias = "meta_is_wayland_compositor")]
pub fn is_wayland_compositor() -> bool {
    unsafe {
        from_glib(ffi::meta_is_wayland_compositor())
    }
}

//#[doc(alias = "meta_keybindings_set_custom_handler")]
//pub fn keybindings_set_custom_handler(name: &str, handler: /*Unimplemented*/Fn(&Display, &Window, /*Ignored*/Option<clutter::KeyEvent>, &KeyBinding), user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool {
//    unsafe { TODO: call ffi:meta_keybindings_set_custom_handler() }
//}

//#[doc(alias = "meta_later_add")]
//pub fn later_add<P: Fn() -> bool + Send + Sync + 'static>(when: /*Ignored*/LaterType, func: P) -> u32 {
//    unsafe { TODO: call ffi:meta_later_add() }
//}

/// Removes a callback added with `meta_later_add()`
/// ## `later_id`
/// the integer ID returned from `meta_later_add()`
#[doc(alias = "meta_later_remove")]
pub fn later_remove(later_id: u32) {
    unsafe {
        ffi::meta_later_remove(later_id);
    }
}

#[doc(alias = "meta_pop_no_msg_prefix")]
pub fn pop_no_msg_prefix() {
    unsafe {
        ffi::meta_pop_no_msg_prefix();
    }
}

//#[doc(alias = "meta_prefs_add_listener")]
//pub fn prefs_add_listener(func: /*Unimplemented*/Fn(/*Ignored*/Preference), user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
//    unsafe { TODO: call ffi:meta_prefs_add_listener() }
//}

#[doc(alias = "meta_prefs_bell_is_audible")]
pub fn prefs_bell_is_audible() -> bool {
    unsafe {
        from_glib(ffi::meta_prefs_bell_is_audible())
    }
}

#[doc(alias = "meta_prefs_change_workspace_name")]
pub fn prefs_change_workspace_name(i: i32, name: &str) {
    unsafe {
        ffi::meta_prefs_change_workspace_name(i, name.to_glib_none().0);
    }
}

//#[doc(alias = "meta_prefs_get_action_double_click_titlebar")]
//pub fn prefs_get_action_double_click_titlebar() -> /*Ignored*/gdesktop_enums::TitlebarAction {
//    unsafe { TODO: call ffi:meta_prefs_get_action_double_click_titlebar() }
//}

//#[doc(alias = "meta_prefs_get_action_middle_click_titlebar")]
//pub fn prefs_get_action_middle_click_titlebar() -> /*Ignored*/gdesktop_enums::TitlebarAction {
//    unsafe { TODO: call ffi:meta_prefs_get_action_middle_click_titlebar() }
//}

//#[doc(alias = "meta_prefs_get_action_right_click_titlebar")]
//pub fn prefs_get_action_right_click_titlebar() -> /*Ignored*/gdesktop_enums::TitlebarAction {
//    unsafe { TODO: call ffi:meta_prefs_get_action_right_click_titlebar() }
//}

#[doc(alias = "meta_prefs_get_attach_modal_dialogs")]
pub fn prefs_get_attach_modal_dialogs() -> bool {
    unsafe {
        from_glib(ffi::meta_prefs_get_attach_modal_dialogs())
    }
}

#[doc(alias = "meta_prefs_get_auto_maximize")]
pub fn prefs_get_auto_maximize() -> bool {
    unsafe {
        from_glib(ffi::meta_prefs_get_auto_maximize())
    }
}

#[doc(alias = "meta_prefs_get_auto_raise")]
pub fn prefs_get_auto_raise() -> bool {
    unsafe {
        from_glib(ffi::meta_prefs_get_auto_raise())
    }
}

#[doc(alias = "meta_prefs_get_auto_raise_delay")]
pub fn prefs_get_auto_raise_delay() -> i32 {
    unsafe {
        ffi::meta_prefs_get_auto_raise_delay()
    }
}

//#[doc(alias = "meta_prefs_get_button_layout")]
//pub fn prefs_get_button_layout(button_layout: /*Ignored*/ButtonLayout) {
//    unsafe { TODO: call ffi:meta_prefs_get_button_layout() }
//}

#[doc(alias = "meta_prefs_get_center_new_windows")]
pub fn prefs_get_center_new_windows() -> bool {
    unsafe {
        from_glib(ffi::meta_prefs_get_center_new_windows())
    }
}

#[doc(alias = "meta_prefs_get_check_alive_timeout")]
pub fn prefs_get_check_alive_timeout() -> u32 {
    unsafe {
        ffi::meta_prefs_get_check_alive_timeout()
    }
}

#[doc(alias = "meta_prefs_get_compositing_manager")]
pub fn prefs_get_compositing_manager() -> bool {
    unsafe {
        from_glib(ffi::meta_prefs_get_compositing_manager())
    }
}

#[doc(alias = "meta_prefs_get_cursor_size")]
pub fn prefs_get_cursor_size() -> i32 {
    unsafe {
        ffi::meta_prefs_get_cursor_size()
    }
}

#[doc(alias = "meta_prefs_get_cursor_theme")]
pub fn prefs_get_cursor_theme() -> Option<glib::GString> {
    unsafe {
        from_glib_none(ffi::meta_prefs_get_cursor_theme())
    }
}

#[doc(alias = "meta_prefs_get_disable_workarounds")]
pub fn prefs_get_disable_workarounds() -> bool {
    unsafe {
        from_glib(ffi::meta_prefs_get_disable_workarounds())
    }
}

#[doc(alias = "meta_prefs_get_drag_threshold")]
pub fn prefs_get_drag_threshold() -> i32 {
    unsafe {
        ffi::meta_prefs_get_drag_threshold()
    }
}

#[doc(alias = "meta_prefs_get_draggable_border_width")]
pub fn prefs_get_draggable_border_width() -> i32 {
    unsafe {
        ffi::meta_prefs_get_draggable_border_width()
    }
}

#[doc(alias = "meta_prefs_get_dynamic_workspaces")]
pub fn prefs_get_dynamic_workspaces() -> bool {
    unsafe {
        from_glib(ffi::meta_prefs_get_dynamic_workspaces())
    }
}

#[doc(alias = "meta_prefs_get_edge_tiling")]
pub fn prefs_get_edge_tiling() -> bool {
    unsafe {
        from_glib(ffi::meta_prefs_get_edge_tiling())
    }
}

#[doc(alias = "meta_prefs_get_focus_change_on_pointer_rest")]
pub fn prefs_get_focus_change_on_pointer_rest() -> bool {
    unsafe {
        from_glib(ffi::meta_prefs_get_focus_change_on_pointer_rest())
    }
}

//#[doc(alias = "meta_prefs_get_focus_mode")]
//pub fn prefs_get_focus_mode() -> /*Ignored*/gdesktop_enums::FocusMode {
//    unsafe { TODO: call ffi:meta_prefs_get_focus_mode() }
//}

//#[doc(alias = "meta_prefs_get_focus_new_windows")]
//pub fn prefs_get_focus_new_windows() -> /*Ignored*/gdesktop_enums::FocusNewWindows {
//    unsafe { TODO: call ffi:meta_prefs_get_focus_new_windows() }
//}

#[doc(alias = "meta_prefs_get_force_fullscreen")]
pub fn prefs_get_force_fullscreen() -> bool {
    unsafe {
        from_glib(ffi::meta_prefs_get_force_fullscreen())
    }
}

#[doc(alias = "meta_prefs_get_gnome_accessibility")]
pub fn prefs_get_gnome_accessibility() -> bool {
    unsafe {
        from_glib(ffi::meta_prefs_get_gnome_accessibility())
    }
}

#[doc(alias = "meta_prefs_get_gnome_animations")]
pub fn prefs_get_gnome_animations() -> bool {
    unsafe {
        from_glib(ffi::meta_prefs_get_gnome_animations())
    }
}

//#[doc(alias = "meta_prefs_get_keybinding_action")]
//pub fn prefs_get_keybinding_action(name: &str) -> /*Ignored*/KeyBindingAction {
//    unsafe { TODO: call ffi:meta_prefs_get_keybinding_action() }
//}

#[doc(alias = "meta_prefs_get_mouse_button_menu")]
pub fn prefs_get_mouse_button_menu() -> i32 {
    unsafe {
        ffi::meta_prefs_get_mouse_button_menu()
    }
}

//#[doc(alias = "meta_prefs_get_mouse_button_mods")]
//pub fn prefs_get_mouse_button_mods() -> /*Ignored*/VirtualModifier {
//    unsafe { TODO: call ffi:meta_prefs_get_mouse_button_mods() }
//}

#[doc(alias = "meta_prefs_get_mouse_button_resize")]
pub fn prefs_get_mouse_button_resize() -> i32 {
    unsafe {
        ffi::meta_prefs_get_mouse_button_resize()
    }
}

#[doc(alias = "meta_prefs_get_num_workspaces")]
pub fn prefs_get_num_workspaces() -> i32 {
    unsafe {
        ffi::meta_prefs_get_num_workspaces()
    }
}

#[doc(alias = "meta_prefs_get_raise_on_click")]
pub fn prefs_get_raise_on_click() -> bool {
    unsafe {
        from_glib(ffi::meta_prefs_get_raise_on_click())
    }
}

#[doc(alias = "meta_prefs_get_show_fallback_app_menu")]
pub fn prefs_get_show_fallback_app_menu() -> bool {
    unsafe {
        from_glib(ffi::meta_prefs_get_show_fallback_app_menu())
    }
}

//#[doc(alias = "meta_prefs_get_titlebar_font")]
//pub fn prefs_get_titlebar_font() -> /*Ignored*/Option<pango::FontDescription> {
//    unsafe { TODO: call ffi:meta_prefs_get_titlebar_font() }
//}

#[doc(alias = "meta_prefs_get_visual_bell")]
pub fn prefs_get_visual_bell() -> bool {
    unsafe {
        from_glib(ffi::meta_prefs_get_visual_bell())
    }
}

//#[doc(alias = "meta_prefs_get_visual_bell_type")]
//pub fn prefs_get_visual_bell_type() -> /*Ignored*/gdesktop_enums::VisualBellType {
//    unsafe { TODO: call ffi:meta_prefs_get_visual_bell_type() }
//}

#[doc(alias = "meta_prefs_get_workspace_name")]
pub fn prefs_get_workspace_name(i: i32) -> Option<glib::GString> {
    unsafe {
        from_glib_none(ffi::meta_prefs_get_workspace_name(i))
    }
}

#[doc(alias = "meta_prefs_get_workspaces_only_on_primary")]
pub fn prefs_get_workspaces_only_on_primary() -> bool {
    unsafe {
        from_glib(ffi::meta_prefs_get_workspaces_only_on_primary())
    }
}

#[doc(alias = "meta_prefs_init")]
pub fn prefs_init() {
    unsafe {
        ffi::meta_prefs_init();
    }
}

//#[doc(alias = "meta_prefs_remove_listener")]
//pub fn prefs_remove_listener(func: /*Unimplemented*/Fn(/*Ignored*/Preference), user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
//    unsafe { TODO: call ffi:meta_prefs_remove_listener() }
//}

#[doc(alias = "meta_prefs_set_force_fullscreen")]
pub fn prefs_set_force_fullscreen(whether: bool) {
    unsafe {
        ffi::meta_prefs_set_force_fullscreen(whether.into_glib());
    }
}

#[doc(alias = "meta_prefs_set_num_workspaces")]
pub fn prefs_set_num_workspaces(n_workspaces: i32) {
    unsafe {
        ffi::meta_prefs_set_num_workspaces(n_workspaces);
    }
}

#[doc(alias = "meta_prefs_set_show_fallback_app_menu")]
pub fn prefs_set_show_fallback_app_menu(whether: bool) {
    unsafe {
        ffi::meta_prefs_set_show_fallback_app_menu(whether.into_glib());
    }
}

#[doc(alias = "meta_push_no_msg_prefix")]
pub fn push_no_msg_prefix() {
    unsafe {
        ffi::meta_push_no_msg_prefix();
    }
}

/// Stops Mutter. This tells the event loop to stop processing; it is
/// rather dangerous to use this because this will leave the user with
/// no window manager. We generally do this only if, for example, the
/// session manager asks us to; we assume the session manager knows
/// what it's talking about.
/// ## `code`
/// The success or failure code to return to the calling process.
#[doc(alias = "meta_quit")]
pub fn quit(code: ExitCode) {
    unsafe {
        ffi::meta_quit(code.into_glib());
    }
}

/// Registers mutter with the session manager. Call this after completing your own
/// initialization.
///
/// This should be called when the session manager can safely continue to the
/// next phase of startup and potentially display windows.
#[doc(alias = "meta_register_with_session")]
pub fn register_with_session() {
    unsafe {
        ffi::meta_register_with_session();
    }
}

//#[doc(alias = "meta_remove_clutter_debug_flags")]
//pub fn remove_clutter_debug_flags(debug_flags: /*Ignored*/clutter::DebugFlag, draw_flags: /*Ignored*/clutter::DrawDebugFlag, pick_flags: /*Ignored*/clutter::PickDebugFlag) {
//    unsafe { TODO: call ffi:meta_remove_clutter_debug_flags() }
//}

//#[doc(alias = "meta_remove_debug_paint_flag")]
//pub fn remove_debug_paint_flag(flag: /*Ignored*/DebugPaintFlag) {
//    unsafe { TODO: call ffi:meta_remove_debug_paint_flag() }
//}

//#[doc(alias = "meta_remove_verbose_topic")]
//pub fn remove_verbose_topic(topic: /*Ignored*/DebugTopic) {
//    unsafe { TODO: call ffi:meta_remove_verbose_topic() }
//}

/// Starts the process of restarting the compositor. Note that Mutter's
/// involvement here is to make the restart visually smooth for the
/// user - it cannot itself safely reexec a program that embeds libmuttter.
/// So in order for this to work, the compositor must handle two
/// signals - MetaDisplay::show-restart-message, to display the
/// message passed here on the Clutter stage, and ::restart to actually
/// reexec the compositor.
/// ## `message`
/// message to display to the user, or [`None`]
#[doc(alias = "meta_restart")]
pub fn restart(message: Option<&str>) {
    unsafe {
        ffi::meta_restart(message.to_glib_none().0);
    }
}

/// Runs mutter. Call this after completing initialization that doesn't require
/// an event loop.
///
/// # Returns
///
/// mutter's exit status
#[doc(alias = "meta_run")]
pub fn run() -> i32 {
    unsafe {
        ffi::meta_run()
    }
}

#[doc(alias = "meta_run_main_loop")]
pub fn run_main_loop() {
    unsafe {
        ffi::meta_run_main_loop();
    }
}

/// Set the value to use for the _GNOME_WM_KEYBINDINGS property. To take
/// effect, it is necessary to call this function before [`init()`][crate::init()].
/// ## `wm_keybindings`
/// value for _GNOME_WM_KEYBINDINGS
#[doc(alias = "meta_set_gnome_wm_keybindings")]
pub fn set_gnome_wm_keybindings(wm_keybindings: &str) {
    unsafe {
        ffi::meta_set_gnome_wm_keybindings(wm_keybindings.to_glib_none().0);
    }
}

/// Set the value to use for the _NET_WM_NAME property. To take effect,
/// it is necessary to call this function before [`init()`][crate::init()].
/// ## `wm_name`
/// value for _NET_WM_NAME
#[doc(alias = "meta_set_wm_name")]
pub fn set_wm_name(wm_name: &str) {
    unsafe {
        ffi::meta_set_wm_name(wm_name.to_glib_none().0);
    }
}

//#[doc(alias = "meta_show_dialog")]
//pub fn show_dialog(type_: &str, message: &str, timeout: &str, display: &str, ok_text: &str, cancel_text: &str, icon_name: &str, transient_for: i32, columns: /*Unimplemented*/&[&Fundamental: Pointer], entries: /*Unimplemented*/&[&Fundamental: Pointer]) -> glib::Pid {
//    unsafe { TODO: call ffi:meta_show_dialog() }
//}

#[doc(alias = "meta_start")]
pub fn start() {
    unsafe {
        ffi::meta_start();
    }
}

#[doc(alias = "meta_test_init")]
pub fn test_init() {
    unsafe {
        ffi::meta_test_init();
    }
}

//#[doc(alias = "meta_topic_real")]
//pub fn topic_real(topic: /*Ignored*/DebugTopic, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call ffi:meta_topic_real() }
//}

//#[doc(alias = "meta_unsigned_long_equal")]
//pub fn unsigned_long_equal(v1: /*Unimplemented*/Option<Fundamental: Pointer>, v2: /*Unimplemented*/Option<Fundamental: Pointer>) -> i32 {
//    unsafe { TODO: call ffi:meta_unsigned_long_equal() }
//}

//#[doc(alias = "meta_unsigned_long_hash")]
//pub fn unsigned_long_hash(v: /*Unimplemented*/Option<Fundamental: Pointer>) -> u32 {
//    unsafe { TODO: call ffi:meta_unsigned_long_hash() }
//}

//#[doc(alias = "meta_verbose_real")]
//pub fn verbose_real(format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call ffi:meta_verbose_real() }
//}

//#[doc(alias = "meta_warning")]
//pub fn warning(format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call ffi:meta_warning() }
//}

//#[doc(alias = "meta_x11_error_trap_pop")]
//pub fn x11_error_trap_pop(x11_display: /*Ignored*/&X11Display) {
//    unsafe { TODO: call ffi:meta_x11_error_trap_pop() }
//}

//#[doc(alias = "meta_x11_error_trap_pop_with_return")]
//pub fn x11_error_trap_pop_with_return(x11_display: /*Ignored*/&X11Display) -> i32 {
//    unsafe { TODO: call ffi:meta_x11_error_trap_pop_with_return() }
//}

//#[doc(alias = "meta_x11_error_trap_push")]
//pub fn x11_error_trap_push(x11_display: /*Ignored*/&X11Display) {
//    unsafe { TODO: call ffi:meta_x11_error_trap_push() }
//}

#[doc(alias = "meta_x11_init_gdk_display")]
pub fn x11_init_gdk_display() -> Result<(), glib::Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let _ = ffi::meta_x11_init_gdk_display(&mut error);
        if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
    }
}
