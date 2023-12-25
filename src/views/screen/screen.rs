use adbridger::struct_def::Device;

pub fn screen(device: &Device) {
    adbridger::screen::display_screen(device).expect("unable to record screen");
}
