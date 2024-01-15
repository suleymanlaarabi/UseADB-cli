use std::ops::Add;

use adbridger::struct_def::Device;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Select};

use crate::{
    utils::clear_stdout,
    views::{self, home::HomePages},
};

pub fn home(device: &mut Device, term: &Term, theme: &ColorfulTheme) -> bool {
    let mut menu: Vec<HomePages> = vec![
        HomePages::SelectDevice,
        HomePages::UseAdbPage,
        HomePages::ExitPage,
    ];

    clear_stdout();

    let app_title: console::StyledObject<&str> = style("UseADB - cli").bold().underlined().green();
    println!("\n{}", app_title);

    let title = String::from("Device selected: ").add(&device.device_id);
    let title_styled: console::StyledObject<&str> = style(title.as_str()).bold().underlined();

    if device.device_id == "None" {
        menu.remove(1);
        println!("\n{}\n", title_styled.red());
    } else {
        let battery_level = adbridger::device::get_battery_info(device)
            .expect("Unable to get device battery info")
            .battery_level;
        println!("\n{}  🔋: {}%\n", title_styled.green(), battery_level);
    }

    let selection = Select::with_theme(theme)
        .with_prompt("Options: ")
        .default(0)
        .items(&menu)
        .interact_on_opt(&term);

    match selection {
        Ok(index) => match index {
            Some(option) => {
                if menu[option].to_string() == HomePages::ExitPage.to_string() {
                    return false;
                }

                match option {
                    0 => {
                        views::select_device::select_device(&theme, &term, device);
                        true
                    }
                    1 => {
                        views::useadb(device, theme, term);
                        true
                    }
                    _ => false,
                }
            }
            None => {
                println!("Please select an option");
                true
            }
        },
        Err(_) => {
            println!("Please select an option");
            true
        }
    }
}
