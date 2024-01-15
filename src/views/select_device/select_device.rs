use std::ops::Add;

use adbridger::struct_def::Device;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};

use crate::utils::clear_stdout_up;

pub fn select_device(theme: &ColorfulTheme, term: &Term, device: &mut Device) {
    loop {
        let devices = adbridger::device::list_devices()
            .expect("Impossible de recuperer la liste des appareille");

        let mut devices_id: Vec<String> = devices
            .iter()
            .map(|device| device.device_id.clone().add(" | mode: ").add(&device.mode))
            .collect();

        devices_id.insert(devices_id.len(), "go back".to_string());
        let selection = Select::with_theme(theme)
            .with_prompt("Select a device")
            .clear(true)
            .default(0)
            .items(&devices_id)
            .interact_on_opt(&term)
            .expect("Please select a device connected ton your computer")
            .unwrap();

        if selection != devices_id.len() - 1 {
            *device = devices[selection].clone();

            if match device.mode.as_str() {
                "device" => true,
                "recovery" => true,
                "bootloader" => true,
                "fastboot" => true,
                _ => false,
            } {
                break;
            }

            clear_stdout_up();
            clear_stdout_up();
            println!("❌ Please select an authorized device");
        } else {
            break;
        }
    }
    if !match device.mode.as_str() {
        "device" => true,
        "recovery" => true,
        "bootloader" => true,
        "fastboot" => true,
        _ => false,
    } {
        *device = Device {
            device_id: "None".to_string(),
            mode: "None".to_string(),
        };
    }
}
