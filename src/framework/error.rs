use thiserror::Error;

/// The main error type for the Alula framework
#[derive(Debug, Error)]
pub enum Error {
    #[error("Render error: {0}")]
    Render(#[from] RenderError),
    
    #[error("Layout error: {0}")]
    Layout(#[from] LayoutError),
    
    #[error("Widget error: {0}")]
    Widget(#[from] WidgetError),
    
    #[error("State error: {0}")]
    State(#[from] StateError),
}

/// Errors that can occur during rendering
#[derive(Debug, Error)]
pub enum RenderError {
    #[error("Failed to create surface: {0}")]
    SurfaceCreationError(#[from] wgpu::CreateSurfaceError),
    
    #[error("No suitable graphics adapter found")]
    AdapterNotFound,
    
    #[error("Failed to create device: {0}")]
    DeviceCreationError(#[from] wgpu::RequestDeviceError),
    
    #[error("Surface error: {0}")]
    SurfaceError(#[from] wgpu::SurfaceError),
}

/// Errors that can occur during layout
#[derive(Debug, Error)]
pub enum LayoutError {
    #[error("Invalid constraints: {0}")]
    InvalidConstraints(String),
    
    #[error("Layout overflow")]
    Overflow,
}

/// Errors that can occur in widgets
#[derive(Debug, Error)]
pub enum WidgetError {
    #[error("Widget build failed: {0}")]
    BuildError(String),
    
    #[error("Invalid child widget")]
    InvalidChild,
}

/// Errors that can occur in state management
#[derive(Debug, Error)]
pub enum StateError {
    #[error("Invalid state access: {0}")]
    InvalidStateAccess(String),
    
    #[error("State not found")]
    StateNotFound,
}