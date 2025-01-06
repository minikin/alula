use wgpu;
use winit::window::Window;

struct Render {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
}

impl Render {
    pub fn new(window: &Window) -> Self {
        todo!()
    }

    pub fn render(&self) {
        todo!()
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), wgpu::SurfaceError> {
        todo!()
    }
}
