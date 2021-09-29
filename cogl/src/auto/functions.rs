// Generated by gir (https://github.com/gtk-rs/gir @ 45cd7bc)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 8e47c67)
// from mutter-gir-files
// DO NOT EDIT

#[cfg(any(feature = "v0_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8")))]
use crate::PixelFormat;
use glib::translate::*;


#[doc(alias = "cogl_blend_string_error_quark")]
pub fn blend_string_error_quark() -> u32 {
    unsafe {
        ffi::cogl_blend_string_error_quark()
    }
}

//#[doc(alias = "cogl_blit_framebuffer")]
//pub fn blit_framebuffer(framebuffer: /*Ignored*/&Framebuffer, dst: /*Ignored*/&Framebuffer, src_x: i32, src_y: i32, dst_x: i32, dst_y: i32, width: i32, height: i32, error: /*Ignored*/Option<glib::Error>) -> bool {
//    unsafe { TODO: call ffi:cogl_blit_framebuffer() }
//}

//#[doc(alias = "cogl_clutter_winsys_has_feature_CLUTTER")]
//pub fn clutter_winsys_has_feature_CLUTTER(feature: /*Ignored*/WinsysFeature) -> bool {
//    unsafe { TODO: call ffi:cogl_clutter_winsys_has_feature_CLUTTER() }
//}

//#[cfg_attr(feature = "v1_16", deprecated = "Since 1.16")]
//#[doc(alias = "cogl_create_program")]
//pub fn create_program() -> /*Unimplemented*/Option<Handle> {
//    unsafe { TODO: call ffi:cogl_create_program() }
//}

//#[cfg_attr(feature = "v1_16", deprecated = "Since 1.16")]
//#[cfg(any(feature = "v1_0", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_0")))]
//#[doc(alias = "cogl_create_shader")]
//pub fn create_shader(shader_type: /*Ignored*/ShaderType) -> /*Unimplemented*/Option<Handle> {
//    unsafe { TODO: call ffi:cogl_create_shader() }
//}

//#[cfg(any(feature = "v1_8", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_8")))]
//#[doc(alias = "cogl_debug_object_foreach_type")]
//pub fn debug_object_foreach_type(func: /*Unimplemented*/FnMut(/*Ignored*/DebugObjectTypeInfo), user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
//    unsafe { TODO: call ffi:cogl_debug_object_foreach_type() }
//}

/// Prints a list of all the object types that Cogl uses along with the
/// number of objects of that type that are currently in use. This is
/// intended to be used solely for debugging purposes to track down
/// issues with objects leaking.
#[cfg(any(feature = "v1_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_8")))]
#[doc(alias = "cogl_debug_object_print_instances")]
pub fn debug_object_print_instances() {
    unsafe {
        ffi::cogl_debug_object_print_instances();
    }
}

/// This function should only need to be called in exceptional circumstances.
///
/// As an optimization Cogl drawing functions may batch up primitives
/// internally, so if you are trying to use raw GL outside of Cogl you stand a
/// better chance of being successful if you ask Cogl to flush any batched
/// geometry before making your state changes.
///
/// It only ensure that the underlying driver is issued all the commands
/// necessary to draw the batched primitives. It provides no guarantees about
/// when the driver will complete the rendering.
///
/// This provides no guarantees about the GL state upon returning and to avoid
/// confusing Cogl you should aim to restore any changes you make before
/// resuming use of Cogl.
///
/// If you are making state changes with the intention of affecting Cogl drawing
/// primitives you are 100% on your own since you stand a good chance of
/// conflicting with Cogl internals. For example clutter-gst which currently
/// uses direct GL calls to bind ARBfp programs will very likely break when Cogl
/// starts to use ARBfb programs itself for the material API.
#[cfg(any(feature = "v1_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_0")))]
#[doc(alias = "cogl_flush")]
pub fn flush() {
    unsafe {
        ffi::cogl_flush();
    }
}

//#[cfg(any(feature = "v1_10", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
//#[doc(alias = "cogl_foreach_feature")]
//pub fn foreach_feature(context: /*Ignored*/&Context, callback: /*Unimplemented*/FnMut(/*Ignored*/FeatureID), user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
//    unsafe { TODO: call ffi:cogl_foreach_feature() }
//}

