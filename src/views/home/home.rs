use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::{io::stdout, ops::Add};

use adbridger::struct_def::Device;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Select};

use crate::views::{self, home::HomePages};

pub fn home(device: &mut Device, term: &Term, theme: &ColorfulTheme) -> bool {
    let mut stdout = stdout();

    stdout
        .execute(Clear(ClearType::All))
        .expect("Unable to clear console");

    stdout
        .execute(MoveTo(0, 0))
        .expect("Unable to replace console cursor");

    let app_title: console::StyledObject<&str> = style("UseADB - cli").bold().underlined().green();
    println!("\n{}", app_title);

    let title = String::from("Device selected: ").add(&device.device_id);
    let title_styled: console::StyledObject<&str> =
        style(title.as_str()).bold().underlined().green();

    println!("\n{}\n", title_styled);

    let select_device_page = HomePages::SelectDevice;
    let use_adb_page: HomePages = HomePages::UseAdbPage;
    let exit_page = HomePages::ExitPage;

    let mut menu = vec![select_device_page, use_adb_page, exit_page];

    if device.device_id == "None" {
        menu.remove(1);
    }

    let selection = Select::with_theme(theme)
        .with_prompt("Select an option")
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
