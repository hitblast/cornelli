use anyhow::bail;
use nix::unistd::Uid;

/// Only run the command if the process is running as non-root.
pub fn run_with_noroot() -> Result<(), anyhow::Error> {
    if Uid::effective().is_root() {
        bail!("Do not use sudo on this command!");
    }

    Ok(())
}
