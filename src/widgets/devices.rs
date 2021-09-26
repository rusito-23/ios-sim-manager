use tui;
use simctl;
use crate::strings;
use crate::device_utils;

// Constants

const WIDTHS: &[tui::layout::Constraint] = &[
    tui::layout::Constraint::Percentage(30),
    tui::layout::Constraint::Percentage(30),
    tui::layout::Constraint::Percentage(20),
    tui::layout::Constraint::Percentage(20),
];

static HEADER_TITLES: &[&str] = &[
    strings::NAME_TITLE,
    strings::UDID_TITLE,
    strings::STATE_TITLE,
    strings::RUNTIME_TITLE,
];

// Builder

pub fn build(devices: Vec<&simctl::Device>) -> tui::widgets::Table<'static> {

    // Create rows
    let rows: Vec<tui::widgets::Row> = devices
        .iter()
        .map(|device| { build_row(device) })
        .collect();

    // Create table with rows
    let table = tui::widgets::Table::new(rows)
        .header(build_header())
        .block(build_block())
        .highlight_style(build_highlight_style())
        .widths(WIDTHS);

    return table;
}

// Styles

fn build_highlight_style() -> tui::style::Style {
    return tui::style::Style::default()
        .fg(tui::style::Color::Cyan)
}

fn build_header_style() -> tui::style::Style {
    return tui::style::Style::default()
        .fg(tui::style::Color::Cyan)
        .add_modifier(tui::style::Modifier::BOLD)
        .add_modifier(tui::style::Modifier::UNDERLINED)
}

fn build_block_style() -> tui::style::Style {
    return tui::style::Style::default()
        .fg(tui::style::Color::White)
}

// Cells

fn build_header_cell(title: String) -> tui::widgets::Cell<'static> {
    return tui::widgets::Cell::from(
        tui::text::Span::styled(
            title,
            build_header_style()
        )
    )
}

fn build_cell(text: String) -> tui::widgets::Cell<'static> {
    return tui::widgets::Cell::from(
        tui::text::Span::styled(
            text,
            tui::style::Style::default()
        )
    )
}

// Builders

fn build_header() -> tui::widgets::Row<'static> {
    let cells = HEADER_TITLES.iter().map( |title| { build_header_cell(title.to_string()) });
    return tui::widgets::Row::new(cells);
}

fn build_row(device: &simctl::Device) -> tui::widgets::Row<'static> {
    return tui::widgets::Row::new(vec![
        build_cell(device.name.clone()),
        build_cell(device.udid.clone()),
        build_cell(device_utils::get_state(device)),
        build_cell(device_utils::get_runtime(device)),
    ]);
}

fn build_block() -> tui::widgets::Block<'static> {
    return tui::widgets::Block::default()
        .borders(tui::widgets::Borders::ALL)
        .style(build_block_style())
        .border_type(tui::widgets::BorderType::Plain)
}
