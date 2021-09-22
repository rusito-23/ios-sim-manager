mod strings;
mod widgets;

use crossterm::{event::{self, Event as CEvent, KeyCode}, terminal};
use std::time::{Duration, Instant};
use std::thread;
use std::io;
use std::sync::mpsc;
use tui::{backend::CrosstermBackend, Terminal};

// Custom Event Definition

enum Event<I> {
    Input(I),
    Tick,
}

// Main

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Allow re-drawing the terminal emulator content
    terminal::enable_raw_mode().expect(strings::RAW_MODE_ERROR);

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

            if event::poll(timeout).expect(strings::POLL_ERROR) {
                if let CEvent::Key(key) = event::read().expect(strings::READ_EVENTS_ERROR) {
                    tx.send(Event::Input(key)).expect(strings::SEND_EVENTS_ERROR);
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    // Clear existing terminal items
    terminal.clear()?;

    // Main application loop
    loop {

        // Display
        terminal.draw(|rect| {
            // Create app layout
            let chunks = widgets::layout::build(rect.size());

            // Create widgets
            let menu_widget = widgets::menu::build();
            let copyright_widget = widgets::copyright::build();

            // Render widgets
            rect.render_widget(menu_widget, chunks[0]);
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
