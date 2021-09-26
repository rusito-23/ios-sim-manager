mod strings;
mod widgets;
mod device_utils;
mod app;
mod event_manager;

use std::{error::Error, io};
use crossterm;
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), Box<dyn Error>> {

    // Allow re-drawing the terminal emulator content
    crossterm::terminal::enable_raw_mode().unwrap();

    // Initialize terminal
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Setup event receiver
    let rx = event_manager::setup();

    // Initialize application
    let mut app = app::App::default();

    // Main application loop
    loop {

        // Display
        terminal.draw(|rect| {
            // Create app layout
            let chunks = widgets::layout::build(rect.size());

            // Create widgets
            let menu_widget = widgets::menu::build();
            let search_bar = widgets::search_bar::build(app.input.clone());

            // Render widgets
            rect.render_widget(menu_widget, chunks[0]);
            rect.render_widget(search_bar, chunks[2]);

            // Display state driven components
            match app.state {

                // Display devices
                app::State::Normal => {
                    let home_widget = widgets::devices::build(app.devices());
                    rect.render_stateful_widget(home_widget, chunks[1], &mut app.table_state);
                }

                // Display help
                app::State::Help => {
                    let help_widget = widgets::help::build();
                    rect.render_widget(help_widget, chunks[1]);
                }

                // Display search bar cursor
                app::State::Search => {
                    let chunk = chunks[2];
                    let x_pos = chunk.x + app.input.len() as u16;
                    let y_pos = chunk.y;
                    rect.set_cursor(x_pos + 1, y_pos + 1);
                }
            }
        })?;

        // Handle events
        match rx.recv()? {
            // Key event handler
            event_manager::Event::Input(input) => {

                // Check current app state
                match app.state {

                    // Normal state events
                    app::State::Normal => match input.code {

                        // Screenshot action
                        crossterm::event::KeyCode::Char('S') => {
                            app.take_screenshot();
                        }

                        // Copy UDID action
                        crossterm::event::KeyCode::Char('C') => {
                            app.copy_udid();
                        }

                        // Display Help action
                        crossterm::event::KeyCode::Char('H') => {
                            app.state = app::State::Help;
                        }

                        // Quit event
                        crossterm::event::KeyCode::Char('Q') => {
                            crossterm::terminal::disable_raw_mode()?;
                            terminal.show_cursor()?;
                            break;
                        }

                        // Begin search
                        crossterm::event::KeyCode::Char('/') => {
                            app.state = app::State::Search;
                        }

                        // Clear search
                        crossterm::event::KeyCode::Esc => {
                            app.clear_search();
                        }

                        // Navigation down event
                        crossterm::event::KeyCode::Char('j') => {
                            app.decrement_selection();
                        }

                        // Navigation up event
                        crossterm::event::KeyCode::Char('k') => {
                            app.increment_selection();
                        }

                        _ => {}
                    }

                    // Search state events
                    app::State::Search => match input.code {

                        // Apply search
                        crossterm::event::KeyCode::Enter => {
                            app.reset_selection();
                            app.state = app::State::Normal;
                        }

                        // Clear search
                        crossterm::event::KeyCode::Esc => {
                            app.clear_search();
                            app.state = app::State::Normal;
                        }

                        // Add character to the search
                        crossterm::event::KeyCode::Char(c) => {
                            app.input.push(c);
                        }

                        // Remove character from the search
                        crossterm::event::KeyCode::Backspace => {
                            app.input.pop();
                        }

                        _ => {}
                    }

                    // Help state events
                    app::State::Help => match input.code {
                        // Any key goes back to normal
                        _ => {
                            app.clear_search();
                            app.state = app::State::Normal;
                        }
                    }
                }
            },

            // Tick event handler
            event_manager::Event::Tick => {}
        }
    }

    Ok(())
}
