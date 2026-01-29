use std::{fs, path::PathBuf};

use aes::{
    Aes256,
    cipher::{KeyIvInit, StreamCipher},
};
use anyhow::{Context, Result, bail};
use chrono::{Duration, Local, NaiveDateTime};
use pbkdf2::pbkdf2_hmac;
use rand::{TryRngCore, rngs::OsRng};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::utils::path::get_database_path;

type Aes256Ctr = ctr::Ctr64BE<Aes256>;

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Capsule {
    data: Vec<u8>,
    nonce: [u8; 16],
    should_be_kept_for: Duration,
    time_added: NaiveDateTime,
}

impl Capsule {
    pub fn is_awaiting_decryption(&self) -> Result<bool> {
        if let Some(future) = self.time_added.checked_add_signed(self.should_be_kept_for) {
            Ok(future < Local::now().naive_local())
        } else {
            bail!("Duration overflow when computing unlock time.")
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ChristmasDB {
    pub capsules: Vec<Capsule>,
    #[serde(skip)]
    key: [u8; 32],
    #[serde(skip)]
    path: PathBuf,
}

impl ChristmasDB {
    /// Initialize a ChristmasDB instance and load/save database/passwords.
    pub fn init(password: String) -> Result<Self> {
        let path = get_database_path()?;

        let mut key = [0u8; 32];
        pbkdf2_hmac::<Sha256>(
            password.as_bytes(),
            b"tasty salt",
            password.len() as u32,
            &mut key,
        );

        let capsules = if path.try_exists()? {
            let data = fs::read_to_string(&path)?;
            let parsed: Self = serde_json::from_str(&data)?;
            parsed.capsules
        } else {
            Vec::new()
        };

        Ok(Self {
            capsules,
            key,
            path,
        })
    }

    /// Autosaves current ChristmasDB data to the given path.
    fn autosave(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(self).context("Failed to serialize DB.")?;
        let parent = self
            .path
            .parent()
            .with_context(|| "Cannot create parent directories.".to_string())?;
        fs::create_dir_all(parent)?;
        fs::write(&self.path, json)?;
        Ok(())
    }

    /// Returns the path of the database instance.
    #[must_use]
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Returns a reference vector to all capsules.
    #[must_use]
    pub fn list_capsules(&self) -> &[Capsule] {
        &self.capsules
    }

    /// Ciphers a given text and adds to the capsule.
    pub fn add_new_capsule(&mut self, text: String, should_be_kept_for: Duration) -> Result<()> {
        let mut data = text.into_bytes();
        let mut nonce = [0u8; 16];
        let mut rng = OsRng;
        rng.try_fill_bytes(&mut nonce)?;

        let mut cipher = Aes256Ctr::new(&self.key.into(), &nonce.into());
        cipher.apply_keystream(&mut data);

        self.capsules.push(Capsule {
            data,
            nonce,
            should_be_kept_for,
            time_added: Local::now().naive_local(),
        });

        self.autosave()?;
        Ok(())
    }

    /// Non-invasive capsule deciphering. Use `.remove()` to remove the capsule by index.
    ///
    /// Returns the decrypted text and the index of the capsule at the time of the removal.
    pub fn decrypt(&self, cap: &Capsule) -> Result<(String, usize)> {
        let mut data = cap.data.clone();
        let mut cipher = Aes256Ctr::new(&self.key.into(), &cap.nonce.into());
        cipher.apply_keystream(&mut data);

        let text = String::from_utf8(data).context("Invalid UTF-8, possibly a faulty password?")?;

        let idx = self
            .capsules
            .iter()
            .position(|x| x == cap)
            .context("Capsule not found! Did you delete it manually? :suspicious_eyes:")?;

        Ok((text, idx))
    }

    /// Removes a capsule from a given index.
    pub fn remove(&mut self, idx: usize) -> Result<()> {
        self.capsules.remove(idx);
        self.autosave()?;

        Ok(())
    }

    /// Removes the entire database instance from memory.
    pub fn delete(&self) -> Result<()> {
        if let Some(dir) = self.path.parent() {
            fs::remove_dir_all(dir)?;
        }

        Ok(())
    }
}
