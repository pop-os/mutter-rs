// Generated by gir (https://github.com/gtk-rs/gir @ 45cd7bc)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 8e47c67)
// from mutter-gir-files
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::mem;

glib::wrapper! {
    #[doc(alias = "CoglTexture")]
    pub struct Texture(Interface<ffi::CoglTexture>);

    match fn {
        type_ => || ffi::cogl_texture_get_gtype(),
    }
}

impl Texture {
    #[doc(alias = "cogl_texture_error_quark")]
    pub fn error_quark() -> u32 {
        unsafe {
            ffi::cogl_texture_error_quark()
        }
    }

    //#[cfg_attr(feature = "v1_18", deprecated = "Since 1.18")]
    //#[cfg(any(feature = "v1_0", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_0")))]
    //#[doc(alias = "cogl_texture_new_from_bitmap")]
    //pub fn new_from_bitmap(bitmap: /*Ignored*/&Bitmap, flags: /*Ignored*/TextureFlags, internal_format: PixelFormat) -> Option<Texture> {
    //    unsafe { TODO: call ffi:cogl_texture_new_from_bitmap() }
    //}

    //#[cfg_attr(feature = "v1_18", deprecated = "Since 1.18")]
    //#[cfg(any(feature = "v1_0", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_0")))]
    //#[doc(alias = "cogl_texture_new_from_data")]
    //pub fn new_from_data(width: i32, height: i32, flags: /*Ignored*/TextureFlags, format: PixelFormat, internal_format: PixelFormat, rowstride: i32, data: &[u8]) -> Option<Texture> {
    //    unsafe { TODO: call ffi:cogl_texture_new_from_data() }
    //}

    //#[cfg_attr(feature = "v1_18", deprecated = "Since 1.18")]
    //#[cfg(any(feature = "v1_0", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_0")))]
    //#[doc(alias = "cogl_texture_new_from_file")]
    //pub fn new_from_file(filename: &str, flags: /*Ignored*/TextureFlags, internal_format: PixelFormat, error: /*Ignored*/Option<glib::Error>) -> Option<Texture> {
    //    unsafe { TODO: call ffi:cogl_texture_new_from_file() }
    //}

    //#[cfg_attr(feature = "v1_18", deprecated = "Since 1.18")]
    //#[cfg(any(feature = "v1_0", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_0")))]
    //#[doc(alias = "cogl_texture_new_with_size")]
    //pub fn new_with_size(width: u32, height: u32, flags: /*Ignored*/TextureFlags, internal_format: PixelFormat) -> Option<Texture> {
    //    unsafe { TODO: call ffi:cogl_texture_new_with_size() }
    //}
}

pub const NONE_TEXTURE: Option<&Texture> = None;

