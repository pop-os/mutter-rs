// Generated by gir (https://github.com/gtk-rs/gir @ b193568)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 8e47c67)
// from mutter-gir-files
// DO NOT EDIT

use crate::DisplayCorner;
use crate::MotionDirection;
use crate::Workspace;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "MetaWorkspaceManager")]
    pub struct WorkspaceManager(Object<ffi::MetaWorkspaceManager, ffi::MetaWorkspaceManagerClass>);

    match fn {
        type_ => || ffi::meta_workspace_manager_get_type(),
    }
}

impl WorkspaceManager {
    /// Append a new workspace to the workspace manager and (optionally) switch to that
    /// display.
    /// ## `activate`
    /// [`true`] if the workspace should be switched to after creation
    /// ## `timestamp`
    /// if switching to a new workspace, timestamp to be used when
    ///  focusing a window on the new workspace. (Doesn't hurt to pass a valid
    ///  timestamp when available even if not switching workspaces.)
    ///
    /// # Returns
    ///
    /// the newly appended workspace.
    #[doc(alias = "meta_workspace_manager_append_new_workspace")]
    pub fn append_new_workspace(&self, activate: bool, timestamp: u32) -> Option<Workspace> {
        unsafe {
            from_glib_none(ffi::meta_workspace_manager_append_new_workspace(self.to_glib_none().0, activate.into_glib(), timestamp))
        }
    }

    ///
    /// # Returns
    ///
    /// The current workspace
    #[doc(alias = "meta_workspace_manager_get_active_workspace")]
    #[doc(alias = "get_active_workspace")]
    pub fn active_workspace(&self) -> Option<Workspace> {
        unsafe {
            from_glib_none(ffi::meta_workspace_manager_get_active_workspace(self.to_glib_none().0))
        }
    }

    #[doc(alias = "meta_workspace_manager_get_active_workspace_index")]
    #[doc(alias = "get_active_workspace_index")]
    pub fn active_workspace_index(&self) -> i32 {
        unsafe {
            ffi::meta_workspace_manager_get_active_workspace_index(self.to_glib_none().0)
        }
    }

    #[doc(alias = "meta_workspace_manager_get_n_workspaces")]
    #[doc(alias = "get_n_workspaces")]
    pub fn n_workspaces(&self) -> i32 {
        unsafe {
            ffi::meta_workspace_manager_get_n_workspaces(self.to_glib_none().0)
        }
    }

    /// Gets the workspace object for one of a workspace manager's workspaces given the workspace
    /// index. It's valid to call this function with an out-of-range index and it
    /// will robustly return [`None`].
    /// ## `index`
    /// index of one of the display's workspaces
    ///
    /// # Returns
    ///
    /// the workspace object with specified
    ///  index, or [`None`] if the index is out of range.
    #[doc(alias = "meta_workspace_manager_get_workspace_by_index")]
    #[doc(alias = "get_workspace_by_index")]
    pub fn workspace_by_index(&self, index: i32) -> Option<Workspace> {
        unsafe {
            from_glib_none(ffi::meta_workspace_manager_get_workspace_by_index(self.to_glib_none().0, index))
        }
    }

