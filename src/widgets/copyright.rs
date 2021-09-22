use tui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::strings;


// Builder

pub fn build() -> Paragraph<'static> {
    return Paragraph::new(strings::COPYRIGHT_BODY)
        .style(build_copyright_style())
        .alignment(Alignment::Center)
        .block(build_copyright_block())
}

// Styles

fn build_copyright_style() -> Style {
    return Style::default()
        .fg(Color::LightCyan)
}

fn build_copyright_border_style() -> Style {
    return Style::default()
        .fg(Color::White)
}

// Block Builder

fn build_copyright_block() -> Block<'static> {
    return Block::default()
        .borders(Borders::ALL)
        .style(build_copyright_border_style())
        .title(strings::COPYRIGHT_TITLE)
        .border_type(BorderType::Plain)
}
