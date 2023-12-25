use core::fmt;

pub enum HomePages {
    SelectDevice,
    UseAdbPage,
    ExitPage,
    // Ajoutez d'autres pages ici si nécessaire
}

impl fmt::Display for HomePages {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            HomePages::SelectDevice => "Select Device",
            HomePages::UseAdbPage => "Use ADB",
            HomePages::ExitPage => "Exit",
            // Complétez pour d'autres pages
        };
        write!(f, "{}", description)
    }
}
