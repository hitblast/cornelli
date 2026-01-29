use std::process::Command;

use anyhow::Result;
use dialoguer::Confirm;
use rpassword::prompt_password;

use crate::log_orb;

/// Ask "Y/N?"; returns true if accept_all is set or the user types "y" or "Y"
#[must_use]
pub fn confirm_action(prompt: &str) -> bool {
    Confirm::new()
        .with_prompt(prompt)
        .interact()
        .unwrap_or_default()
}

pub fn get_string_input(prompt: &str) -> Result<String> {
    log_orb!("{prompt}");
    let password = prompt_password("Put here (invisible): ")?;
    Ok(password)
}

pub fn clear_terminal() {
    #[cfg(windows)]
    Command::new("cmd").args(["/C", "cls"]).status().ok();

    #[cfg(not(windows))]
    Command::new("clear").status().ok();
}
