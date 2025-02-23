use ratatui::{buffer::Buffer, layout::{Alignment, Constraint, Layout, Margin, Rect}, text::Line, widgets::{Block, BorderType, Widget}};
use yazi_config::THEME;

use crate::Ctx;

pub(crate) struct Confirm<'a> {
	cx: &'a Ctx,
}

impl<'a> Confirm<'a> {
	pub(crate) fn new(cx: &'a Ctx) -> Self { Self { cx } }
}

impl Widget for Confirm<'_> {
	fn render(self, _win: Rect, buf: &mut Buffer) {
		let confirm = &self.cx.confirm;
		let area = self.cx.manager.area(confirm.position);

		yazi_plugin::elements::Clear::default().render(area, buf);

		Block::bordered()
			.border_type(BorderType::Rounded)
			.border_style(THEME.confirm.border)
			.title(Line::styled(&confirm.title, THEME.confirm.title))
			.title_alignment(Alignment::Center)
			.render(area, buf);

		let content = confirm.content.clone();
		let content_height = content.line_count(area.width).saturating_add(1) as u16;

		let chunks = Layout::vertical([
			Constraint::Length(if content_height == 1 { 0 } else { content_height }),
			Constraint::Fill(1),
			Constraint::Length(1),
		])
		.split(area.inner(Margin::new(0, 1)));

		super::Content::new(content).render(chunks[0], buf);
		super::List::new(self.cx).render(chunks[1], buf);
		super::Buttons.render(chunks[2], buf);
	}
}
