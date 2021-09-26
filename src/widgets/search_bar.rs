use crate::strings;
use tui;


// Builder

pub fn build(value: String) -> tui::widgets::Paragraph<'static> {
    return tui::widgets::Paragraph::new(value)
        .style(build_style())
        .block(build_block())
}

// Styles

fn build_style() -> tui::style::Style {
    return tui::style::Style::default()
        .fg(tui::style::Color::LightCyan)
}

fn build_border_style() -> tui::style::Style {
    return tui::style::Style::default()
        .fg(tui::style::Color::White)
}

// Block Builder

fn build_block() -> tui::widgets::Block<'static> {
    return tui::widgets::Block::default()
        .borders(tui::widgets::Borders::ALL)
        .style(build_border_style())
        .title(strings::SEARCH_BAR_TITLE)
        .border_type(tui::widgets::BorderType::Plain)
}
