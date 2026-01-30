use std::process::Command;

use dialoguer::Confirm;

/// Ask "Y/N?"; returns true if accept_all is set or the user types "y" or "Y"
#[must_use]
pub fn confirm_action(prompt: &str) -> bool {
    Confirm::new()
        .with_prompt(prompt)
        .interact()
        .unwrap_or_default()
}

pub fn clear_terminal() {
    #[cfg(windows)]
    Command::new("cmd").args(["/C", "cls"]).status().ok();

    #[cfg(not(windows))]
    Command::new("clear").status().ok();
}
