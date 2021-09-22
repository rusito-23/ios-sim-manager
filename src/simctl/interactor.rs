use serde_json as json;
use crate::simctl::commands;
use crate::strings;

pub fn list_simulators() -> Result<Vec<json::Value>, Box<dyn std::error::Error>> {

    // Run the list devices command
    let command_output = commands::make_list_devices().output().unwrap();

    // Check the command output
    if !command_output.status.success() {
        return Err(strings::LIST_SIM_ERROR.into())
    }

    // Parse JSON object from stdout
    let stdout = String::from_utf8_lossy(&command_output.stdout);
    let object: json::Value = json::from_str(&stdout)?;

    // Get device list
    let devices_by_os: json::Map<String, json::Value> = object["devices"]
        .as_object()
        .unwrap()
        .clone();

    // Create device vector
    let mut simulators = Vec::new();

    // Fill the device vector with parsed data
    for (key, value) in devices_by_os {

        // Get all of the devices for the current OS
        let devices = value.as_array().unwrap().clone();

        // Add the OS to every listed device
        for device in devices {
            let mut new_device = device.clone();
            new_device["os_info"] = json::Value::String(key.to_string().clone());
            simulators.push(new_device);
        }
    }

    return Ok(simulators);
}


