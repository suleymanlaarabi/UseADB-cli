use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};

pub fn use_menu(theme: &ColorfulTheme, term: &Term, menu: &Vec<&str>) -> usize {
    let selection = Select::with_theme(theme)
        .with_prompt("Select:")
        .default(0)
        .items(menu)
        .interact_on_opt(term);
    return selection.expect("Unable to get response").unwrap();
}
