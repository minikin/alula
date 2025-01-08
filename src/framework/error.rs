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