use getset::{Setters, WithSetters};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},
};

use crate::filter_mode::FilterMode;

#[derive(Debug, Default, WithSetters)]
#[getset(set_with = "pub")]
pub struct FilterWindow {
    filter_mode: FilterMode,
}

impl Widget for FilterWindow {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Ensure that all cells under the popup are cleared to avoid leaking content
        Clear.render(area, buf);
        let block = Block::new()
            .title("Filter")
            .title_style(Style::new().white().bold())
            .borders(Borders::ALL)
            .border_style(Style::new().red());
        Paragraph::new("This is a test")
            .wrap(Wrap { trim: true })
            .style(Style::new().yellow())
            .block(block)
            .render(area, buf);
    }
}
