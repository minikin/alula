use super::layout::BoxConstraints;
use super::state::State;

/// Widget is the base trait for all widgets.
pub trait Widget {
    /// Build the widget.
    fn build(&self, state: &mut State);

    /// Layout the widget.
    fn layout(&mut self, constraints: BoxConstraints) -> Size;

    /// Paint the widget.
    fn paint(&self, state: &mut State);
}
