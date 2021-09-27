use crate::strings;
use tui;

// Builder

pub fn build(message: &'static str) -> tui::widgets::Paragraph<'static> {
    return tui::widgets::Paragraph::new(vec![tui::text::Spans::from(vec![tui::text::Span::raw(message)])])
        .alignment(tui::layout::Alignment::Center)
        .block(build_block());
}

// Block Builder

fn build_block() -> tui::widgets::Block<'static> {
    return tui::widgets::Block::default()
        .borders(tui::widgets::Borders::ALL)
        .style(tui::style::Style::default().fg(tui::style::Color::White))
        .title(strings::HELP_TITLE)
        .border_type(tui::widgets::BorderType::Plain);
}