/// Queries if backface culling has been enabled via
/// [`set_backface_culling_enabled()`][crate::set_backface_culling_enabled()]
///
/// # Deprecated since 1.16
///
/// Use `cogl_pipeline_get_cull_face_mode()` instead
///
/// # Returns
///
/// [`true`] if backface culling is enabled, and [`false`] otherwise
#[cfg_attr(feature = "v1_16", deprecated = "Since 1.16")]
#[doc(alias = "cogl_get_backface_culling_enabled")]
#[doc(alias = "get_backface_culling_enabled")]
pub fn is_backface_culling_enabled() -> bool {
    unsafe {
        from_glib(ffi::cogl_get_backface_culling_enabled())
    }
}

/// Queries if depth testing has been enabled via `cogl_set_depth_test_enable()`
///
/// # Deprecated since 1.16
///
/// Use `cogl_pipeline_set_depth_state()` instead
///
/// # Returns
///
/// [`true`] if depth testing is enabled, and [`false`] otherwise
#[cfg_attr(feature = "v1_16", deprecated = "Since 1.16")]
#[doc(alias = "cogl_get_depth_test_enabled")]
#[doc(alias = "get_depth_test_enabled")]
pub fn is_depth_test_enabled() -> bool {
    unsafe {
        from_glib(ffi::cogl_get_depth_test_enabled())
    }
}

//#[doc(alias = "cogl_get_graphics_reset_status")]
//#[doc(alias = "get_graphics_reset_status")]
//pub fn graphics_reset_status(context: /*Ignored*/&Context) -> /*Ignored*/GraphicsResetStatus {
//    unsafe { TODO: call ffi:cogl_get_graphics_reset_status() }
//}

//#[cfg_attr(feature = "v1_16", deprecated = "Since 1.16")]
//#[cfg(any(feature = "v1_0", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_0")))]
//#[doc(alias = "cogl_get_option_group")]
//#[doc(alias = "get_option_group")]
//pub fn option_group() -> /*Ignored*/Option<glib::OptionGroup> {
//    unsafe { TODO: call ffi:cogl_get_option_group() }
//}

//#[doc(alias = "cogl_get_proc_address")]
//#[doc(alias = "get_proc_address")]
//pub fn proc_address(name: &str) -> Option<Box_<dyn Fn() + 'static>> {
//    unsafe { TODO: call ffi:cogl_get_proc_address() }
//}

#[doc(alias = "cogl_handle_get_type")]
pub fn handle_get_type() -> glib::types::Type {
    unsafe {
        from_glib(ffi::cogl_handle_get_type())
    }
}

//#[cfg(any(feature = "v1_10", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
//#[doc(alias = "cogl_has_feature")]
//pub fn has_feature(context: /*Ignored*/&Context, feature: /*Ignored*/FeatureID) -> bool {
//    unsafe { TODO: call ffi:cogl_has_feature() }
//}

//#[cfg(any(feature = "v1_10", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
//#[doc(alias = "cogl_has_features")]
//pub fn has_features(context: /*Ignored*/&Context, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool {
//    unsafe { TODO: call ffi:cogl_has_features() }
//}

//#[cfg(any(feature = "v1_0", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_0")))]
//#[doc(alias = "cogl_is_bitmap")]
//pub fn is_bitmap(object: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool {
//    unsafe { TODO: call ffi:cogl_is_bitmap() }
//}

//#[cfg(any(feature = "v1_10", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
//#[doc(alias = "cogl_is_context")]
//pub fn is_context(object: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool {
//    unsafe { TODO: call ffi:cogl_is_context() }
//}

//#[cfg(any(feature = "v2_0", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_0")))]
//#[doc(alias = "cogl_is_frame_info")]
//pub fn is_frame_info(object: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool {
//    unsafe { TODO: call ffi:cogl_is_frame_info() }
//}

//#[cfg(any(feature = "v1_10", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
//#[doc(alias = "cogl_is_framebuffer")]
//pub fn is_framebuffer(object: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool {
//    unsafe { TODO: call ffi:cogl_is_framebuffer() }
//}

//#[cfg(any(feature = "v2_0", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_0")))]
//#[doc(alias = "cogl_is_pipeline")]
//pub fn is_pipeline(object: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool {
//    unsafe { TODO: call ffi:cogl_is_pipeline() }
//}

//#[cfg_attr(feature = "v1_16", deprecated = "Since 1.16")]
//#[doc(alias = "cogl_is_program")]
//pub fn is_program(handle: /*Unimplemented*/Handle) -> bool {
//    unsafe { TODO: call ffi:cogl_is_program() }
//}

//#[cfg_attr(feature = "v1_16", deprecated = "Since 1.16")]
//#[doc(alias = "cogl_is_shader")]
//pub fn is_shader(handle: /*Unimplemented*/Handle) -> bool {
//    unsafe { TODO: call ffi:cogl_is_shader() }
//}

