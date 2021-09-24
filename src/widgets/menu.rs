use crate::strings;
use tui;

// Titles

static MENU_TITLES: [&str; 3] = [
    strings::HOME_TAB_TITLE,
    strings::LIST_TAB_TITLE,
    strings::QUIT_TAB_TITLE,
];

// Builder

pub fn build() -> tui::widgets::Tabs<'static> {
    return tui::widgets::Tabs::new(build_menu_items())
        .block(build_block())
        .style(build_regular_style())
        .divider(tui::text::Span::raw("|"));
}

// Menu items builder

fn build_menu_items() -> Vec<tui::text::Spans<'static>> {
    return MENU_TITLES.iter().map(|title| {
        let (initial, rest) = title.split_at(1);
        tui::text::Spans::from(vec![
            tui::text::Span::styled(initial, build_highlight_style()),
            tui::text::Span::styled(rest, build_regular_style()),
        ])
    }).collect()
}

// Style builders

fn build_highlight_style() -> tui::style::Style {
    return tui::style::Style::default()
        .fg(tui::style::Color::LightCyan)
        .add_modifier(tui::style::Modifier::UNDERLINED)
}

fn build_regular_style() -> tui::style::Style {
    return tui::style::Style::default()
        .fg(tui::style::Color::White)
}

// Block Builder

fn build_block() -> tui::widgets::Block<'static> {
    return tui::widgets::Block::default()
        .title(strings::MENU_TITLE)
        .borders(tui::widgets::Borders::ALL)
}
