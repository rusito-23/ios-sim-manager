use tui::layout::{Constraint, Direction, Layout, Rect};

// Builder

pub fn build(size: Rect) -> Vec<Rect> {
    return Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(build_constraints())
        .split(size)
}

// Constraints

fn build_constraints() -> Vec<Constraint> {
    return [
        Constraint::Length(3),
        Constraint::Min(2),
        Constraint::Length(3),
    ].to_vec()
}
