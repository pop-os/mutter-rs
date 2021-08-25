// Generated by gir (https://github.com/gtk-rs/gir @ 5bbf6cb)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 8e47c67)
// from mutter-gir-files
// DO NOT EDIT

use crate::Actor;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "ClutterContainer")]
    pub struct Container(Interface<ffi::ClutterContainer, ffi::ClutterContainerIface>);

    match fn {
        type_ => || ffi::clutter_container_get_type(),
    }
}

impl Container {
    //#[cfg(any(feature = "v0_8", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8")))]
    //#[doc(alias = "clutter_container_class_find_child_property")]
    //pub fn class_find_child_property(klass: /*Ignored*/&mut glib::ObjectClass, property_name: &str) -> /*Ignored*/Option<glib::ParamSpec> {
    //    unsafe { TODO: call ffi:clutter_container_class_find_child_property() }
    //}

    //#[cfg(any(feature = "v0_8", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8")))]
    //#[doc(alias = "clutter_container_class_list_child_properties")]
    //pub fn class_list_child_properties(klass: /*Ignored*/&mut glib::ObjectClass) -> /*Ignored*/Vec<glib::ParamSpec> {
    //    unsafe { TODO: call ffi:clutter_container_class_list_child_properties() }
    //}
}

pub const NONE_CONTAINER: Option<&Container> = None;

