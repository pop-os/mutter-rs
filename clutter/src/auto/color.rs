// Generated by gir (https://github.com/gtk-rs/gir @ 5bbf6cb)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 8e47c67)
// from mutter-gir-files
// DO NOT EDIT

#[cfg(any(feature = "v1_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_6")))]
use crate::StaticColor;
use glib::translate::*;
#[cfg(any(feature = "v0_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_2")))]
use std::fmt;
use std::mem;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Color(Boxed<ffi::ClutterColor>);

    match fn {
        copy => |ptr| ffi::clutter_color_copy(ptr),
        free => |ptr| ffi::clutter_color_free(ptr),
        type_ => || ffi::clutter_color_get_type(),
    }
}

impl Color {
    /// Allocates a new, transparent black [`Color`][crate::Color].
    ///
    /// # Returns
    ///
    /// the newly allocated [`Color`][crate::Color]; use
    ///  `clutter_color_free()` to free its resources
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "clutter_color_alloc")]
    pub fn alloc() -> Color {
        unsafe {
            from_glib_full(ffi::clutter_color_alloc())
        }
    }

    /// Creates a new [`Color`][crate::Color] with the given values.
    ///
    /// This function is the equivalent of:
    ///
    ///
    /// ```text
    ///   clutter_color_init (clutter_color_alloc (), red, green, blue, alpha);
    /// ```
    /// ## `red`
    /// red component of the color, between 0 and 255
    /// ## `green`
    /// green component of the color, between 0 and 255
    /// ## `blue`
    /// blue component of the color, between 0 and 255
    /// ## `alpha`
    /// alpha component of the color, between 0 and 255
    ///
    /// # Returns
    ///
    /// the newly allocated color.
    ///  Use `clutter_color_free()` when done
    #[cfg(any(feature = "v0_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8")))]
    #[doc(alias = "clutter_color_new")]
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        unsafe {
            from_glib_full(ffi::clutter_color_new(red, green, blue, alpha))
        }
    }

    /// Adds `self` to `b` and saves the resulting color inside `result`.
    ///
    /// The alpha channel of `result` is set as as the maximum value
    /// between the alpha channels of `self` and `b`.
    /// ## `b`
    /// a [`Color`][crate::Color]
    ///
    /// # Returns
    ///
    ///
    /// ## `result`
    /// return location for the result
    #[doc(alias = "clutter_color_add")]
    pub fn add(&self, b: &Color) -> Color {
        unsafe {
            let mut result = Color::uninitialized();
            ffi::clutter_color_add(self.to_glib_none().0, b.to_glib_none().0, result.to_glib_none_mut().0);
            result
        }
    }

    /// Darkens `self` by a fixed amount, and saves the changed color
    /// in `result`.
    ///
    /// # Returns
    ///
    ///
    /// ## `result`
    /// return location for the darker color
    #[doc(alias = "clutter_color_darken")]
    pub fn darken(&self) -> Color {
        unsafe {
            let mut result = Color::uninitialized();
            ffi::clutter_color_darken(self.to_glib_none().0, result.to_glib_none_mut().0);
            result
        }
    }

    /// Initializes `self` with the given values.
    /// ## `red`
    /// red component of the color, between 0 and 255
    /// ## `green`
    /// green component of the color, between 0 and 255
    /// ## `blue`
    /// blue component of the color, between 0 and 255
    /// ## `alpha`
    /// alpha component of the color, between 0 and 255
    ///
    /// # Returns
    ///
    /// the initialized [`Color`][crate::Color]
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "clutter_color_init")]
    pub fn init(&mut self, red: u8, green: u8, blue: u8, alpha: u8) -> Option<Color> {
        unsafe {
            from_glib_none(ffi::clutter_color_init(self.to_glib_none_mut().0, red, green, blue, alpha))
        }
    }

    /// Interpolates between `self` and `final_` [`Color`][crate::Color]<!-- -->s
    /// using `progress`
    /// ## `final_`
    /// the final [`Color`][crate::Color]
    /// ## `progress`
    /// the interpolation progress
    ///
    /// # Returns
    ///
    ///
    /// ## `result`
    /// return location for the interpolation
    #[cfg(any(feature = "v1_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_6")))]
    #[doc(alias = "clutter_color_interpolate")]
    pub fn interpolate(&self, final_: &Color, progress: f64) -> Color {
        unsafe {
            let mut result = Color::uninitialized();
            ffi::clutter_color_interpolate(self.to_glib_none().0, final_.to_glib_none().0, progress, result.to_glib_none_mut().0);
            result
        }
    }

    /// Lightens `self` by a fixed amount, and saves the changed color
    /// in `result`.
    ///
    /// # Returns
    ///
    ///
    /// ## `result`
    /// return location for the lighter color
    #[doc(alias = "clutter_color_lighten")]
    pub fn lighten(&self) -> Color {
        unsafe {
            let mut result = Color::uninitialized();
            ffi::clutter_color_lighten(self.to_glib_none().0, result.to_glib_none_mut().0);
            result
        }
    }

    /// Shades `self` by `factor` and saves the modified color into `result`.
    /// ## `factor`
    /// the shade factor to apply
    ///
    /// # Returns
    ///
    ///
    /// ## `result`
    /// return location for the shaded color
    #[doc(alias = "clutter_color_shade")]
    pub fn shade(&self, factor: f64) -> Color {
        unsafe {
            let mut result = Color::uninitialized();
            ffi::clutter_color_shade(self.to_glib_none().0, factor, result.to_glib_none_mut().0);
            result
        }
    }

    /// Subtracts `b` from `self` and saves the resulting color inside `result`.
    ///
    /// This function assumes that the components of `self` are greater than the
    /// components of `b`; the result is, otherwise, undefined.
    ///
    /// The alpha channel of `result` is set as the minimum value
    /// between the alpha channels of `self` and `b`.
    /// ## `b`
    /// a [`Color`][crate::Color]
    ///
    /// # Returns
    ///
    ///
    /// ## `result`
    /// return location for the result
    #[doc(alias = "clutter_color_subtract")]
    pub fn subtract(&self, b: &Color) -> Color {
        unsafe {
            let mut result = Color::uninitialized();
            ffi::clutter_color_subtract(self.to_glib_none().0, b.to_glib_none().0, result.to_glib_none_mut().0);
            result
        }
    }

    /// Converts `self` to the HLS format.
    ///
    /// The `hue` value is in the 0 .. 360 range. The `luminance` and
    /// `saturation` values are in the 0 .. 1 range.
    ///
    /// # Returns
    ///
    ///
    /// ## `hue`
    /// return location for the hue value or [`None`]
    ///
    /// ## `luminance`
    /// return location for the luminance value or [`None`]
    ///
    /// ## `saturation`
    /// return location for the saturation value or [`None`]
    #[doc(alias = "clutter_color_to_hls")]
    pub fn to_hls(&self) -> (f32, f32, f32) {
        unsafe {
            let mut hue = mem::MaybeUninit::uninit();
            let mut luminance = mem::MaybeUninit::uninit();
            let mut saturation = mem::MaybeUninit::uninit();
            ffi::clutter_color_to_hls(self.to_glib_none().0, hue.as_mut_ptr(), luminance.as_mut_ptr(), saturation.as_mut_ptr());
            let hue = hue.assume_init();
            let luminance = luminance.assume_init();
            let saturation = saturation.assume_init();
            (hue, luminance, saturation)
        }
    }

    /// Converts `self` into a packed 32 bit integer, containing
    /// all the four 8 bit channels used by [`Color`][crate::Color].
    ///
    /// # Returns
    ///
    /// a packed color
    #[doc(alias = "clutter_color_to_pixel")]
    pub fn to_pixel(&self) -> u32 {
        unsafe {
            ffi::clutter_color_to_pixel(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v0_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_2")))]
    #[doc(alias = "clutter_color_to_string")]
    #[doc(alias = "to_string")]
    pub fn to_str(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::clutter_color_to_string(self.to_glib_none().0))
        }
    }

    /// Converts a color expressed in HLS (hue, luminance and saturation)
    /// values into a [`Color`][crate::Color].
    /// ## `hue`
    /// hue value, in the 0 .. 360 range
    /// ## `luminance`
    /// luminance value, in the 0 .. 1 range
    /// ## `saturation`
    /// saturation value, in the 0 .. 1 range
    ///
    /// # Returns
    ///
    ///
    /// ## `color`
    /// return location for a [`Color`][crate::Color]
    #[doc(alias = "clutter_color_from_hls")]
    pub fn from_hls(hue: f32, luminance: f32, saturation: f32) -> Color {
        unsafe {
            let mut color = Color::uninitialized();
            ffi::clutter_color_from_hls(color.to_glib_none_mut().0, hue, luminance, saturation);
            color
        }
    }

    /// Converts `pixel` from the packed representation of a four 8 bit channel
    /// color to a [`Color`][crate::Color].
    /// ## `pixel`
    /// a 32 bit packed integer containing a color
    ///
    /// # Returns
    ///
    ///
    /// ## `color`
    /// return location for a [`Color`][crate::Color]
    #[doc(alias = "clutter_color_from_pixel")]
    pub fn from_pixel(pixel: u32) -> Color {
        unsafe {
            let mut color = Color::uninitialized();
            ffi::clutter_color_from_pixel(color.to_glib_none_mut().0, pixel);
            color
        }
    }

    /// Parses a string definition of a color, filling the `ClutterColor.red`,
    /// `ClutterColor.green`, `ClutterColor.blue` and `ClutterColor.alpha` fields
    /// of `color`.
    ///
    /// The `color` is not allocated.
    ///
    /// The format of `str` can be either one of:
    ///
    ///  - a standard name (as taken from the X11 rgb.txt file)
    ///  - an hexadecimal value in the form: ``rgb``, ``rrggbb``, ``rgba``, or ``rrggbbaa``
    ///  - a RGB color in the form: `rgb(r, g, b)`
    ///  - a RGB color in the form: `rgba(r, g, b, a)`
    ///  - a HSL color in the form: `hsl(h, s, l)`
    ///  -a HSL color in the form: `hsla(h, s, l, a)`
    ///
    /// where 'r', 'g', 'b' and 'a' are (respectively) the red, green, blue color
    /// intensities and the opacity. The 'h', 's' and 'l' are (respectively) the
    /// hue, saturation and luminance values.
    ///
    /// In the `rgb()` and `rgba()` formats, the 'r', 'g', and 'b' values are either
    /// integers between 0 and 255, or percentage values in the range between 0%
    /// and 100%; the percentages require the '%' character. The 'a' value, if
    /// specified, can only be a floating point value between 0.0 and 1.0.
    ///
    /// In the `hls()` and `hlsa()` formats, the 'h' value (hue) is an angle between
    /// 0 and 360.0 degrees; the 'l' and 's' values (luminance and saturation) are
    /// percentage values in the range between 0% and 100%. The 'a' value, if specified,
    /// can only be a floating point value between 0.0 and 1.0.
    ///
    /// Whitespace inside the definitions is ignored; no leading whitespace
    /// is allowed.
    ///
    /// If the alpha component is not specified then it is assumed to be set to
    /// be fully opaque.
    /// ## `str`
    /// a string specifying a color
    ///
    /// # Returns
    ///
    /// [`true`] if parsing succeeded, and [`false`] otherwise
    ///
    /// ## `color`
    /// return location for a [`Color`][crate::Color]
    #[cfg(any(feature = "v1_0", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_0")))]
    #[doc(alias = "clutter_color_from_string")]
    pub fn from_string(str: &str) -> Option<Color> {
        unsafe {
            let mut color = Color::uninitialized();
            let ret = from_glib(ffi::clutter_color_from_string(color.to_glib_none_mut().0, str.to_glib_none().0));
            if ret { Some(color) } else { None }
        }
    }

    /// Retrieves a static color for the given `color` name
    ///
    /// Static colors are created by Clutter and are guaranteed to always be
    /// available and valid
    /// ## `color`
    /// the named global color
    ///
    /// # Returns
    ///
    /// a pointer to a static color; the returned pointer
    ///  is owned by Clutter and it should never be modified or freed
    #[cfg(any(feature = "v1_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_6")))]
    #[doc(alias = "clutter_color_get_static")]
    #[doc(alias = "get_static")]
    pub fn static_(color: StaticColor) -> Option<Color> {
        unsafe {
            from_glib_none(ffi::clutter_color_get_static(color.into_glib()))
        }
    }
}

#[cfg(any(feature = "v0_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_2")))]
impl fmt::Display for Color {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_str())
    }
}
