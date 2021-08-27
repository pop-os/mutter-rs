<!-- file * -->
<!-- fn blit_framebuffer -->
`return_` FALSE for an immediately detected error, TRUE otherwise.

This blits a region of the color buffer of the source buffer
to the destination buffer. This function should only be
called if the COGL_FEATURE_ID_BLIT_FRAMEBUFFER feature is
advertised.

The source and destination rectangles are defined in offscreen
framebuffer orientation. When copying between an offscreen and
onscreen framebuffers, the image is y-flipped accordingly.

The two buffers must have the same value types (e.g. floating-point,
unsigned int, signed int, or fixed-point), but color formats do not
need to match. This limitation comes from OpenGL ES 3.0 definition
of glBlitFramebuffer.

Note that this function differs a lot from the glBlitFramebuffer
function provided by the GL_EXT_framebuffer_blit extension. Notably
it doesn't support having different sizes for the source and
destination rectangle. This doesn't seem
like a particularly useful feature. If the application wanted to
scale the results it may make more sense to draw a primitive
instead.

The GL function is documented to be affected by the scissor. This
function therefore ensure that an empty clip stack is flushed
before performing the blit which means the scissor is effectively
ignored.

The function also doesn't support specifying the buffers to copy
and instead only the color buffer is copied. When copying the depth
or stencil buffers the extension on GLES2.0 only supports copying
the full buffer which would be awkward to document with this
API. If we wanted to support that feature it may be better to have
a separate function to copy the entire buffer for a given mask.

The `c` error argument is optional, it can be NULL. If it is not NULL
and this function returns FALSE, an error object with a code from
COGL_SYSTEM_ERROR will be created.
## `framebuffer`
The source `CoglFramebuffer`
## `dst`
The destination `CoglFramebuffer`
## `src_x`
Source x position
## `src_y`
Source y position
## `dst_x`
Destination x position
## `dst_y`
Destination y position
## `width`
Width of region to copy
## `height`
Height of region to copy
<!-- fn create_program -->
Create a new cogl program object that can be used to replace parts of the GL
rendering pipeline with custom code.

# Deprecated since 1.16

Use `CoglSnippet` api

# Returns

a new cogl program.
<!-- fn create_shader -->
Create a new shader handle, use `cogl_shader_source()` to set the
source code to be used on it.

# Deprecated since 1.16

Use `CoglSnippet` api
## `shader_type`
COGL_SHADER_TYPE_VERTEX or COGL_SHADER_TYPE_FRAGMENT.

# Returns

a new shader handle.
<!-- fn debug_object_foreach_type -->
Invokes `func` once for each type of object that Cogl uses and
passes a count of the number of objects for that type. This is
intended to be used solely for debugging purposes to track down
issues with objects leaking.
## `func`
A callback function for each type
<!-- fn foreach_feature -->
Iterates through all the context level features currently supported
for a given `context` and for each feature `callback` is called.
## `context`
A `CoglContext` pointer
## `callback`
A `CoglFeatureCallback` called for each
 supported feature
<!-- fn graphics_reset_status -->
Returns the graphics reset status as reported by
GetGraphicsResetStatusARB defined in the ARB_robustness extension.

Note that Cogl doesn't normally enable the ARB_robustness
extension in which case this will only ever return
`COGL_GRAPHICS_RESET_STATUS_NO_ERROR`.

Applications must explicitly use a backend specific method to
request that errors get reported such as X11's
`cogl_xlib_renderer_request_reset_on_video_memory_purge()`.
## `context`
a `CoglContext` pointer

# Returns

a `CoglGraphicsResetStatus`
<!-- fn option_group -->
Retrieves the `GOptionGroup` used by Cogl to parse the command
line options. Clutter uses this to handle the Cogl command line
options during its initialization process.

# Deprecated since 1.16

Not replaced

# Returns

a `GOptionGroup`
<!-- fn proc_address -->
Gets a pointer to a given GL or GL ES extension function. This acts
as a wrapper around glXGetProcAddress() or whatever is the
appropriate function for the current backend.

`<note>`This function should not be used to query core opengl API
symbols since eglGetProcAddress for example doesn't allow this and
and may return a junk pointer if you do.`</note>`
## `name`
the name of the function.

# Returns

a pointer to the requested function or [`None`] if the
 function is not available.
