use adbridger::struct_def::Device;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};

use crate::views;

pub fn useadb(device: &mut Device, theme: &ColorfulTheme, term: &Term) {
    if device.device_id == "None" {
        return;
    }
    let menu = ["display device screen", "reboot", "go back"];
    let selection = Select::with_theme(theme)
        .with_prompt("Use ADB for:")
        .default(0)
        .items(&menu)
        .interact_on_opt(term)
        .expect("Please select a valid option");

    match selection {
        Some(index) => match index {
            0 => views::screen(device, theme, term),
            1 => {
                if views::useadb::reboot(device, theme, term) {
                    *device = Device {
                        device_id: "None".to_string(),
                        mode: "None".to_string(),
                    }
                }
            }
            _ => {
                return;
            }
        },
        None => {
            println!("Please select a valid option");
        }
    }
}