/// Trait containing all [`struct@Container`] methods.
///
/// # Implementors
///
/// [`Actor`][struct@crate::Actor], [`Container`][struct@crate::Container], [`Stage`][struct@crate::Stage], [`Text`][struct@crate::Text]
pub trait ContainerExt: 'static {
    //#[cfg_attr(feature = "v1_10", deprecated = "Since 1.10")]
    //#[doc(alias = "clutter_container_add")]
    //fn add<P: IsA<Actor>>(&self, first_actor: &P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    /// Adds a [`Actor`][crate::Actor] to `self`. This function will emit the
    /// "actor-added" signal. The actor should be parented to
    /// `self`. You cannot add a [`Actor`][crate::Actor] to more than one
    /// [`Container`][crate::Container].
    ///
    /// This function will call `ClutterContainerIface.add()`, which is a
    /// deprecated virtual function. The default implementation will
    /// call [`ActorExt::add_child()`][crate::prelude::ActorExt::add_child()].
    ///
    /// # Deprecated since 1.10
    ///
    /// Use [`ActorExt::add_child()`][crate::prelude::ActorExt::add_child()] instead.
    /// ## `actor`
    /// the first [`Actor`][crate::Actor] to add
    #[cfg_attr(feature = "v1_10", deprecated = "Since 1.10")]
    #[doc(alias = "clutter_container_add_actor")]
    fn add_actor<P: IsA<Actor>>(&self, actor: &P);

    //#[cfg(any(feature = "v0_8", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8")))]
    //#[doc(alias = "clutter_container_child_get")]
    //fn child_get<P: IsA<Actor>>(&self, actor: &P, first_prop: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //#[cfg(any(feature = "v0_8", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8")))]
    //#[doc(alias = "clutter_container_child_get_property")]
    //fn child_get_property<P: IsA<Actor>>(&self, child: &P, property: &str, value: /*Ignored*/&mut glib::Value);

    //#[cfg(any(feature = "v1_6", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_6")))]
    //#[doc(alias = "clutter_container_child_notify")]
    //fn child_notify<P: IsA<Actor>>(&self, child: &P, pspec: /*Ignored*/&glib::ParamSpec);

    //#[cfg(any(feature = "v0_8", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8")))]
    //#[doc(alias = "clutter_container_child_set")]
    //fn child_set<P: IsA<Actor>>(&self, actor: &P, first_prop: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //#[cfg(any(feature = "v0_8", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8")))]
    //#[doc(alias = "clutter_container_child_set_property")]
    //fn child_set_property<P: IsA<Actor>>(&self, child: &P, property: &str, value: /*Ignored*/&glib::Value);

    /// Creates the `ClutterChildMeta` wrapping `actor` inside the
    /// `self`, if the `ClutterContainerIface::child_meta_type`
    /// class member is not set to `G_TYPE_INVALID`.
    ///
    /// This function is only useful when adding a [`Actor`][crate::Actor] to
    /// a [`Container`][crate::Container] implementation outside of the
    /// `ClutterContainer::add()` virtual function implementation.
    ///
    /// Applications should not call this function.
    /// ## `actor`
    /// a [`Actor`][crate::Actor]
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "clutter_container_create_child_meta")]
    fn create_child_meta<P: IsA<Actor>>(&self, actor: &P);

    /// Destroys the `ClutterChildMeta` wrapping `actor` inside the
    /// `self`, if any.
    ///
    /// This function is only useful when removing a [`Actor`][crate::Actor] to
    /// a [`Container`][crate::Container] implementation outside of the
    /// `ClutterContainer::add()` virtual function implementation.
    ///
    /// Applications should not call this function.
    /// ## `actor`
    /// a [`Actor`][crate::Actor]
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "clutter_container_destroy_child_meta")]
    fn destroy_child_meta<P: IsA<Actor>>(&self, actor: &P);

    /// Finds a child actor of a container by its name. Search recurses
    /// into any child container.
    /// ## `child_name`
    /// the name of the requested child.
    ///
    /// # Returns
    ///
    /// The child actor with the requested name,
    ///  or [`None`] if no actor with that name was found.
    #[cfg(any(feature = "v0_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_6")))]
    #[doc(alias = "clutter_container_find_child_by_name")]
    fn find_child_by_name(&self, child_name: &str) -> Option<Actor>;

    //#[cfg(any(feature = "v0_8", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8")))]
    //#[doc(alias = "clutter_container_get_child_meta")]
    //#[doc(alias = "get_child_meta")]
    //fn child_meta<P: IsA<Actor>>(&self, actor: &P) -> /*Ignored*/Option<ChildMeta>;

    /// Retrieves all the children of `self`.
    ///
    /// # Deprecated since 1.10
    ///
    /// Use [`ActorExt::children()`][crate::prelude::ActorExt::children()] instead.
    ///
    /// # Returns
    ///
    /// a list
    ///  of [`Actor`][crate::Actor]<!-- -->s. Use `g_list_free()` on the returned
    ///  list when done.
    #[cfg_attr(feature = "v1_10", deprecated = "Since 1.10")]
    #[doc(alias = "clutter_container_get_children")]
    #[doc(alias = "get_children")]
    fn children(&self) -> Vec<Actor>;

    /// Lowers `actor` to `sibling` level, in the depth ordering.
    ///
    /// This function calls the `ClutterContainerIface.lower()` virtual function,
    /// which has been deprecated. The default implementation will call
    /// [`ActorExt::set_child_below_sibling()`][crate::prelude::ActorExt::set_child_below_sibling()].
    ///
    /// # Deprecated since 1.10
    ///
    /// Use [`ActorExt::set_child_below_sibling()`][crate::prelude::ActorExt::set_child_below_sibling()] instead.
    /// ## `actor`
    /// the actor to raise
    /// ## `sibling`
    /// the sibling to lower to, or [`None`] to lower
    ///  to the bottom
    #[cfg_attr(feature = "v1_10", deprecated = "Since 1.10")]
    #[cfg(any(feature = "v0_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_6")))]
    #[doc(alias = "clutter_container_lower_child")]
    fn lower_child<P: IsA<Actor>, Q: IsA<Actor>>(&self, actor: &P, sibling: Option<&Q>);

    /// Raises `actor` to `sibling` level, in the depth ordering.
    ///
    /// This function calls the `ClutterContainerIface.raise()` virtual function,
    /// which has been deprecated. The default implementation will call
    /// [`ActorExt::set_child_above_sibling()`][crate::prelude::ActorExt::set_child_above_sibling()].
    ///
    /// # Deprecated since 1.10
    ///
    /// Use [`ActorExt::set_child_above_sibling()`][crate::prelude::ActorExt::set_child_above_sibling()] instead.
    /// ## `actor`
    /// the actor to raise
    /// ## `sibling`
    /// the sibling to raise to, or [`None`] to raise
    ///  to the top
    #[cfg_attr(feature = "v1_10", deprecated = "Since 1.10")]
    #[cfg(any(feature = "v0_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_6")))]
    #[doc(alias = "clutter_container_raise_child")]
    fn raise_child<P: IsA<Actor>, Q: IsA<Actor>>(&self, actor: &P, sibling: Option<&Q>);

    //#[cfg_attr(feature = "v1_10", deprecated = "Since 1.10")]
    //#[doc(alias = "clutter_container_remove")]
    //fn remove<P: IsA<Actor>>(&self, first_actor: &P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    /// Removes `actor` from `self`. The actor should be unparented, so
    /// if you want to keep it around you must hold a reference to it
    /// yourself, using `g_object_ref()`. When the actor has been removed,
    /// the "actor-removed" signal is emitted by `self`.
    ///
    /// This function will call `ClutterContainerIface.remove()`, which is a
    /// deprecated virtual function. The default implementation will call
    /// [`ActorExt::remove_child()`][crate::prelude::ActorExt::remove_child()].
    ///
    /// # Deprecated since 1.10
    ///
    /// Use [`ActorExt::remove_child()`][crate::prelude::ActorExt::remove_child()] instead.
    /// ## `actor`
    /// a [`Actor`][crate::Actor]
    #[cfg_attr(feature = "v1_10", deprecated = "Since 1.10")]
    #[doc(alias = "clutter_container_remove_actor")]
    fn remove_actor<P: IsA<Actor>>(&self, actor: &P);

    /// Sorts a container's children using their depth. This function should not
    /// be normally used by applications.
    ///
    /// # Deprecated since 1.10
    ///
    /// The `ClutterContainerIface.sort_depth_order()` virtual
    ///  function should not be used any more; the default implementation in
    ///  [`Container`][crate::Container] does not do anything.
    #[cfg_attr(feature = "v1_10", deprecated = "Since 1.10")]
    #[cfg(any(feature = "v0_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_6")))]
    #[doc(alias = "clutter_container_sort_depth_order")]
    fn sort_depth_order(&self);

    /// The ::actor-added signal is emitted each time an actor
    /// has been added to `container`.
    /// ## `actor`
    /// the new child that has been added to `container`
    #[cfg(any(feature = "v0_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4")))]
    #[doc(alias = "actor-added")]
    fn connect_actor_added<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId;

    /// The ::actor-removed signal is emitted each time an actor
    /// is removed from `container`.
    /// ## `actor`
    /// the child that has been removed from `container`
    #[cfg(any(feature = "v0_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4")))]
    #[doc(alias = "actor-removed")]
    fn connect_actor_removed<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId;

    //#[cfg(any(feature = "v0_8", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8")))]
    //#[doc(alias = "child-notify")]
    //fn connect_child_notify<Unsupported or ignored types>(&self, detail: Option<&str>, f: F) -> SignalHandlerId;
}

