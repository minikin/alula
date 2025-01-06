use super::render::Render;
use winit::dpi::PhysicalSize;
use winit::window::Window;

struct State {
    pub size: PhysicalSize<u32>,
    pub render: Render,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        Self {
            size: window.inner_size(),
            render: Render::new(window),
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.size = PhysicalSize::new(width, height);
        self.render.resize(width, height).unwrap();
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.render.render();
    }
}
