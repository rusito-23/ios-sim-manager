use tui;
use simctl;
use crate::strings;

// Constants

const WIDTHS: &[tui::layout::Constraint] = &[
    tui::layout::Constraint::Percentage(30),
    tui::layout::Constraint::Percentage(30),
    tui::layout::Constraint::Percentage(20),
    tui::layout::Constraint::Percentage(20),
];

// Builder

pub fn build(devices: &[simctl::Device]) -> tui::widgets::Table<'static> {

    // Create rows
    let rows: Vec<tui::widgets::Row> = devices.iter().map(|device| {
        return tui::widgets::Row::new(vec![
            build_cell(device.name.clone()),
            build_cell(device.udid.clone()),
        ]);
    }).collect();

    // Create table with rows
    let table = tui::widgets::Table::new(rows)
        .header(build_header())
        .block(build_block())
        .widths(WIDTHS);

    return table;
}

// Styles

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
    return tui::widgets::Row::new(vec![
        build_header_cell(strings::NAME_TITLE.to_string()),
        build_header_cell(strings::UDID_TITLE.to_string()),
    ])
}

fn build_block() -> tui::widgets::Block<'static> {
    return tui::widgets::Block::default()
        .borders(tui::widgets::Borders::ALL)
        .style(build_block_style())
        .border_type(tui::widgets::BorderType::Plain)
}