impl<O: IsA<Container>> ContainerExt for O {
    //fn add<P: IsA<Actor>>(&self, first_actor: &P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:clutter_container_add() }
    //}

    fn add_actor<P: IsA<Actor>>(&self, actor: &P) {
        unsafe {
            ffi::clutter_container_add_actor(self.as_ref().to_glib_none().0, actor.as_ref().to_glib_none().0);
        }
    }

    //#[cfg(any(feature = "v0_8", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8")))]
    //fn child_get<P: IsA<Actor>>(&self, actor: &P, first_prop: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:clutter_container_child_get() }
    //}

    //#[cfg(any(feature = "v0_8", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8")))]
    //fn child_get_property<P: IsA<Actor>>(&self, child: &P, property: &str, value: /*Ignored*/&mut glib::Value) {
    //    unsafe { TODO: call ffi:clutter_container_child_get_property() }
    //}

    //#[cfg(any(feature = "v1_6", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_6")))]
    //fn child_notify<P: IsA<Actor>>(&self, child: &P, pspec: /*Ignored*/&glib::ParamSpec) {
    //    unsafe { TODO: call ffi:clutter_container_child_notify() }
    //}

    //#[cfg(any(feature = "v0_8", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8")))]
    //fn child_set<P: IsA<Actor>>(&self, actor: &P, first_prop: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:clutter_container_child_set() }
    //}

