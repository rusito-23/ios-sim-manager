/// Holds the application logic and state manage

use simctl;
use tui;

/// Describes the app state
pub enum State {
    Normal,
    Search
    // TODO: Missing States:
    // Help
    // Message -> Either Success or Error
}

/// Holds the app state and configuration
pub struct App {
    /// Current search input
    pub input: String,

    /// Current state
    pub state: State,

    /// The simctl list
    pub list: simctl::list::List,

    /// The current table state
    pub table_state: tui::widgets::TableState,
}

/// Application Initialization
impl Default for App {
    fn default() -> App {

        // Initialize application
        let mut app = App {
            input: String::new(),
            state: State::Normal,
            list: simctl::Simctl::new().list().unwrap(),
            table_state: tui::widgets::TableState::default()
        };

        // Create default values
        app.reset_selection();

        return app;
    }
}

/// Application selection handling methods
impl App {

    /// Reset selection back to the first element
    pub fn reset_selection(&mut self) {
        self.table_state.select(Some(0));
    }

    /// Increment the selection
    pub fn increment_selection(&mut self) {
        if let Some(selected) = self.table_state.selected() {
            if selected > 0 {
                self.table_state.select(Some(selected - 1));
            } else {
                self.table_state.select(Some(self.list.devices().len() - 1));
            }
        }
    }

    /// Decrement selection
    pub fn decrement_selection(&mut self) {
        if let Some(selected) = self.table_state.selected() {
            if selected >= self.list.devices().len() - 1 {
                self.table_state.select(Some(0));
            } else {
                self.table_state.select(Some(selected + 1));
            }
        }
    }
}

/// Application device retrieval
impl App {

    /// Retrieve devices
    /// either all of them or
    /// the ones that match the current search input
    pub fn devices(&mut self) -> Vec<&simctl::Device> {

        // Get devices
        let devices = self.list.devices();

        // Only filter if the string if non-empty
        if !self.input.trim().is_empty() {
            // Define search input
            let input = self.input.to_lowercase();

            // Kind-of Fuzzy Filter
            return devices
                .iter()
                .filter( |device| { device.name.to_lowercase().contains(&input) })
                .collect();
        }

        // By default return all of them
        return devices.iter().collect();
    }
}

/// Application Actions
impl App {

    /// Take a screenshot for the current selected device
    pub fn take_screenshot(&mut self) {
        // TODO: Implementation
    }

    /// Copy current device hash
    pub fn copy_udid(&mut self) {
        // TODO: Implementation
    }

    /// Show the help pages
    pub fn show_help(&mut self) {
        // TODO: Implementation
    }
}
