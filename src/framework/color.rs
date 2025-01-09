/// A struct representing a color with red, green, blue, and alpha components.
/// Each component is a floating-point value between 0.0 and 1.0.
#[derive(Debug, Clone, Copy)]
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
    pub r: f32,
    /// * `g` - Green component
    pub g: f32,
    /// * `b` - Blue component
    pub b: f32,
    /// * `a` - Alpha (opacity) component
    pub a: f32,
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
            r: color.r as f64,
            g: color.g as f64,
            b: color.b as f64,
            a: color.a as f64,
        }
    }
}
