// Event utils
// Initialize the thread to react to different events

use std::time::{Duration, Instant};
use std::sync::mpsc::{self, Receiver};
use std::thread;

use crossterm::event::{self, KeyEvent};

/// Describes the possible events
pub enum Event<I> {
    Input(I),
    Tick,
}

pub fn setup() -> Receiver<Event<KeyEvent>> {

    // Create sender and receiver
    let (tx, rx) = mpsc::channel();

    // Define tick duration
    let tick_rate = Duration::from_millis(200);

    // Spawn thread
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let event::Event::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    return rx;
}
