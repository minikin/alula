use super::render::Render;
use winit::window::Window;

/// State is the main state of the application.
#[derive(Debug)]
pub struct State<'a> {
    render: Option<Render<'a>>,
}

impl<'a> State<'a> {
    /// Create a new State instance.
    pub async fn new(window: &'a Window) -> Result<Self, crate::framework::error::RenderError> {
        let render = Render::new(window).await?;
        Ok(Self {
            render: Some(render),
        })
    }

    /// Get a reference to the renderer.
    pub fn render(&self) -> Option<&Render<'a>> {
        self.render.as_ref()
    }

    /// Get a mutable reference to the renderer.
    pub fn render_mut(&mut self) -> Option<&mut Render<'a>> {
        self.render.as_mut()
    }
}