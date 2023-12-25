use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::{io::stdout, ops::Add};

use adbridger::struct_def::Device;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Select};

use crate::views;

pub fn home(device: &mut Device, term: &Term, theme: &ColorfulTheme) {
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

    if device.device_id == "None" {
        let select_device_title: console::StyledObject<&str> =
            style("Please Select a device").bold().underlined().red();
        println!("{}\n", select_device_title)
    }

    let menu = vec!["Select device", "Use ADB"];

    let selection = Select::with_theme(theme)
        .with_prompt("Select an option")
        .default(0)
        .items(&menu)
        .interact_on_opt(&term);

    match selection {
        Ok(index) => match index {
            Some(option) => match option {
                0 => views::select_device::select_device(&theme, &term, device),
                1 => views::useadb(device, theme, term),
                _ => {}
            },
            None => println!("Please select an option"),
        },
        Err(_) => println!("Please select an option"),
    }
}
