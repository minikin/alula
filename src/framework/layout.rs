/// Size is a 2D size.
#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

/// Offset is a 2D offset.
#[derive(Debug, Clone, Copy)]
pub struct Offset {
    pub x: f32,
    pub y: f32,
}

/// BoxConstraints is a set of constraints for a widget's size.
#[derive(Debug, Clone, Copy)]
pub struct BoxConstraints {
    pub min_width: f32,
    pub max_width: f32,
    pub min_height: f32,
    pub max_height: f32,
}

impl BoxConstraints {
    /// Create a new BoxConstraints instance with tight constraints.
    pub fn tight(size: Size) -> Self {
        BoxConstraints {
            min_width: size.width,
            max_width: size.width,
            min_height: size.height,
            max_height: size.height,
        }
    }

    /// Create a new BoxConstraints instance with loose constraints.
    pub fn loose(size: Size) -> Self {
        BoxConstraints {
            min_width: 0.0,
            max_width: size.width,
            min_height: 0.0,
            max_height: size.height,
        }
    }
}

/// Alignment is a set of alignments for a widget's position.
#[derive(Debug, Clone, Copy)]
pub enum Alignment {
    /// Align to the top-left corner.
    TopLeft,
    /// Align to the top-center.
    TopCenter,
    /// Align to the top-right corner.
    TopRight,
    /// Align to the center-left.
    CenterLeft,
    /// Align to the center.
    Center,
    /// Align to the center-right.
    CenterRight,
    /// Align to the bottom-left corner.
    BottomLeft,
    /// Align to the bottom-center.
    BottomCenter,
    /// Align to the bottom-right corner.
    BottomRight,
}
