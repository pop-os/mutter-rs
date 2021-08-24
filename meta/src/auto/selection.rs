// Generated by gir (https://github.com/gtk-rs/gir @ 5bbf6cb)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 8e47c67)
// from mutter-gir-files
// DO NOT EDIT

use crate::Display;
use crate::SelectionSource;
use crate::SelectionType;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "MetaSelection")]
    pub struct Selection(Object<ffi::MetaSelection, ffi::MetaSelectionClass>);

    match fn {
        type_ => || ffi::meta_selection_get_type(),
    }
}

impl Selection {
    #[doc(alias = "meta_selection_new")]
    pub fn new(display: &Display) -> Selection {
        unsafe {
            from_glib_full(ffi::meta_selection_new(display.to_glib_none().0))
        }
    }

    /// Returns the list of supported mimetypes for the given selection type.
    /// ## `selection_type`
    /// Selection to query
    ///
    /// # Returns
    ///
    /// The supported mimetypes
    #[doc(alias = "meta_selection_get_mimetypes")]
    #[doc(alias = "get_mimetypes")]
    pub fn mimetypes(&self, selection_type: SelectionType) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::meta_selection_get_mimetypes(self.to_glib_none().0, selection_type.into_glib()))
        }
    }

    /// Sets `owner` as the owner of the selection given by `selection_type`,
    /// unsets any previous owner there was.
    /// ## `selection_type`
    /// Selection type
    /// ## `owner`
    /// New selection owner
    #[doc(alias = "meta_selection_set_owner")]
    pub fn set_owner<P: IsA<SelectionSource>>(&self, selection_type: SelectionType, owner: &P) {
        unsafe {
            ffi::meta_selection_set_owner(self.to_glib_none().0, selection_type.into_glib(), owner.as_ref().to_glib_none().0);
        }
    }

    /// Requests a transfer of `mimetype` on the selection given by
    /// `selection_type`.
    /// ## `selection_type`
    /// Selection type
    /// ## `mimetype`
    /// Mimetype to transfer
    /// ## `size`
    /// Maximum size to transfer, -1 for unlimited
    /// ## `output`
    /// Output stream to write contents to
    /// ## `cancellable`
    /// Cancellable
    /// ## `callback`
    /// User callback
    #[doc(alias = "meta_selection_transfer_async")]
    pub fn transfer_async<P: IsA<gio::OutputStream>, Q: IsA<gio::Cancellable>, R: FnOnce(Result<(), glib::Error>) + Send + 'static>(&self, selection_type: SelectionType, mimetype: &str, size: isize, output: &P, cancellable: Option<&Q>, callback: R) {
        let user_data: Box_<R> = Box_::new(callback);
        unsafe extern "C" fn transfer_async_trampoline<R: FnOnce(Result<(), glib::Error>) + Send + 'static>(_source_object: *mut glib::gobject_ffi::GObject, res: *mut gio::ffi::GAsyncResult, user_data: glib::ffi::gpointer) {
            let mut error = ptr::null_mut();
            let _ = ffi::meta_selection_transfer_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box_<R> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = transfer_async_trampoline::<R>;
        unsafe {
            ffi::meta_selection_transfer_async(self.to_glib_none().0, selection_type.into_glib(), mimetype.to_glib_none().0, size, output.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box_::into_raw(user_data) as *mut _);
        }
    }

    
    pub fn transfer_async_future<P: IsA<gio::OutputStream> + Clone + 'static>(&self, selection_type: SelectionType, mimetype: &str, size: isize, output: &P) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {

        let mimetype = String::from(mimetype);
        let output = output.clone();
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.transfer_async(
                selection_type,
                &mimetype,
                size,
                &output,
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    /// Unsets `owner` as the owner the selection given by `selection_type`. If
    /// `owner` does not own the selection, nothing is done.
    /// ## `selection_type`
    /// Selection type
    /// ## `owner`
    /// Owner to unset
    #[doc(alias = "meta_selection_unset_owner")]
    pub fn unset_owner<P: IsA<SelectionSource>>(&self, selection_type: SelectionType, owner: &P) {
        unsafe {
            ffi::meta_selection_unset_owner(self.to_glib_none().0, selection_type.into_glib(), owner.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "owner-changed")]
    pub fn connect_owner_changed<F: Fn(&Self, u32, &SelectionSource) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn owner_changed_trampoline<F: Fn(&Selection, u32, &SelectionSource) + 'static>(this: *mut ffi::MetaSelection, object: libc::c_uint, p0: *mut ffi::MetaSelectionSource, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), object, &from_glib_borrow(p0))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"owner-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(owner_changed_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Selection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Selection")
    }
}