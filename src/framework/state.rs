use super::render::Render;
use winit::dpi::PhysicalSize;
use winit::window::Window;

/// State is the main state of the application.
struct State {
    pub size: PhysicalSize<u32>,
    pub render: Render,
}

impl State {
    /// Create a new State instance.
    pub async fn new(window: &Window) -> Self {
        Self {
            size: window.inner_size(),
            render: Render::new(window),
        }
    }

    /// Resize the state.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.size = PhysicalSize::new(width, height);
        self.render.resize(width, height).unwrap();
    }

    /// Render a frame.
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.render.render();
    }
}

impl Drop for State {
    fn drop(&mut self) {
        todo!()
    }
}