    ///
    /// # Returns
    ///
    /// The workspaces for `display`
    #[doc(alias = "meta_workspace_manager_get_workspaces")]
    #[doc(alias = "get_workspaces")]
    pub fn workspaces(&self) -> Vec<Workspace> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::meta_workspace_manager_get_workspaces(self.to_glib_none().0))
        }
    }

    /// Explicitly set the layout of workspaces. Once this has been called, the contents of the
    /// _NET_DESKTOP_LAYOUT property on the root window are completely ignored.
    /// ## `starting_corner`
    /// the corner at which the first workspace is found
    /// ## `vertical_layout`
    /// if [`true`] the workspaces are laid out in columns rather than rows
    /// ## `n_rows`
    /// number of rows of workspaces, or -1 to determine the number of rows from
    ///  `n_columns` and the total number of workspaces
    /// ## `n_columns`
    /// number of columns of workspaces, or -1 to determine the number of columns from
    ///  `n_rows` and the total number of workspaces
    #[doc(alias = "meta_workspace_manager_override_workspace_layout")]
    pub fn override_workspace_layout(&self, starting_corner: DisplayCorner, vertical_layout: bool, n_rows: i32, n_columns: i32) {
        unsafe {
            ffi::meta_workspace_manager_override_workspace_layout(self.to_glib_none().0, starting_corner.into_glib(), vertical_layout.into_glib(), n_rows, n_columns);
        }
    }

    #[doc(alias = "meta_workspace_manager_remove_workspace")]
    pub fn remove_workspace(&self, workspace: &Workspace, timestamp: u32) {
        unsafe {
            ffi::meta_workspace_manager_remove_workspace(self.to_glib_none().0, workspace.to_glib_none().0, timestamp);
        }
    }

    /// Reorder a workspace to a new index. If the workspace is currently active
    /// the "active-workspace-changed" signal will be emitted.
    /// If the workspace's index is the same as `new_index` or the workspace
    /// will not be found in the list, this function will return.
    ///
    /// Calling this function will also emit the "workspaces-reordered" signal.
    /// ## `workspace`
    /// a [`Workspace`][crate::Workspace] to reorder
    /// ## `new_index`
    /// the new index of the passed workspace
    #[doc(alias = "meta_workspace_manager_reorder_workspace")]
    pub fn reorder_workspace(&self, workspace: &Workspace, new_index: i32) {
        unsafe {
            ffi::meta_workspace_manager_reorder_workspace(self.to_glib_none().0, workspace.to_glib_none().0, new_index);
        }
    }

    #[doc(alias = "layout-columns")]
    pub fn layout_columns(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.as_ptr() as *mut glib::gobject_ffi::GObject, b"layout-columns\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `layout-columns` getter")
        }
    }

    #[doc(alias = "layout-rows")]
    pub fn layout_rows(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.as_ptr() as *mut glib::gobject_ffi::GObject, b"layout-rows\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `layout-rows` getter")
        }
    }

    #[doc(alias = "active-workspace-changed")]
    pub fn connect_active_workspace_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn active_workspace_changed_trampoline<F: Fn(&WorkspaceManager) + 'static>(this: *mut ffi::MetaWorkspaceManager, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"active-workspace-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(active_workspace_changed_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "showing-desktop-changed")]
    pub fn connect_showing_desktop_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn showing_desktop_changed_trampoline<F: Fn(&WorkspaceManager) + 'static>(this: *mut ffi::MetaWorkspaceManager, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"showing-desktop-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(showing_desktop_changed_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "workspace-added")]
    pub fn connect_workspace_added<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn workspace_added_trampoline<F: Fn(&WorkspaceManager, i32) + 'static>(this: *mut ffi::MetaWorkspaceManager, object: libc::c_int, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), object)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"workspace-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(workspace_added_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "workspace-removed")]
    pub fn connect_workspace_removed<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn workspace_removed_trampoline<F: Fn(&WorkspaceManager, i32) + 'static>(this: *mut ffi::MetaWorkspaceManager, object: libc::c_int, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), object)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"workspace-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(workspace_removed_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "workspace-switched")]
    pub fn connect_workspace_switched<F: Fn(&Self, i32, i32, MotionDirection) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn workspace_switched_trampoline<F: Fn(&WorkspaceManager, i32, i32, MotionDirection) + 'static>(this: *mut ffi::MetaWorkspaceManager, object: libc::c_int, p0: libc::c_int, p1: ffi::MetaMotionDirection, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), object, p0, from_glib(p1))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"workspace-switched\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(workspace_switched_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "workspaces-reordered")]
    pub fn connect_workspaces_reordered<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn workspaces_reordered_trampoline<F: Fn(&WorkspaceManager) + 'static>(this: *mut ffi::MetaWorkspaceManager, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"workspaces-reordered\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(workspaces_reordered_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "layout-columns")]
    pub fn connect_layout_columns_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_layout_columns_trampoline<F: Fn(&WorkspaceManager) + 'static>(this: *mut ffi::MetaWorkspaceManager, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::layout-columns\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_layout_columns_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "layout-rows")]
    pub fn connect_layout_rows_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_layout_rows_trampoline<F: Fn(&WorkspaceManager) + 'static>(this: *mut ffi::MetaWorkspaceManager, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::layout-rows\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_layout_rows_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "n-workspaces")]
    pub fn connect_n_workspaces_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_n_workspaces_trampoline<F: Fn(&WorkspaceManager) + 'static>(this: *mut ffi::MetaWorkspaceManager, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::n-workspaces\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_n_workspaces_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for WorkspaceManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WorkspaceManager")
    }
}
