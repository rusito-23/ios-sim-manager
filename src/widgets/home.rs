use tui;
use simctl;

// Builder

pub fn build(devices: &[simctl::Device]) -> tui::widgets::List<'static> {
    let mut items: Vec<tui::widgets::ListItem> = Vec::new();

    for sim in devices {
        let uudid = sim.name.clone();
        let content = tui::text::Span::styled(uudid, tui::style::Style::default());
        let item = tui::widgets::ListItem::new(content);
        items.push(item);
    }

    return tui::widgets::List::new(items);
}