<!-- fn has_feature -->
Checks if a given `feature` is currently available

Cogl does not aim to be a lowest common denominator API, it aims to
expose all the interesting features of GPUs to application which
means applications have some responsibility to explicitly check
that certain features are available before depending on them.
## `context`
A `CoglContext` pointer
## `feature`
A `CoglFeatureID`

# Returns

[`true`] if the `feature` is currently supported or [`false`] if
not.
<!-- fn has_features -->
Checks if a list of features are all currently available.

This checks all of the listed features using `cogl_has_feature()` and
returns [`true`] if all the features are available or [`false`]
otherwise.
## `context`
A `CoglContext` pointer

# Returns

[`true`] if all the features are available, [`false`]
otherwise.
<!-- fn is_bitmap -->
Checks whether `object` is a `CoglBitmap`
## `object`
a `CoglObject` pointer

# Returns

[`true`] if the passed `object` represents a bitmap,
 and [`false`] otherwise
<!-- fn is_context -->
Gets whether the given object references an existing context object.
## `object`
An object or [`None`]

# Returns

[`true`] if the `object` references a `CoglContext`,
 [`false`] otherwise
<!-- fn is_frame_info -->
Gets whether the given object references a `CoglFrameInfo`.
## `object`
A `CoglObject` pointer

# Returns

[`true`] if the object references a `CoglFrameInfo`
 and [`false`] otherwise.
<!-- fn is_framebuffer -->
Gets whether the given object references a `CoglFramebuffer`.
## `object`
A `CoglObject` pointer

# Returns

[`true`] if the object references a `CoglFramebuffer`
 and [`false`] otherwise.
<!-- fn is_pipeline -->
Gets whether the given `object` references an existing pipeline object.
## `object`
A `CoglObject`

# Returns

[`true`] if the `object` references a `CoglPipeline`,
 [`false`] otherwise
<!-- fn is_program -->
Gets whether the given handle references an existing program object.

# Deprecated since 1.16

Use `CoglSnippet` api
## `handle`
A CoglHandle

# Returns

[`true`] if the handle references a program,
 [`false`] otherwise
<!-- fn is_shader -->
Gets whether the given handle references an existing shader object.

# Deprecated since 1.16

Use `CoglSnippet` api
## `handle`
A CoglHandle

# Returns

[`true`] if the handle references a shader,
 [`false`] otherwise
<!-- fn is_texture -->
Gets whether the given object references a texture object.
## `object`
A `CoglObject` pointer

# Returns

[`true`] if the `object` references a texture, and
 [`false`] otherwise
<!-- fn is_texture_2d -->
Gets whether the given object references an existing `CoglTexture2D`
object.
## `object`
A `CoglObject`

# Returns

[`true`] if the object references a `CoglTexture2D`,
 [`false`] otherwise
<!-- fn is_texture_2d_sliced -->
Gets whether the given object references a `CoglTexture2DSliced`.
## `object`
A `CoglObject` pointer

# Returns

[`true`] if the object references a `CoglTexture2DSliced`
 and [`false`] otherwise.
<!-- fn program_attach_shader -->
Attaches a shader to a program object. A program can have multiple
vertex or fragment shaders but only one of them may provide a
`main()` function. It is allowed to use a program with only a vertex
shader or only a fragment shader.

# Deprecated since 1.16

Use `CoglSnippet` api
## `program_handle`
a `CoglHandle` for a shdaer program.
## `shader_handle`
a `CoglHandle` for a vertex of fragment shader.
<!-- fn program_get_uniform_location -->
Retrieve the location (offset) of a uniform variable in a shader program,
a uniform is a variable that is constant for all vertices/fragments for a
shader object and is possible to modify as an external parameter.

# Deprecated since 1.16

Use `CoglSnippet` api instead
## `handle`
a `CoglHandle` for a shader program.
## `uniform_name`
the name of a uniform.

# Returns

the offset of a uniform in a specified program.
<!-- fn program_link -->
Links a program making it ready for use. Note that calling this
function is optional. If it is not called the program will
automatically be linked the first time it is used.

# Deprecated since 1.16

Use `CoglSnippet` api
## `handle`
a `CoglHandle` for a shader program.
<!-- fn program_set_uniform_1f -->
Changes the value of a floating point uniform for the given linked
`program`.

# Deprecated since 1.16

