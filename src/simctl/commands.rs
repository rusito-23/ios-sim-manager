// Commands used by the simctl interface

use std::process::Command;

// List devices

pub fn make_list_devices() -> Command {
    let mut command = Command::new("xcrun");

    command
        .arg("simctl")
        .arg("list")
        .arg("devices")
        .arg("-j");

    return command;
}
