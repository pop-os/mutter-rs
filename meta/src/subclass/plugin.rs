use crate::Plugin;
use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;
use libc::c_int;
use std::ptr;

// TODO default implementation? just empty? Required?
pub trait PluginImpl: ObjectImpl {
    fn start(&self, plugin: &Self::Type);
    fn minimize(&self, plugin: &Self::Type, actor: &crate::WindowActor);
    fn unminimize(&self, plugin: &Self::Type, actor: &crate::WindowActor);
    fn size_changed(&self, plugin: &Self::Type, actor: &crate::WindowActor);
    // size_change
    fn map(&self, plugin: &Self::Type, actor: &crate::WindowActor);
    fn destroy(&self, plugin: &Self::Type, actor: &crate::WindowActor);
    fn switch_workspace(
        &self,
        plugin: &Self::Type,
        from: i32,
        to: i32,
        direction: crate::MotionDirection,
    );
    fn show_tile_preview(
        &self,
        plugin: &Self::Type,
        window: &crate::Window,
        rect: &crate::Rectangle,
        monitor_number: i32,
    );
    fn hide_tile_preview(&self, plugin: &Self::Type);
    // show_window_menu
    // show_window_menu_for_rect
    fn kill_window_effects(&self, plugin: &Self::Type, actor: &crate::WindowActor);
    fn kill_switch_workspace(&self, plugin: &Self::Type);
    // xevent_filter
    fn keybinding_filter(&self, plugin: &Self::Type, key_binding: &crate::KeyBinding) -> bool;
    fn confirm_display_change(&self, plugin: &Self::Type);
    // XXX
    fn plugin_info(&self, plugin: &Self::Type) -> Option<&'static ffi::MetaPluginInfo>;
    // create_close_dialog
    // create_inhibit_shortcuts_dialog
    // locate_pointer
}

unsafe impl<T: PluginImpl> IsSubclassable<T> for Plugin {
    fn class_init(class: &mut ::glib::Class<Self>) {
        <glib::Object as IsSubclassable<T>>::class_init(class);

        let klass = class.as_mut();
        klass.start = Some(plugin_start::<T>);
        klass.minimize = Some(plugin_minimize::<T>);
        klass.unminimize = Some(plugin_unminimize::<T>);
        klass.size_changed = Some(plugin_size_changed::<T>);
        klass.map = Some(plugin_map::<T>);
        klass.destroy = Some(plugin_destroy::<T>);
        klass.switch_workspace = Some(plugin_switch_workspace::<T>);
        klass.show_tile_preview = Some(plugin_show_tile_preview::<T>);
        klass.hide_tile_preview = Some(plugin_hide_tile_preview::<T>);
        klass.kill_window_effects = Some(plugin_kill_window_effects::<T>);
        klass.kill_switch_workspace = Some(plugin_kill_switch_workspace::<T>);
        klass.keybinding_filter = Some(plugin_keybinding_filter::<T>);
        klass.confirm_display_change = Some(plugin_confirm_display_change::<T>);
        klass.plugin_info = Some(plugin_plugin_info::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <glib::Object as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn plugin_start<T: PluginImpl>(ptr: *mut ffi::MetaPlugin) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Plugin> = from_glib_borrow(ptr);

    imp.start(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn plugin_minimize<T: PluginImpl>(
    ptr: *mut ffi::MetaPlugin,
    actor: *mut ffi::MetaWindowActor,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Plugin> = from_glib_borrow(ptr);

    imp.minimize(wrap.unsafe_cast_ref(), &from_glib_borrow(actor))
}

unsafe extern "C" fn plugin_unminimize<T: PluginImpl>(
    ptr: *mut ffi::MetaPlugin,
    actor: *mut ffi::MetaWindowActor,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Plugin> = from_glib_borrow(ptr);

    imp.unminimize(wrap.unsafe_cast_ref(), &from_glib_borrow(actor))
}

unsafe extern "C" fn plugin_size_changed<T: PluginImpl>(
    ptr: *mut ffi::MetaPlugin,
    actor: *mut ffi::MetaWindowActor,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Plugin> = from_glib_borrow(ptr);

    imp.size_changed(wrap.unsafe_cast_ref(), &from_glib_borrow(actor))
}

unsafe extern "C" fn plugin_map<T: PluginImpl>(
    ptr: *mut ffi::MetaPlugin,
    actor: *mut ffi::MetaWindowActor,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Plugin> = from_glib_borrow(ptr);

    imp.map(wrap.unsafe_cast_ref(), &from_glib_borrow(actor))
}

unsafe extern "C" fn plugin_destroy<T: PluginImpl>(
    ptr: *mut ffi::MetaPlugin,
    actor: *mut ffi::MetaWindowActor,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Plugin> = from_glib_borrow(ptr);

    imp.destroy(wrap.unsafe_cast_ref(), &from_glib_borrow(actor))
}

unsafe extern "C" fn plugin_switch_workspace<T: PluginImpl>(
    ptr: *mut ffi::MetaPlugin,
    from: c_int,
    to: c_int,
    direction: ffi::MetaMotionDirection,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Plugin> = from_glib_borrow(ptr);

    imp.switch_workspace(
        wrap.unsafe_cast_ref(),
        from as i32,
        to as i32,
        crate::MotionDirection::from_glib(direction),
    )
}

unsafe extern "C" fn plugin_show_tile_preview<T: PluginImpl>(
    ptr: *mut ffi::MetaPlugin,
    window: *mut ffi::MetaWindow,
    rect: *mut ffi::MetaRectangle,
    monitor_number: c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Plugin> = from_glib_borrow(ptr);

    imp.show_tile_preview(
        wrap.unsafe_cast_ref(),
        &from_glib_borrow(window),
        &from_glib_borrow(rect),
        monitor_number as i32,
    )
}

unsafe extern "C" fn plugin_hide_tile_preview<T: PluginImpl>(ptr: *mut ffi::MetaPlugin) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Plugin> = from_glib_borrow(ptr);

    imp.hide_tile_preview(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn plugin_kill_window_effects<T: PluginImpl>(
    ptr: *mut ffi::MetaPlugin,
    actor: *mut ffi::MetaWindowActor,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Plugin> = from_glib_borrow(ptr);

    imp.kill_window_effects(wrap.unsafe_cast_ref(), &from_glib_borrow(actor))
}

unsafe extern "C" fn plugin_kill_switch_workspace<T: PluginImpl>(ptr: *mut ffi::MetaPlugin) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Plugin> = from_glib_borrow(ptr);

    imp.kill_switch_workspace(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn plugin_keybinding_filter<T: PluginImpl>(
    ptr: *mut ffi::MetaPlugin,
    key_binding: *mut ffi::MetaKeyBinding,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Plugin> = from_glib_borrow(ptr);

    imp.keybinding_filter(wrap.unsafe_cast_ref(), &from_glib_borrow(key_binding))
        .into_glib()
}

unsafe extern "C" fn plugin_confirm_display_change<T: PluginImpl>(ptr: *mut ffi::MetaPlugin) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Plugin> = from_glib_borrow(ptr);

    imp.confirm_display_change(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn plugin_plugin_info<T: PluginImpl>(
    ptr: *mut ffi::MetaPlugin,
) -> *const ffi::MetaPluginInfo {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Plugin> = from_glib_borrow(ptr);

    if let Some(info) = imp.plugin_info(wrap.unsafe_cast_ref()) {
        info
    } else {
        ptr::null()
    }
}
