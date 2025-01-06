use wgpu;
use winit::window::Window;

/// Render is a wrapper around wgpu's surface, device, queue, and configuration.
struct Render {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
}

impl Render {
    /// Create a new Render instance.
    pub fn new(window: &Window) -> Self {
        todo!()
    }

    /// Render a frame.
    pub fn render(&self) {
        todo!()
    }

    /// Resize the render target.
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), wgpu::SurfaceError> {
        todo!()
    }
}