    //#[cfg(any(feature = "v0_8", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8")))]
    //fn child_set_property<P: IsA<Actor>>(&self, child: &P, property: &str, value: /*Ignored*/&glib::Value) {
    //    unsafe { TODO: call ffi:clutter_container_child_set_property() }
    //}

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    fn create_child_meta<P: IsA<Actor>>(&self, actor: &P) {
        unsafe {
            ffi::clutter_container_create_child_meta(self.as_ref().to_glib_none().0, actor.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    fn destroy_child_meta<P: IsA<Actor>>(&self, actor: &P) {
        unsafe {
            ffi::clutter_container_destroy_child_meta(self.as_ref().to_glib_none().0, actor.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v0_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_6")))]
    fn find_child_by_name(&self, child_name: &str) -> Option<Actor> {
        unsafe {
            from_glib_none(ffi::clutter_container_find_child_by_name(self.as_ref().to_glib_none().0, child_name.to_glib_none().0))
        }
    }

    //#[cfg(any(feature = "v0_8", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8")))]
    //fn child_meta<P: IsA<Actor>>(&self, actor: &P) -> /*Ignored*/Option<ChildMeta> {
    //    unsafe { TODO: call ffi:clutter_container_get_child_meta() }
    //}

    fn children(&self) -> Vec<Actor> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::clutter_container_get_children(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v0_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_6")))]
    fn lower_child<P: IsA<Actor>, Q: IsA<Actor>>(&self, actor: &P, sibling: Option<&Q>) {
        unsafe {
            ffi::clutter_container_lower_child(self.as_ref().to_glib_none().0, actor.as_ref().to_glib_none().0, sibling.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v0_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_6")))]
    fn raise_child<P: IsA<Actor>, Q: IsA<Actor>>(&self, actor: &P, sibling: Option<&Q>) {
        unsafe {
            ffi::clutter_container_raise_child(self.as_ref().to_glib_none().0, actor.as_ref().to_glib_none().0, sibling.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    //fn remove<P: IsA<Actor>>(&self, first_actor: &P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:clutter_container_remove() }
    //}

    fn remove_actor<P: IsA<Actor>>(&self, actor: &P) {
        unsafe {
            ffi::clutter_container_remove_actor(self.as_ref().to_glib_none().0, actor.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v0_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_6")))]
    fn sort_depth_order(&self) {
        unsafe {
            ffi::clutter_container_sort_depth_order(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v0_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4")))]
    fn connect_actor_added<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn actor_added_trampoline<P: IsA<Container>, F: Fn(&P, &Actor) + 'static>(this: *mut ffi::ClutterContainer, actor: *mut ffi::ClutterActor, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Container::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(actor))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"actor-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(actor_added_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v0_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4")))]
    fn connect_actor_removed<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn actor_removed_trampoline<P: IsA<Container>, F: Fn(&P, &Actor) + 'static>(this: *mut ffi::ClutterContainer, actor: *mut ffi::ClutterActor, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Container::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(actor))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"actor-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(actor_removed_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    //#[cfg(any(feature = "v0_8", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8")))]
    //fn connect_child_notify<Unsupported or ignored types>(&self, detail: Option<&str>, f: F) -> SignalHandlerId {
    //    Ignored pspec: GObject.ParamSpec
    //}
}

impl fmt::Display for Container {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Container")
    }
}
