use thiserror::Error;

/// The main error type for the Alula framework
#[derive(Debug, Error)]
pub enum Error {
    /// Errors that occur during rendering
    #[error("Render error: {0}")]
    Render(#[from] RenderError),

    /// Errors that occur during layout
    #[error("Layout error: {0}")]
    Layout(#[from] LayoutError),

    /// Errors that occur in widgets
    #[error("Widget error: {0}")]
    Widget(#[from] WidgetError),

    /// Errors that occur in state management
    #[error("State error: {0}")]
    State(#[from] StateError),
}

/// Errors that can occur during rendering
#[derive(Debug, Error)]
pub enum RenderError {
    /// Failed to create a surface
    #[error("Failed to create surface: {0}")]
    SurfaceCreationError(#[from] wgpu::CreateSurfaceError),

    /// No suitable graphics adapter was found
    #[error("No suitable graphics adapter found")]
    AdapterNotFound,

    /// Failed to create a device
    #[error("Failed to create device: {0}")]
    DeviceCreationError(#[from] wgpu::RequestDeviceError),

    /// Surface error occurred
    #[error("Surface error: {0}")]
    SurfaceError(#[from] wgpu::SurfaceError),
}

/// Errors that can occur during layout
#[derive(Debug, Error)]
pub enum LayoutError {
    /// Invalid constraints were provided
    #[error("Invalid constraints: {0}")]
    InvalidConstraints(String),

    /// Layout overflow occurred
    #[error("Layout overflow")]
    Overflow,
}

/// Errors that can occur in widgets
#[derive(Debug, Error)]
pub enum WidgetError {
    /// Widget build failed
    #[error("Widget build failed: {0}")]
    BuildError(String),

    /// Invalid child widget
    #[error("Invalid child widget")]
    InvalidChild,
}

/// Errors that can occur in state management
#[derive(Debug, Error)]
pub enum StateError {
    /// Invalid state access
    #[error("Invalid state access: {0}")]
    InvalidStateAccess(String),

    /// State not found
    #[error("State not found")]
    StateNotFound,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_errors() {
        // Test surface error conversion
        let surface_error = RenderError::from(wgpu::SurfaceError::Lost);
        assert!(matches!(surface_error, RenderError::SurfaceError(_)));

        // Test adapter not found error
        let adapter_error = RenderError::AdapterNotFound;
        assert_eq!(
            adapter_error.to_string(),
            "No suitable graphics adapter found"
        );
    }

    #[test]
    fn test_layout_errors() {
        let layout_error = LayoutError::InvalidConstraints("width cannot be negative".to_string());
        assert_eq!(
            layout_error.to_string(),
            "Invalid constraints: width cannot be negative"
        );

        let overflow_error = LayoutError::Overflow;
        assert_eq!(overflow_error.to_string(), "Layout overflow");
    }

    #[test]
    fn test_widget_errors() {
        let build_error = WidgetError::BuildError("missing required property".to_string());
        assert_eq!(
            build_error.to_string(),
            "Widget build failed: missing required property"
        );

        let child_error = WidgetError::InvalidChild;
        assert_eq!(child_error.to_string(), "Invalid child widget");
    }

    #[test]
    fn test_state_errors() {
        let access_error = StateError::InvalidStateAccess("state already borrowed".to_string());
        assert_eq!(
            access_error.to_string(),
            "Invalid state access: state already borrowed"
        );

        let not_found_error = StateError::StateNotFound;
        assert_eq!(not_found_error.to_string(), "State not found");
    }

    #[test]
    fn test_error_conversion() {
        // Test conversion from RenderError to Error
        let render_error = RenderError::AdapterNotFound;
        let error: Error = render_error.into();
        assert!(matches!(error, Error::Render(_)));

        // Test conversion from LayoutError to Error
        let layout_error = LayoutError::Overflow;
        let error: Error = layout_error.into();
        assert!(matches!(error, Error::Layout(_)));

        // Test conversion from WidgetError to Error
        let widget_error = WidgetError::InvalidChild;
        let error: Error = widget_error.into();
        assert!(matches!(error, Error::Widget(_)));

        // Test conversion from StateError to Error
        let state_error = StateError::StateNotFound;
        let error: Error = state_error.into();
        assert!(matches!(error, Error::State(_)));
    }

    #[test]
    fn test_error_display() {
        let error = Error::Widget(WidgetError::InvalidChild);
        assert_eq!(error.to_string(), "Widget error: Invalid child widget");

        let error = Error::Layout(LayoutError::Overflow);
        assert_eq!(error.to_string(), "Layout error: Layout overflow");
    }
}
