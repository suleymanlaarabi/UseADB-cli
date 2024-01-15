use adbridger::struct_def::Device;
use console::Term;
use dialoguer::theme::ColorfulTheme;

use crate::utils::use_menu;

pub fn display_settings_view(device: &Device, theme: &ColorfulTheme, term: &Term) -> bool {
    let menu = vec!["quality"];
    let option_selected = use_menu(theme, term, &menu);

    match menu[option_selected] {
        "quality" => {
            println!("quality");
            true
        }
        _ => false,
    }
}
