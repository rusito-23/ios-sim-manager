use crate::strings;
use tui;

// Builder

pub fn build() -> tui::widgets::Paragraph<'static> {
    return tui::widgets::Paragraph::new(vec![
        build_header(),

        // Normal mode commands
        break_line(),
        build_title(strings::HELP_NORMAL_COMMANDS_TITLE),
        break_line(),

        build_command_help(strings::HELP_NAV_CMD, strings::HELP_NAV_CMD_DESCRIPTION),
        build_command_help(strings::HELP_SEARCH_CMD, strings::HELP_SEARCH_CMD_DESCRIPTION),
        build_command_help(strings::HELP_CLEAR_CMD, strings::HELP_CLEAR_CMD_DESCRIPTION),
        build_command_help(strings::HELP_SCREENSHOT_CMD, strings::HELP_SCREENSHOT_CMD_DESCRIPTION),
        build_command_help(strings::HELP_COPY_CMD, strings::HELP_COPY_CMD_DESCRIPTION),
        build_command_help(strings::HELP_QUIT_CMD, strings::HELP_QUIT_CMD_DESCRIPTION),
        build_command_help(strings::HELP_HELP_CMD, strings::HELP_HELP_CMD_DESCRIPTION),

        // Search mode commands
        break_line(),
        build_title(strings::HELP_SEARCH_COMMANDS_TITLE),
        break_line(),
        build_command_help(strings::HELP_CLEAR_CMD, strings::HELP_CLEAR_CMD_DESCRIPTION),
        build_command_help(strings::HELP_APPLY_CMD, strings::HELP_APPLY_CMD_DESCRIPTION),
    ])
    .alignment(tui::layout::Alignment::Center)
    .block(build_block());
}

// Text builders

fn break_line() -> tui::text::Spans<'static> {
    return tui::text::Spans::from(vec![
        tui::text::Span::raw(""), // break line
    ]);
}

fn build_header() -> tui::text::Spans<'static> {
    return tui::text::Spans::from(vec![
        tui::text::Span::raw(strings::HELP_WELCOME_TITLE),
        tui::text::Span::styled("ios-sim-manager", title_style()),
        tui::text::Span::raw("!"),
    ]);
}

fn build_title(title: &'static str) -> tui::text::Spans<'static> {
    return tui::text::Spans::from(vec![
        tui::text::Span::styled(title, title_style()),
        tui::text::Span::raw(""), // break line
    ]);
}

fn build_command_help(c: &'static str, d: &'static str) -> tui::text::Spans<'static>  {
    return tui::text::Spans::from(vec![
        tui::text::Span::styled(c, title_style()),
        tui::text::Span::raw(d),
    ]);
}

// Style Builders

fn title_style() -> tui::style::Style {
    return tui::style::Style::default()
        .fg(tui::style::Color::Cyan);
}

// Block Builder

fn build_block() -> tui::widgets::Block<'static> {
    return tui::widgets::Block::default()
        .borders(tui::widgets::Borders::ALL)
        .style(tui::style::Style::default().fg(tui::style::Color::White))
        .title(strings::HELP_TITLE)
        .border_type(tui::widgets::BorderType::Plain);
}
