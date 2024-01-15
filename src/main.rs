use adbridger::struct_def::Device;
use clap::App;
use console::Term;
use dialoguer::theme::ColorfulTheme;

pub mod utils;
mod views;

fn main() {
    let term = Term::stdout();

    let mut device: Device = Device {
        device_id: "None".to_string(),
        mode: "None".to_string(),
    };

    let theme = &ColorfulTheme::default();

    let _ = App::new("Use ADB")
        .version("0.1.0")
        .author("Laarabi Suleyman")
        .about("A simple cli to use Android debug bridge");

    loop {
        if !views::home(&mut device, &term, theme) {
            break;
        }
    }
}
