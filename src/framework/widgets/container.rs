use super::widget::Widget;
use crate::framework::{BoxConstraints, Color, Size, State};
use std::fmt;

/// Container is a widget that contains another widget.
pub struct Container {
    child: Option<Box<dyn Widget>>,
    width: Option<f32>,
    height: Option<f32>,
    padding: Option<f32>,
    color: Option<Color>,
}

impl Container {
    /// Create a new Container instance.
    pub fn new() -> Self {
        Self {
            child: None,
            width: None,
            height: None,
            padding: None,
            color: None,
        }
    }

    /// Set the child of the container.
    pub fn with_child(mut self, child: impl Widget + 'static) -> Self {
        self.child = Some(Box::new(child));
        self
    }

    /// Set the size of the container.
    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// Set the padding of the container.
    pub fn with_padding(mut self, padding: f32) -> Self {
        self.padding = Some(padding);
        self
    }

    /// Set the background color of the container.
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

impl Widget for Container {
    fn build(&self, state: &mut State<'_>) {
        if let Some(child) = &self.child {
            child.build(state);
        }
    }
    /// Layout the widget.
    fn layout(&mut self, constraints: BoxConstraints) -> Size {
        let padding = self.padding.unwrap_or(0.0) * 2.0;

        if let Some(child) = &mut self.child {
            let child_constraints = BoxConstraints {
                min_width: constraints.min_width - padding,
                max_width: constraints.max_width - padding,
                min_height: constraints.min_height - padding,
                max_height: constraints.max_height - padding,
            };

            let child_size = child.layout(child_constraints);
            Size {
                width: child_size.width + padding,
                height: child_size.height + padding,
            }
        } else {
            Size {
                width: self.width.unwrap_or(constraints.min_width),
                height: self.height.unwrap_or(constraints.min_height),
            }
        }
    }
    /// Paint the widget.
    fn paint(&self, state: &mut State<'_>) {
        if let Some(_color) = self.color {
            todo!("Paint the container with the background color");
        }

        if let Some(child) = &self.child {
            child.paint(state);
        }
    }
}

impl fmt::Debug for Container {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Container")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("padding", &self.padding)
            .field("color", &self.color)
            .field("has_child", &self.child)
            .finish()
    }
}