//#[doc(alias = "cogl_is_texture")]
//pub fn is_texture(object: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool {
//    unsafe { TODO: call ffi:cogl_is_texture() }
//}

//#[doc(alias = "cogl_is_texture_2d")]
//pub fn is_texture_2d(object: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool {
//    unsafe { TODO: call ffi:cogl_is_texture_2d() }
//}

//#[cfg(any(feature = "v1_10", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
//#[doc(alias = "cogl_is_texture_2d_sliced")]
//pub fn is_texture_2d_sliced(object: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool {
//    unsafe { TODO: call ffi:cogl_is_texture_2d_sliced() }
//}

/// Queries the number of bytes per pixel for a given format in the given plane.
/// ## `format`
/// The pixel format
/// ## `plane`
/// The index of the plane (should not be more than the number of planes
///  in the given format).
///
/// # Returns
///
/// The number of bytes per pixel in the given format's given plane.
#[cfg(any(feature = "v0_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8")))]
#[doc(alias = "cogl_pixel_format_get_bytes_per_pixel")]
pub fn pixel_format_get_bytes_per_pixel(format: PixelFormat, plane: i32) -> i32 {
    unsafe {
        ffi::cogl_pixel_format_get_bytes_per_pixel(format.into_glib(), plane)
    }
}

/// Returns the number of planes the given CoglPixelFormat specifies.
/// ## `format`
/// The format for which to get the number of planes
///
/// # Returns
///
/// The no. of planes of `format` (at most `COGL_PIXEL_FORMAT_MAX_PLANES`)
#[cfg(any(feature = "v0_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8")))]
#[doc(alias = "cogl_pixel_format_get_n_planes")]
pub fn pixel_format_get_n_planes(format: PixelFormat) -> i32 {
    unsafe {
        ffi::cogl_pixel_format_get_n_planes(format.into_glib())
    }
}

/// Returns a string representation of `format`, useful for debugging purposes.
/// ## `format`
/// a [`PixelFormat`][crate::PixelFormat]
///
/// # Returns
///
/// A string representation of `format`.
#[cfg(any(feature = "v0_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8")))]
#[doc(alias = "cogl_pixel_format_to_string")]
pub fn pixel_format_to_string(format: PixelFormat) -> Option<glib::GString> {
    unsafe {
        from_glib_none(ffi::cogl_pixel_format_to_string(format.into_glib()))
    }
}

//#[cfg_attr(feature = "v1_16", deprecated = "Since 1.16")]
//#[doc(alias = "cogl_program_attach_shader")]
//pub fn program_attach_shader(program_handle: /*Unimplemented*/Handle, shader_handle: /*Unimplemented*/Handle) {
//    unsafe { TODO: call ffi:cogl_program_attach_shader() }
//}

//#[cfg_attr(feature = "v1_16", deprecated = "Since 1.16")]
//#[doc(alias = "cogl_program_get_uniform_location")]
//pub fn program_get_uniform_location(handle: /*Unimplemented*/Handle, uniform_name: &str) -> i32 {
//    unsafe { TODO: call ffi:cogl_program_get_uniform_location() }
//}

//#[cfg_attr(feature = "v1_16", deprecated = "Since 1.16")]
//#[doc(alias = "cogl_program_link")]
//pub fn program_link(handle: /*Unimplemented*/Handle) {
//    unsafe { TODO: call ffi:cogl_program_link() }
//}

//#[cfg_attr(feature = "v1_16", deprecated = "Since 1.16")]
//#[cfg(any(feature = "v1_4", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_4")))]
//#[doc(alias = "cogl_program_set_uniform_1f")]
//pub fn program_set_uniform_1f(program: /*Unimplemented*/Handle, uniform_location: i32, value: f32) {
//    unsafe { TODO: call ffi:cogl_program_set_uniform_1f() }
//}

//#[cfg_attr(feature = "v1_16", deprecated = "Since 1.16")]
//#[cfg(any(feature = "v1_4", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_4")))]
//#[doc(alias = "cogl_program_set_uniform_1i")]
//pub fn program_set_uniform_1i(program: /*Unimplemented*/Handle, uniform_location: i32, value: i32) {
//    unsafe { TODO: call ffi:cogl_program_set_uniform_1i() }
//}

//#[cfg_attr(feature = "v1_16", deprecated = "Since 1.16")]
//#[cfg(any(feature = "v1_4", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_4")))]
//#[doc(alias = "cogl_program_set_uniform_float")]
//pub fn program_set_uniform_float(program: /*Unimplemented*/Handle, uniform_location: i32, n_components: i32, value: &[f32]) {
//    unsafe { TODO: call ffi:cogl_program_set_uniform_float() }
//}

