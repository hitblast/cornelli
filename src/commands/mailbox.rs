use std::{collections::HashMap, time::Duration};

use async_trait::async_trait;
use clap::Args;
use tokio::{process::Command, time::sleep};

use crate::{
    commands::Runnable, core::ChristmasDB, log_letter, log_orb, log_sparkles,
    utils::confirm::confirm_action,
};
use anyhow::Result;

#[derive(Debug, Args)]
pub struct MailboxCmd;

async fn clear_terminal() {
    #[cfg(windows)]
    Command::new("cmd").args(["/C", "cls"]).status().await.ok();

    #[cfg(not(windows))]
    Command::new("clear").status().await.ok();
}

#[async_trait]
impl Runnable for MailboxCmd {
    async fn run(&self, db: &mut ChristmasDB) -> Result<()> {
        let mut pending: HashMap<usize, String> = HashMap::new();

        for capsule in db.capsules.iter() {
            if capsule.is_awaiting_decryption()?
                && let Ok((decrypted, index)) = db.decrypt(&capsule)
            {
                pending.insert(index, decrypted);
            }
        }

        // intro sequence below
        // pretty much christmas all over

        if pending.is_empty() {
            clear_terminal().await;
            sleep(Duration::from_secs(1)).await;
            log_sparkles!("Looks like your letters are still spread across the abyss...\n");
            sleep(Duration::from_secs(3)).await;
            clear_terminal().await;
        } else {
            clear_terminal().await;

            sleep(Duration::from_secs(3)).await;
            log_orb!("Oh...");
            sleep(Duration::from_secs(4)).await;
            clear_terminal().await;
            log_orb!("I see...");
            sleep(Duration::from_secs(4)).await;
            clear_terminal().await;
            log_orb!("Your mailbox has a few things...\n");
            sleep(Duration::from_secs(5)).await;

            if confirm_action("Do you want to proceed?") {
                sleep(Duration::from_secs(1)).await;
                clear_terminal().await;
                log_orb!("In we go~");
                sleep(Duration::from_secs(2)).await;

                clear_terminal().await;

                for (index, decrypted) in pending {
                    db.remove(index).await?;

                    sleep(Duration::from_secs(3)).await;
                    log_letter!("{decrypted}");
                    println!();
                    log_orb!("- Signed by you.");
                    println!();
                    sleep(Duration::from_secs(5)).await;

                    clear_terminal().await;
                }
            }
        }

        Ok(())
    }
}
