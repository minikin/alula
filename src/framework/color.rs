/// A struct representing a color with red, green, blue, and alpha components.
/// Each component is a floating-point value between 0.0 and 1.0.
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
/// Represents a color in RGBA color space.
///
/// Each component (red, green, blue, alpha) is stored as a 32-bit floating point number
/// in the range [0.0, 1.0].
///
/// # Examples
///
/// ```
/// let white = Color {
///     r: 1.0,
///     g: 1.0,
///     b: 1.0,
///     a: 1.0,
/// };
/// ```
///
/// # Fields
///
pub struct Color {
    /// * `r` - Red component
    pub r: f64,
    /// * `g` - Green component
    pub g: f64,
    /// * `b` - Blue component
    pub b: f64,
    /// * `a` - Alpha (opacity) component
    pub a: f64,
}

impl Color {
    /// Creates a new `Color` instance with the specified red, green, blue, and alpha components.
    ///
    /// # Arguments
    ///
    /// * `r` - The red component of the color.
    /// * `g` - The green component of the color.
    /// * `b` - The blue component of the color.
    /// * `a` - The alpha (opacity) component of the color.
    ///
    /// # Returns
    ///
    /// A new `Color` instance with the specified color components.
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Color { r, g, b, a }
    }

    /// Creates a new `Color` instance with the specified red, green, and blue components.
    /// The alpha component is set to 1.0 (fully opaque).
    ///
    /// # Arguments
    ///
    /// * `r` - The red component of the color.
    /// * `g` - The green component of the color.
    /// * `b` - The blue component of the color.
    ///
    /// # Returns
    ///
    /// A new `Color` instance with the specified color components.
    pub fn rgb(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b, a: 1.0 }
    }

    /// Creates a new `Color` instance with the specified red, green, blue, and alpha components.
    /// The components are specified as 8-bit integers in the range [0, 255].
    ///
    /// # Arguments
    ///
    /// * `r` - The red component of the color.
    /// * `g` - The green component of the color.
    /// * `b` - The blue component of the color.
    /// * `a` - The alpha (opacity) component of the color.
    ///
    /// # Returns
    ///
    /// A new `Color` instance with the specified color components.
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color {
            r: r as f64 / 255.0,
            g: g as f64 / 255.0,
            b: b as f64 / 255.0,
            a: a as f64 / 255.0,
        }
    }
}

/// Converts a `Color` into a `wgpu::Color`.
///
/// # Arguments
///
/// * `color` - The `Color` instance to convert.
///
/// # Returns
///
/// A `wgpu::Color` instance with the same color components as the input `Color`.
impl From<Color> for wgpu::Color {
    fn from(color: Color) -> Self {
        wgpu::Color {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        }
    }
}