Use `CoglSnippet` api instead
## `program`
A `CoglHandle` for a linked program
## `uniform_location`
the uniform location retrieved from
 `cogl_program_get_uniform_location()`.
## `value`
the new value of the uniform.
<!-- fn program_set_uniform_1i -->
Changes the value of an integer uniform for the given linked
`program`.

# Deprecated since 1.16

Use `CoglSnippet` api instead
## `program`
A `CoglHandle` for a linked program
## `uniform_location`
the uniform location retrieved from
 `cogl_program_get_uniform_location()`.
## `value`
the new value of the uniform.
<!-- fn program_set_uniform_float -->
Changes the value of a float vector uniform, or uniform array for
the given linked `program`.

# Deprecated since 1.16

Use `CoglSnippet` api instead
## `program`
A `CoglHandle` for a linked program
## `uniform_location`
the uniform location retrieved from
 `cogl_program_get_uniform_location()`.
## `n_components`
The number of components for the uniform. For
example with glsl you'd use 3 for a vec3 or 4 for a vec4.
## `value`
the new value of the uniform[s].
<!-- fn program_set_uniform_int -->
Changes the value of a int vector uniform, or uniform array for
the given linked `program`.

# Deprecated since 1.16

Use `CoglSnippet` api instead
## `program`
A `CoglHandle` for a linked program
## `uniform_location`
the uniform location retrieved from
 `cogl_program_get_uniform_location()`.
## `n_components`
The number of components for the uniform. For
example with glsl you'd use 3 for a vec3 or 4 for a vec4.
## `value`
the new value of the uniform[s].
<!-- fn program_set_uniform_matrix -->
Changes the value of a matrix uniform, or uniform array in the
given linked `program`.

# Deprecated since 1.16

Use `CoglSnippet` api instead
## `program`
A `CoglHandle` for a linked program
## `uniform_location`
the uniform location retrieved from
 `cogl_program_get_uniform_location()`.
## `dimensions`
The dimensions of the matrix. So for for example pass
 2 for a 2x2 matrix or 3 for 3x3.
## `transpose`
Whether to transpose the matrix when setting the uniform.
## `value`
the new value of the uniform.
<!-- fn shader_get_type -->
Retrieves the type of a shader `CoglHandle`

# Deprecated since 1.16

Use `CoglSnippet` api
## `handle`
`CoglHandle` for a shader.

# Returns

`COGL_SHADER_TYPE_VERTEX` if the shader is a vertex processor
 or `COGL_SHADER_TYPE_FRAGMENT` if the shader is a frament processor
<!-- fn shader_source -->
Replaces the current source associated with a shader with a new
one.

Please see <link
linkend="cogl-Shaders-and-Programmable-Pipeline.description">above`</link>`
for a description of the recommended format for the shader code.

# Deprecated since 1.16

Use `CoglSnippet` api
## `shader`
`CoglHandle` for a shader.
## `source`
Shader source.
<!-- const PIXEL_FORMAT_MAX_PLANES -->
The maximum number of planes of a pixel format (see also
`cogl_pixel_format_get_planes()`).
<!-- struct PixelFormat -->
Pixel formats used by Cogl. For the formats with a byte per
component, the order of the components specify the order in
increasing memory addresses. So for example
[`RGB_888`][Self::RGB_888] would have the red component in the
lowest address, green in the next address and blue after that
regardless of the endianness of the system.

For the formats with non byte aligned components the component
order specifies the order within a 16-bit or 32-bit number from
most significant bit to least significant. So for
[`RGB_565`][Self::RGB_565], the red component would be in bits
11-15, the green component would be in 6-11 and the blue component
would be in 1-5. Therefore the order in memory depends on the
endianness of the system.

When uploading a texture [`ANY`][Self::ANY] can be used as the
internal format. Cogl will try to pick the best format to use
internally and convert the texture data if necessary.
<!-- struct PixelFormat::const ANY -->
Any format
<!-- struct PixelFormat::const A_8 -->
8 bits alpha mask
<!-- struct PixelFormat::const RGB_565 -->
RGB, 16 bits
<!-- struct PixelFormat::const RGBA_4444 -->
RGBA, 16 bits
<!-- struct PixelFormat::const RGBA_5551 -->
RGBA, 16 bits
<!-- struct PixelFormat::const YUV -->
Not currently supported
<!-- struct PixelFormat::const G_8 -->
Single luminance component
<!-- struct PixelFormat::const RG_88 -->
RG, 16 bits. Note that red-green textures
 are only available if `COGL_FEATURE_ID_TEXTURE_RG` is advertised.
 See `cogl_texture_set_components()` for details.
