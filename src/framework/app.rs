use super::widgets::Widget;

#[derive(Debug)]
pub struct Application {
    title: String,
    root: Box<dyn Widget>,
}

impl Application {
    pub fn new(title: impl Into<String>, root: impl Widget + 'static) -> Self {
        Self {
            title: title.into(),
            root: Box::new(root),
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}

impl Widget for Application {
    fn build(&self, state: &mut State<'_>) {
        self.root.build(state);
    }

    fn layout(&mut self, constraints: BoxConstraints) -> Size {
        self.root.layout(constraints)
    }

    fn paint(&self, state: &mut State<'_>) {
        self.root.paint(state);
    }
}
