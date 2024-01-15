use adbridger::device;
use adbridger::struct_def::{Device, RebootMode};
use console::Term;
use dialoguer::theme::ColorfulTheme;

use crate::utils::use_menu;
pub fn reboot(device: &Device, theme: &ColorfulTheme, term: &Term) -> bool {
    let menu = vec!["recovery", "system", "go back"];
    let option_selected = use_menu(theme, term, &menu);

    match menu[option_selected] {
        "recovery" => {
            device::reboot(device, RebootMode::Recovery).expect("Unable to reboot");
            true
        }
        "system" => {
            device::reboot(device, RebootMode::System).expect("Unable to reboot");
            true
        }
        _ => false,
    }
}
