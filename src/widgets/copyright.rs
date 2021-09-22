use crate::strings;
use tui::{layout, style, widgets as tui_widgets};


// Builder

pub fn build() -> tui_widgets::Paragraph<'static> {
    return tui_widgets::Paragraph::new(strings::COPYRIGHT_BODY)
        .style(build_copyright_style())
        .alignment(layout::Alignment::Center)
        .block(build_copyright_block())
}

// Styles

fn build_copyright_style() -> style::Style {
    return style::Style::default()
        .fg(style::Color::LightCyan)
}

fn build_copyright_border_style() -> style::Style {
    return style::Style::default()
        .fg(style::Color::White)
}

// Block Builder

fn build_copyright_block() -> tui_widgets::Block<'static> {
    return tui_widgets::Block::default()
        .borders(tui_widgets::Borders::ALL)
        .style(build_copyright_border_style())
        .title(strings::COPYRIGHT_TITLE)
        .border_type(tui_widgets::BorderType::Plain)
}