<!-- struct PixelFormat::const RGB_888 -->
RGB, 24 bits
<!-- struct PixelFormat::const BGR_888 -->
BGR, 24 bits
<!-- struct PixelFormat::const RGBA_8888 -->
RGBA, 32 bits
<!-- struct PixelFormat::const BGRA_8888 -->
BGRA, 32 bits
<!-- struct PixelFormat::const ARGB_8888 -->
ARGB, 32 bits
<!-- struct PixelFormat::const ABGR_8888 -->
ABGR, 32 bits
<!-- struct PixelFormat::const RGBA_1010102 -->
RGBA, 32 bits, 10 bpc
<!-- struct PixelFormat::const BGRA_1010102 -->
BGRA, 32 bits, 10 bpc
<!-- struct PixelFormat::const ARGB_2101010 -->
ARGB, 32 bits, 10 bpc
<!-- struct PixelFormat::const ABGR_2101010 -->
ABGR, 32 bits, 10 bpc
<!-- struct PixelFormat::const RGBA_FP_16161616 -->
RGBA half floating point, 64 bit
<!-- struct PixelFormat::const BGRA_FP_16161616 -->
BGRA half floating point, 64 bit
<!-- struct PixelFormat::const ARGB_FP_16161616 -->
ARGB half floating point, 64 bit
<!-- struct PixelFormat::const ABGR_FP_16161616 -->
ABGR half floating point, 64 bit
<!-- struct PixelFormat::const RGBA_8888_PRE -->
Premultiplied RGBA, 32 bits
<!-- struct PixelFormat::const BGRA_8888_PRE -->
Premultiplied BGRA, 32 bits
<!-- struct PixelFormat::const ARGB_8888_PRE -->
Premultiplied ARGB, 32 bits
<!-- struct PixelFormat::const ABGR_8888_PRE -->
Premultiplied ABGR, 32 bits
<!-- struct PixelFormat::const RGBA_4444_PRE -->
Premultiplied RGBA, 16 bits
<!-- struct PixelFormat::const RGBA_5551_PRE -->
Premultiplied RGBA, 16 bits
<!-- struct PixelFormat::const RGBA_1010102_PRE -->
Premultiplied RGBA, 32 bits, 10 bpc
<!-- struct PixelFormat::const BGRA_1010102_PRE -->
Premultiplied BGRA, 32 bits, 10 bpc
<!-- struct PixelFormat::const ARGB_2101010_PRE -->
Premultiplied ARGB, 32 bits, 10 bpc
<!-- struct PixelFormat::const ABGR_2101010_PRE -->
Premultiplied ABGR, 32 bits, 10 bpc
<!-- struct PixelFormat::const RGBA_FP_16161616_PRE -->
Premultiplied RGBA half floating point, 64 bit
<!-- struct PixelFormat::const BGRA_FP_16161616_PRE -->
Premultiplied BGRA half floating point, 64 bit
<!-- struct PixelFormat::const ARGB_FP_16161616_PRE -->
Premultiplied ARGB half floating point, 64 bit
<!-- struct PixelFormat::const ABGR_FP_16161616_PRE -->
Premultiplied ABGR half floating point, 64 bit
<!-- struct Texture -->


# Implements

[`TextureExt`][trait@crate::prelude::TextureExt]
<!-- impl Texture::fn new_from_bitmap -->
Creates a [`Texture`][crate::Texture] from a `CoglBitmap`.

# Deprecated since 1.18

Use specific constructors such as
 `cogl_texture_2d_new_from_bitmap()`
## `bitmap`
A `CoglBitmap` pointer
## `flags`
Optional flags for the texture, or `COGL_TEXTURE_NONE`
## `internal_format`
the [`PixelFormat`][crate::PixelFormat] to use for the GPU storage of the
texture

# Returns

A newly created [`Texture`][crate::Texture] or
 [`None`] on failure
<!-- impl Texture::fn new_from_data -->
Creates a new [`Texture`][crate::Texture] based on data residing in memory.

