use std::path::PathBuf;

use anyhow::{Result, bail};

pub fn get_database_path() -> Result<PathBuf> {
    let db_path = match dirs::config_dir() {
        Some(path) => path.join("cornelli/christmas.json"),
        None => {
            bail!("Storage couldn't be determined.");
        }
    };

    Ok(db_path)
}
