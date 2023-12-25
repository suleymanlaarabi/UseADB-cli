use adbridger::struct_def::Device;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};

use crate::views;

pub fn useadb(device: &Device, theme: &ColorfulTheme, term: &Term) {
    if device.device_id == "None" {
        return;
    }
    let selection = Select::with_theme(theme)
        .with_prompt("Choissiez une option")
        .default(0)
        .items(&["display device screen"])
        .interact_on_opt(term)
        .expect("Please select a valid option");

    match selection {
        Some(index) => match index {
            0 => views::screen(device),
            _ => {
                println!("Please select a valid option");
            }
        },
        None => {
            println!("Please select a valid option");
        }
    }
}
