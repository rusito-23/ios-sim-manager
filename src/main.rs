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

    // Retrieve available simulators
    let simctl = simctl::Simctl::new();
    let sim_list = simctl.list().unwrap();
    let devices = sim_list.devices();

    // Main application loop
    loop {

        // Display
        terminal.draw(|rect| {
            // Create app layout
            let chunks = widgets::layout::build(rect.size());

            // Create widgets
            let menu_widget = widgets::menu::build();
            let home_widget = widgets::home::build(devices);
            let copyright_widget = widgets::copyright::build();

            // Render widgets
            rect.render_widget(menu_widget, chunks[0]);
            rect.render_widget(home_widget, chunks[1]);
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

                // Default event handler
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}
