use crate::strings;
use tui;


// Builder

pub fn build() -> tui::widgets::Paragraph<'static> {
    return tui::widgets::Paragraph::new(strings::COPYRIGHT_BODY)
        .style(build_copyright_style())
        .alignment(tui::layout::Alignment::Center)
        .block(build_copyright_block())
}

// Styles

fn build_copyright_style() -> tui::style::Style {
    return tui::style::Style::default()
        .fg(tui::style::Color::LightCyan)
}

fn build_copyright_border_style() -> tui::style::Style {
    return tui::style::Style::default()
        .fg(tui::style::Color::White)
}

// Block Builder

fn build_copyright_block() -> tui::widgets::Block<'static> {
    return tui::widgets::Block::default()
        .borders(tui::widgets::Borders::ALL)
        .style(build_copyright_border_style())
        .title(strings::COPYRIGHT_TITLE)
        .border_type(tui::widgets::BorderType::Plain)
}