# Deprecated since 1.18

Use specific constructors such as
 `cogl_texture_2d_new_from_data()`
## `width`
width of texture in pixels
## `height`
height of texture in pixels
## `flags`
Optional flags for the texture, or `COGL_TEXTURE_NONE`
## `format`
the [`PixelFormat`][crate::PixelFormat] the buffer is stored in in RAM
## `internal_format`
the [`PixelFormat`][crate::PixelFormat] that will be used for storing
 the buffer on the GPU. If COGL_PIXEL_FORMAT_ANY is given then a
 premultiplied format similar to the format of the source data will
 be used. The default blending equations of Cogl expect premultiplied
 color data; the main use of passing a non-premultiplied format here
 is if you have non-premultiplied source data and are going to adjust
 the blend mode (see `cogl_material_set_blend()`) or use the data for
 something other than straight blending.
## `rowstride`
the memory offset in bytes between the starts of
 scanlines in `data`
## `data`
pointer the memory region where the source buffer resides

# Returns

A newly created [`Texture`][crate::Texture] or
 [`None`] on failure
<!-- impl Texture::fn new_from_file -->
Creates a [`Texture`][crate::Texture] from an image file.

# Deprecated since 1.18

Use specific constructors such as
 `cogl_texture_2d_new_from_file()`
## `filename`
the file to load
## `flags`
Optional flags for the texture, or `COGL_TEXTURE_NONE`
## `internal_format`
the [`PixelFormat`][crate::PixelFormat] to use for the GPU storage of the
 texture. If [`PixelFormat::ANY`][crate::PixelFormat::ANY] is given then a premultiplied
 format similar to the format of the source data will be used. The
 default blending equations of Cogl expect premultiplied color data;
 the main use of passing a non-premultiplied format here is if you
 have non-premultiplied source data and are going to adjust the blend
 mode (see `cogl_material_set_blend()`) or use the data for something
 other than straight blending.

# Returns

A newly created [`Texture`][crate::Texture] or
 [`None`] on failure
<!-- impl Texture::fn new_with_size -->
Creates a new [`Texture`][crate::Texture] with the specified dimensions and pixel format.

# Deprecated since 1.18

Use specific constructors such as
 `cogl_texture_2d_new_with_size()`
## `width`
width of texture in pixels.
## `height`
height of texture in pixels.
## `flags`
Optional flags for the texture, or `COGL_TEXTURE_NONE`
## `internal_format`
the [`PixelFormat`][crate::PixelFormat] to use for the GPU storage of the
 texture.

# Returns

A newly created [`Texture`][crate::Texture] or [`None`] on failure
<!-- trait TextureExt::fn allocate -->
Explicitly allocates the storage for the given `self` which
allows you to be sure that there is enough memory for the
texture and if not then the error can be handled gracefully.

`<note>`Normally applications don't need to use this api directly
since the texture will be implicitly allocated when data is set on
the texture, or if the texture is attached to a `CoglOffscreen`
framebuffer and rendered too.`</note>`

# Returns

[`true`] if the texture was successfully allocated,
 otherwise [`false`] and `error` will be updated if it
 wasn't [`None`].
<!-- trait TextureExt::fn components -->
Queries what components the given `self` stores internally as set
via `cogl_texture_set_components()`.

For textures created by the ‘_with_size’ constructors the default
is `COGL_TEXTURE_COMPONENTS_RGBA`. The other constructors which take
a `CoglBitmap` or a data pointer default to the same components as
the pixel format of the data.
<!-- trait TextureExt::fn data -->
Copies the pixel data from a cogl texture to system memory.

`<note>`Don't pass the value of `cogl_texture_get_rowstride()` as the
`rowstride` argument, the rowstride should be the rowstride you
want for the destination `data` buffer not the rowstride of the
source texture`</note>`
## `format`
the [`PixelFormat`][crate::PixelFormat] to store the texture as.
## `rowstride`
the rowstride of `data` in bytes or pass 0 to calculate
 from the bytes-per-pixel of `format` multiplied by the
 `self` width.
## `data`
memory location to write the `self`'s contents,
or [`None`] to only query the data size through the return value.

# Returns

the size of the texture data in bytes
<!-- trait TextureExt::fn set_components -->
Affects the internal storage format for this texture by specifying
what components will be required for sampling later.

