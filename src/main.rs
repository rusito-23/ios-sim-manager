mod strings;
mod widgets;

use crossterm::{event::{self, Event as CEvent, KeyCode}, terminal};
use std::time::{Duration, Instant};
use std::thread;
use std::io;
use std::sync::mpsc;
use tui::{backend::CrosstermBackend, Terminal};

// Custom Event Definition
// TODO: Move to a separate file

enum Event<I> {
    Input(I),
    Tick,
}

// Main

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Allow re-drawing the terminal emulator content
    terminal::enable_raw_mode().unwrap();

    // Definitions
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create channels
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);

    // Initialize and set up the main thread
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).unwrap() {
                if let CEvent::Key(key) = event::read().unwrap() {
                    tx.send(Event::Input(key)).unwrap();
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    // Clear existing terminal output
    terminal.clear()?;

    // Initialize simctl
    let simctl = simctl::Simctl::new();
    let mut sim_list = simctl.list().unwrap();

    // Initial state
    let mut table_state = tui::widgets::TableState::default();
    table_state.select(Some(0));

    // Main application loop
    loop {

        // Display
        terminal.draw(|rect| {
            // Create app layout
            let chunks = widgets::layout::build(rect.size());

            // Create widgets
            let menu_widget = widgets::menu::build();
            let home_widget = widgets::home::build(sim_list.devices());
            let copyright_widget = widgets::copyright::build();

            // Render widgets
            rect.render_widget(menu_widget, chunks[0]);
            rect.render_stateful_widget(home_widget, chunks[1], &mut table_state);
            rect.render_widget(copyright_widget, chunks[2]);
        })?;

        // Handle events
        match rx.recv()? {
            Event::Input(event) => match event.code {

                // Exit Event
                KeyCode::Char('Q') => {
                    terminal::disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }

                // Refresh Event
                KeyCode::Char('R') => {
                    sim_list.refresh().unwrap();
                    break;
                }

                // Navigation down event
                KeyCode::Char('j') => {
                    if let Some(selected) = table_state.selected() {
                        if selected >= sim_list.devices().len() - 1 {
                            table_state.select(Some(0));
                        } else {
                            table_state.select(Some(selected + 1));
                        }
                    }
                }

                // Navigation up event
                KeyCode::Char('k') => {
                    if let Some(selected) = table_state.selected() {
                        if selected > 0 {
                            table_state.select(Some(selected - 1));
                        } else {
                            table_state.select(Some(sim_list.devices().len() - 1));
                        }
                    }
                }

                // Default event handler
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}
