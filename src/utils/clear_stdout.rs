use std::io::stdout;

use crossterm::{
    cursor::{MoveTo, MoveToPreviousLine},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};

pub fn clear_stdout() {
    let mut stdout = stdout();

    stdout
        .execute(Clear(ClearType::All))
        .expect("Unable to clear console");

    stdout
        .execute(MoveTo(0, 0))
        .expect("Unable to replace console cursor");
}

pub fn clear_stdout_up() {
    let mut stdout = stdout();

    stdout
        .execute(MoveToPreviousLine(1))
        .expect("Unable to clear up line");

    stdout
        .execute(Clear(ClearType::CurrentLine))
        .expect("Unable to clear up line");
}