//#[cfg_attr(feature = "v1_16", deprecated = "Since 1.16")]
//#[cfg(any(feature = "v1_4", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_4")))]
//#[doc(alias = "cogl_program_set_uniform_int")]
//pub fn program_set_uniform_int(program: /*Unimplemented*/Handle, uniform_location: i32, n_components: i32, value: &[i32]) {
//    unsafe { TODO: call ffi:cogl_program_set_uniform_int() }
//}

//#[cfg_attr(feature = "v1_16", deprecated = "Since 1.16")]
//#[cfg(any(feature = "v1_4", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_4")))]
//#[doc(alias = "cogl_program_set_uniform_matrix")]
//pub fn program_set_uniform_matrix(program: /*Unimplemented*/Handle, uniform_location: i32, dimensions: i32, transpose: bool, value: &[f32]) {
//    unsafe { TODO: call ffi:cogl_program_set_uniform_matrix() }
//}

/// Sets whether textures positioned so that their backface is showing
/// should be hidden. This can be used to efficiently draw two-sided
/// textures or fully closed cubes without enabling depth testing. This
/// only affects calls to the cogl_rectangle* family of functions and
/// cogl_vertex_buffer_draw*. Backface culling is disabled by default.
///
/// # Deprecated since 1.16
///
/// Use `cogl_pipeline_set_cull_face_mode()` instead
/// ## `setting`
/// [`true`] to enable backface culling or [`false`] to disable.
#[cfg_attr(feature = "v1_16", deprecated = "Since 1.16")]
#[doc(alias = "cogl_set_backface_culling_enabled")]
pub fn set_backface_culling_enabled(setting: bool) {
    unsafe {
        ffi::cogl_set_backface_culling_enabled(setting.into_glib());
    }
}

/// Sets whether depth testing is enabled. If it is disabled then the
/// order that actors are layered on the screen depends solely on the
/// order specified using `clutter_actor_raise()` and
/// `clutter_actor_lower()`, otherwise it will also take into account the
/// actor's depth. Depth testing is disabled by default.
///
/// # Deprecated since 1.16
///
/// Use `cogl_pipeline_set_depth_state()` instead
/// ## `setting`
/// [`true`] to enable depth testing or [`false`] to disable.
#[cfg_attr(feature = "v1_16", deprecated = "Since 1.16")]
#[doc(alias = "cogl_set_depth_test_enabled")]
pub fn set_depth_test_enabled(setting: bool) {
    unsafe {
        ffi::cogl_set_depth_test_enabled(setting.into_glib());
    }
}

//#[doc(alias = "cogl_set_tracing_disabled_on_thread")]
//pub fn set_tracing_disabled_on_thread(main_context: /*Ignored*/&glib::MainContext) {
//    unsafe { TODO: call ffi:cogl_set_tracing_disabled_on_thread() }
//}

//#[doc(alias = "cogl_set_tracing_enabled_on_thread")]
//pub fn set_tracing_enabled_on_thread(main_context: /*Ignored*/&glib::MainContext, group: &str, filename: &str) {
//    unsafe { TODO: call ffi:cogl_set_tracing_enabled_on_thread() }
//}

//#[doc(alias = "cogl_set_tracing_enabled_on_thread_with_fd")]
//pub fn set_tracing_enabled_on_thread_with_fd(main_context: /*Ignored*/&glib::MainContext, group: &str, fd: i32) {
//    unsafe { TODO: call ffi:cogl_set_tracing_enabled_on_thread_with_fd() }
//}

//#[cfg_attr(feature = "v1_16", deprecated = "Since 1.16")]
//#[cfg(any(feature = "v1_0", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_0")))]
//#[doc(alias = "cogl_shader_get_type")]
//pub fn shader_get_type(handle: /*Unimplemented*/Handle) -> /*Ignored*/ShaderType {
//    unsafe { TODO: call ffi:cogl_shader_get_type() }
//}

//#[cfg_attr(feature = "v1_16", deprecated = "Since 1.16")]
//#[doc(alias = "cogl_shader_source")]
//pub fn shader_source(shader: /*Unimplemented*/Handle, source: &str) {
//    unsafe { TODO: call ffi:cogl_shader_source() }
//}

//#[doc(alias = "cogl_trace_end")]
//pub fn trace_end(head: /*Ignored*/&mut TraceHead) {
//    unsafe { TODO: call ffi:cogl_trace_end() }
//}