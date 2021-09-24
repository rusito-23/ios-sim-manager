// Utils for the simctl Device struct

use simctl::list::DeviceState as State;
use crate::strings;

// Return the state in a readable way
pub fn get_state(device: &simctl::Device) -> String {
    return match device.state {
        State::Booted => strings::STATE_BOOTED,
        State::Shutdown => strings::STATE_SHUTDOWN,
        State::Unknown => strings::STATE_UNKNOWN,
    }.to_string()
}

// Return the runtime in a readable way
pub fn get_runtime(device: &simctl::Device) -> String {
    return device.runtime_identifier
        .split('.')
        .last()
        .unwrap_or(&strings::UNKNOWN.to_string())
        .to_string()
}
