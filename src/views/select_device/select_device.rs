use adbridger::struct_def::Device;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};

pub fn select_device(theme: &ColorfulTheme, term: &Term, device: &mut Device) {
    let devices =
        adbridger::device::list_devices().expect("Impossible de recuperer la liste des appareille");

    let devices_id: Vec<String> = devices
        .iter()
        .map(|device| device.device_id.clone())
        .collect();

    let selection = Select::with_theme(theme)
        .with_prompt("Select a device")
        .default(0)
        .items(&devices_id)
        .interact_on_opt(&term)
        .expect("Please select a device connected ton your computer")
        .unwrap();

    *device = devices[selection].clone();
    return;
}