This api affects how data is uploaded to the GPU since unused
components can potentially be discarded from source data.

For textures created by the ‘_with_size’ constructors the default
is `COGL_TEXTURE_COMPONENTS_RGBA`. The other constructors which take
a `CoglBitmap` or a data pointer default to the same components as
the pixel format of the data.

Note that the `COGL_TEXTURE_COMPONENTS_RG` format is not available
on all drivers. The availability can be determined by checking for
the `COGL_FEATURE_ID_TEXTURE_RG` feature. If this format is used on
a driver where it is not available then `COGL_TEXTURE_ERROR_FORMAT`
will be raised when the texture is allocated. Even if the feature
is not available then [`PixelFormat::RG_88`][crate::PixelFormat::RG_88] can still be used as
an image format as long as `COGL_TEXTURE_COMPONENTS_RG` isn't used
as the texture's components.
<!-- trait TextureExt::fn set_data -->
`self` a [`Texture`][crate::Texture].
Sets all the pixels for a given mipmap `level` by copying the pixel
data pointed to by the `data` argument into the given `self`.

`data` should point to the first pixel to copy corresponding
to the top left of the mipmap `level` being set.

If `rowstride` equals 0 then it will be automatically calculated
from the width of the mipmap level and the bytes-per-pixel for the
given `format`.

A mipmap `level` of 0 corresponds to the largest, base image of a
texture and `level` 1 is half the width and height of level 0. If
dividing any dimension of the previous level by two results in a
fraction then round the number down (`floor()`), but clamp to 1
something like this:


```text
 next_width = MAX (1, floor (prev_width));
```

You can determine the number of mipmap levels for a given texture
like this:


```text
 n_levels = 1 + floor (log2 (max_dimension));
```

Where `max_dimension` is the larger of [`width()`][Self::width()] and
[`height()`][Self::height()].

It is an error to pass a `level` number >= the number of levels that
`self` can have according to the above calculation.

`<note>`Since the storage for a [`Texture`][crate::Texture] is allocated lazily then
if the given `self` has not previously been allocated then this
api can return [`false`] and throw an exceptional `error` if there is
not enough memory to allocate storage for `self`.`</note>`
## `format`
the [`PixelFormat`][crate::PixelFormat] used in the source `data` buffer.
## `rowstride`
rowstride of the source `data` buffer (computed from
 the texture width and `format` if it equals 0)
## `data`
the source data, pointing to the first top-left pixel to set
## `level`
The mipmap level to update (Normally 0 for the largest,
 base texture)

# Returns

[`true`] if the data upload was successful, and
 [`false`] otherwise
<!-- trait TextureExt::fn set_region -->
Sets the pixels in a rectangular subregion of `self` from an in-memory
buffer containing pixel data.

`<note>`The region set can't be larger than the source `data``</note>`
## `src_x`
upper left coordinate to use from source data.
## `src_y`
upper left coordinate to use from source data.
## `dst_x`
upper left destination horizontal coordinate.
## `dst_y`
upper left destination vertical coordinate.
## `dst_width`
width of destination region to write. (Must be less
 than or equal to `width`)
## `dst_height`
height of destination region to write. (Must be less
 than or equal to `height`)
## `width`
width of source data buffer.
## `height`
height of source data buffer.
## `format`
the [`PixelFormat`][crate::PixelFormat] used in the source buffer.
## `rowstride`
rowstride of source buffer (computed from width if none
specified)
## `data`
the actual pixel data.

# Returns

[`true`] if the subregion upload was successful, and
 [`false`] otherwise
<!-- trait TextureExt::fn set_region_from_bitmap -->
Copies a specified source region from `bitmap` to the position
(`src_x`, `src_y`) of the given destination texture `handle`.

`<note>`The region updated can't be larger than the source
bitmap`</note>`
## `src_x`
upper left coordinate to use from the source bitmap.
## `src_y`
upper left coordinate to use from the source bitmap
## `dst_x`
upper left destination horizontal coordinate.
## `dst_y`
upper left destination vertical coordinate.
## `dst_width`
width of destination region to write. (Must be less
 than or equal to the bitmap width)
## `dst_height`
height of destination region to write. (Must be less
 than or equal to the bitmap height)
## `bitmap`
The source bitmap to read from

# Returns

[`true`] if the subregion upload was successful, and
 [`false`] otherwise
