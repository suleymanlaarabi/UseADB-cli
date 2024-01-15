use std::process::exit;

use adbridger::{screen::display_screen, struct_def::Device};
use console::Term;
use dialoguer::theme::ColorfulTheme;

use crate::utils::use_menu;

use super::views::display_settings_view;

pub fn screen(device: &Device, theme: &ColorfulTheme, term: &Term) {
    let menu = vec!["settings", "start"];

    let option_selected = use_menu(theme, term, &menu);

    match menu[option_selected] {
        "settings" => {
            display_settings_view(device, theme, term);
        }
        "start" => {
            display_screen(device)
                .expect("Unable to display screen have you installed required dependency ?");
        }
        _ => exit(1),
    }
}
