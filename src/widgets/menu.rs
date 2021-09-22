use crate::strings;
use tui::{style, text, widgets as tui_widgets};

// Titles

static MENU_TITLES: [&str; 3] = [
    strings::HOME_TAB_TITLE,
    strings::LIST_TAB_TITLE,
    strings::QUIT_TAB_TITLE,
];

// Builder

pub fn build() -> tui_widgets::Tabs<'static> {
    return tui_widgets::Tabs::new(build_menu_items())
        .block(build_block())
        .style(build_regular_style())
        .highlight_style(build_highlight_style())
        .divider(text::Span::raw("|"));
}

// Menu items builder

fn build_menu_items() -> Vec<text::Spans<'static>> {
    return MENU_TITLES.iter().map(|t| {
        let (initial, rest) = t.split_at(1);
        text::Spans::from(vec![
            text::Span::styled(initial, build_highlight_style()),
            text::Span::styled(rest, build_regular_style()),
        ])
    }).collect()
}

// Style builders

fn build_highlight_style() -> style::Style {
    return style::Style::default()
        .fg(style::Color::LightCyan)
        .add_modifier(style::Modifier::UNDERLINED)
}

fn build_regular_style() -> style::Style {
    return style::Style::default()
        .fg(style::Color::White)
}

// Block Builder

fn build_block() -> tui_widgets::Block<'static> {
    return tui_widgets::Block::default()
        .title(strings::MENU_TITLE)
        .borders(tui_widgets::Borders::ALL)
}