/// Trait containing all [`struct@Texture`] methods.
///
/// # Implementors
///
/// [`Texture`][struct@crate::Texture]
pub trait TextureExt: 'static {
    //#[doc(alias = "cogl_texture_allocate")]
    //fn allocate(&self, error: /*Ignored*/Option<glib::Error>) -> bool;

    //#[cfg(any(feature = "v1_18", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    //#[doc(alias = "cogl_texture_get_components")]
    //#[doc(alias = "get_components")]
    //fn components(&self) -> /*Ignored*/TextureComponents;

    //#[cfg(any(feature = "v0_8", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8")))]
    //#[doc(alias = "cogl_texture_get_data")]
    //#[doc(alias = "get_data")]
    //fn data(&self, format: PixelFormat, rowstride: u32, data: &[u8]) -> i32;

    /// Queries the GL handles for a GPU side texture through its [`Texture`][crate::Texture].
    ///
    /// If the texture is spliced the data for the first sub texture will be
    /// queried.
    ///
    /// # Returns
    ///
    /// [`true`] if the handle was successfully retrieved, [`false`]
    ///  if the handle was invalid
    ///
    /// ## `out_gl_handle`
    /// pointer to return location for the
    ///  textures GL handle, or [`None`].
    ///
    /// ## `out_gl_target`
    /// pointer to return location for the
    ///  GL target type, or [`None`].
    #[doc(alias = "cogl_texture_get_gl_texture")]
    #[doc(alias = "get_gl_texture")]
    fn gl_texture(&self) -> Option<(u32, u32)>;

    /// Queries the height of a cogl texture.
    ///
    /// # Returns
    ///
    /// the height of the GPU side texture in pixels
    #[doc(alias = "cogl_texture_get_height")]
    #[doc(alias = "get_height")]
    fn height(&self) -> u32;

    /// Queries the maximum wasted (unused) pixels in one dimension of a GPU side
    /// texture.
    ///
    /// # Returns
    ///
    /// the maximum waste
    #[doc(alias = "cogl_texture_get_max_waste")]
    #[doc(alias = "get_max_waste")]
    fn max_waste(&self) -> i32;

    /// Queries the pre-multiplied alpha status for internally stored red,
    /// green and blue components for the given `self` as set by
    /// [`set_premultiplied()`][Self::set_premultiplied()].
    ///
    /// By default the pre-multipled state is [`true`].
    ///
    /// # Returns
    ///
    /// [`true`] if red, green and blue components are
    ///  internally stored pre-multiplied by the alpha
    ///  value or [`false`] if not.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "cogl_texture_get_premultiplied")]
    #[doc(alias = "get_premultiplied")]
    fn is_premultiplied(&self) -> bool;

    /// Queries the width of a cogl texture.
    ///
    /// # Returns
    ///
    /// the width of the GPU side texture in pixels
    #[doc(alias = "cogl_texture_get_width")]
    #[doc(alias = "get_width")]
    fn width(&self) -> u32;

    #[doc(alias = "cogl_texture_is_get_data_supported")]
    fn is_get_data_supported(&self) -> bool;

    /// Queries if a texture is sliced (stored as multiple GPU side tecture
    /// objects).
    ///
    /// # Returns
    ///
    /// [`true`] if the texture is sliced, [`false`] if the texture
    ///  is stored as a single GPU texture
    #[doc(alias = "cogl_texture_is_sliced")]
    fn is_sliced(&self) -> bool;

    /// Creates a new texture which represents a subregion of another
    /// texture. The GL resources will be shared so that no new texture
    /// data is actually allocated.
    ///
    /// Sub textures have undefined behaviour texture coordinates outside
    /// of the range [0,1] are used.
    ///
    /// The sub texture will keep a reference to the full texture so you do
    /// not need to keep one separately if you only want to use the sub
    /// texture.
    ///
    /// # Deprecated since 1.18
    ///
    /// Use `cogl_sub_texture_new()`
    /// ## `sub_x`
    /// X coordinate of the top-left of the subregion
    /// ## `sub_y`
    /// Y coordinate of the top-left of the subregion
    /// ## `sub_width`
    /// Width in pixels of the subregion
    /// ## `sub_height`
    /// Height in pixels of the subregion
    ///
    /// # Returns
    ///
    /// A newly created [`Texture`][crate::Texture] or
    ///  [`None`] on failure
    #[cfg_attr(feature = "v1_18", deprecated = "Since 1.18")]
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "cogl_texture_new_from_sub_texture")]
    fn new_from_sub_texture(&self, sub_x: i32, sub_y: i32, sub_width: i32, sub_height: i32) -> Option<Texture>;

    //#[cfg(any(feature = "v1_18", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    //#[doc(alias = "cogl_texture_set_components")]
    //fn set_components(&self, components: /*Ignored*/TextureComponents);

    //#[cfg(any(feature = "v0_8", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8")))]
    //#[doc(alias = "cogl_texture_set_data")]
    //fn set_data(&self, format: PixelFormat, rowstride: i32, data: &[u8], level: i32, error: /*Ignored*/Option<glib::Error>) -> bool;

    /// Affects the internal storage format for this texture by specifying
    /// whether red, green and blue color components should be stored as
    /// pre-multiplied alpha values.
    ///
    /// This api affects how data is uploaded to the GPU since Cogl will
    /// convert source data to have premultiplied or unpremultiplied
    /// components according to this state.
    ///
    /// For example if you create a texture via
    /// `cogl_texture_2d_new_with_size()` and then upload data via
    /// `cogl_texture_set_data()` passing a source format of
    /// [`PixelFormat::RGBA_8888`][crate::PixelFormat::RGBA_8888] then Cogl will internally multiply the
    /// red, green and blue components of the source data by the alpha
    /// component, for each pixel so that the internally stored data has
    /// pre-multiplied alpha components. If you instead upload data that
    /// already has pre-multiplied components by passing
    /// [`PixelFormat::RGBA_8888_PRE`][crate::PixelFormat::RGBA_8888_PRE] as the source format to
    /// `cogl_texture_set_data()` then the data can be uploaded without being
    /// converted.
    ///
    /// By default the `premultipled` state is [`true`].
    /// ## `premultiplied`
    /// Whether any internally stored red, green or blue
    ///  components are pre-multiplied by an alpha
    ///  component.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "cogl_texture_set_premultiplied")]
    fn set_premultiplied(&self, premultiplied: bool);

    //#[cfg(any(feature = "v0_8", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8")))]
    //#[doc(alias = "cogl_texture_set_region")]
    //fn set_region(&self, src_x: i32, src_y: i32, dst_x: i32, dst_y: i32, dst_width: u32, dst_height: u32, width: i32, height: i32, format: PixelFormat, rowstride: u32, data: &[u8]) -> bool;

    //#[cfg(any(feature = "v1_8", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_8")))]
    //#[doc(alias = "cogl_texture_set_region_from_bitmap")]
    //fn set_region_from_bitmap(&self, src_x: i32, src_y: i32, dst_x: i32, dst_y: i32, dst_width: u32, dst_height: u32, bitmap: /*Ignored*/&Bitmap) -> bool;
}

