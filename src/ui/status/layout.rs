use ratatui::{buffer::Buffer, layout::{self, Constraint, Direction, Rect}, widgets::Widget};

use super::{Left, Right};
use crate::ui::Ctx;

pub struct Layout<'a> {
	cx: &'a Ctx,
}

impl<'a> Layout<'a> {
	pub fn new(cx: &'a Ctx) -> Self { Self { cx } }
}

impl<'a> Widget for Layout<'a> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let chunks = layout::Layout::default()
			.direction(Direction::Horizontal)
			.constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
			.split(area);

		Left::new(self.cx).render(chunks[0], buf);
		Right::new(self.cx).render(chunks[1], buf);
	}
}