impl<O: IsA<Texture>> TextureExt for O {
    //fn allocate(&self, error: /*Ignored*/Option<glib::Error>) -> bool {
    //    unsafe { TODO: call ffi:cogl_texture_allocate() }
    //}

    //#[cfg(any(feature = "v1_18", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    //fn components(&self) -> /*Ignored*/TextureComponents {
    //    unsafe { TODO: call ffi:cogl_texture_get_components() }
    //}

    //#[cfg(any(feature = "v0_8", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8")))]
    //fn data(&self, format: PixelFormat, rowstride: u32, data: &[u8]) -> i32 {
    //    unsafe { TODO: call ffi:cogl_texture_get_data() }
    //}

    fn gl_texture(&self) -> Option<(u32, u32)> {
        unsafe {
            let mut out_gl_handle = mem::MaybeUninit::uninit();
            let mut out_gl_target = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::cogl_texture_get_gl_texture(self.as_ref().to_glib_none().0, out_gl_handle.as_mut_ptr(), out_gl_target.as_mut_ptr()));
            let out_gl_handle = out_gl_handle.assume_init();
            let out_gl_target = out_gl_target.assume_init();
            if ret { Some((out_gl_handle, out_gl_target)) } else { None }
        }
    }

    fn height(&self) -> u32 {
        unsafe {
            ffi::cogl_texture_get_height(self.as_ref().to_glib_none().0)
        }
    }

    fn max_waste(&self) -> i32 {
        unsafe {
            ffi::cogl_texture_get_max_waste(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn is_premultiplied(&self) -> bool {
        unsafe {
            from_glib(ffi::cogl_texture_get_premultiplied(self.as_ref().to_glib_none().0))
        }
    }

    fn width(&self) -> u32 {
        unsafe {
            ffi::cogl_texture_get_width(self.as_ref().to_glib_none().0)
        }
    }

    fn is_get_data_supported(&self) -> bool {
        unsafe {
            from_glib(ffi::cogl_texture_is_get_data_supported(self.as_ref().to_glib_none().0))
        }
    }

    fn is_sliced(&self) -> bool {
        unsafe {
            from_glib(ffi::cogl_texture_is_sliced(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    fn new_from_sub_texture(&self, sub_x: i32, sub_y: i32, sub_width: i32, sub_height: i32) -> Option<Texture> {
        unsafe {
            from_glib_full(ffi::cogl_texture_new_from_sub_texture(self.as_ref().to_glib_none().0, sub_x, sub_y, sub_width, sub_height))
        }
    }

    //#[cfg(any(feature = "v1_18", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    //fn set_components(&self, components: /*Ignored*/TextureComponents) {
    //    unsafe { TODO: call ffi:cogl_texture_set_components() }
    //}

    //#[cfg(any(feature = "v0_8", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8")))]
    //fn set_data(&self, format: PixelFormat, rowstride: i32, data: &[u8], level: i32, error: /*Ignored*/Option<glib::Error>) -> bool {
    //    unsafe { TODO: call ffi:cogl_texture_set_data() }
    //}

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn set_premultiplied(&self, premultiplied: bool) {
        unsafe {
            ffi::cogl_texture_set_premultiplied(self.as_ref().to_glib_none().0, premultiplied.into_glib());
        }
    }

    //#[cfg(any(feature = "v0_8", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8")))]
    //fn set_region(&self, src_x: i32, src_y: i32, dst_x: i32, dst_y: i32, dst_width: u32, dst_height: u32, width: i32, height: i32, format: PixelFormat, rowstride: u32, data: &[u8]) -> bool {
    //    unsafe { TODO: call ffi:cogl_texture_set_region() }
    //}

    //#[cfg(any(feature = "v1_8", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_8")))]
    //fn set_region_from_bitmap(&self, src_x: i32, src_y: i32, dst_x: i32, dst_y: i32, dst_width: u32, dst_height: u32, bitmap: /*Ignored*/&Bitmap) -> bool {
    //    unsafe { TODO: call ffi:cogl_texture_set_region_from_bitmap() }
    //}
}

impl fmt::Display for Texture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Texture")
    }
}
