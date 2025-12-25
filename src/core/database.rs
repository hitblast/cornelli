use std::path::PathBuf;

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
use tokio::fs;

type Aes256Ctr = ctr::Ctr64BE<Aes256>;

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Capsule {
    data: Vec<u8>,   // ciphertext
    nonce: [u8; 16], // CTR nonce (must match for decrypt)
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
    pub async fn load_from_pass(password: String, path: PathBuf) -> Result<Self> {
        let mut key = [0u8; 32];
        pbkdf2_hmac::<Sha256>(password.as_bytes(), b"tasty salt", 100, &mut key);

        let capsules = if path.try_exists()? {
            let data = fs::read_to_string(&path).await?;
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

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn list_capsules(&self) -> &[Capsule] {
        &self.capsules
    }

    async fn autosave(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(self).context("Failed to serialize DB.")?;
        let parent = self
            .path
            .parent()
            .with_context(|| format!("Cannot create parent directories."))?;
        fs::create_dir_all(parent).await?;
        fs::write(&self.path, json).await?;
        Ok(())
    }

    pub async fn add_new_capsule(
        &mut self,
        text: String,
        should_be_kept_for: Duration,
    ) -> Result<()> {
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

        self.autosave().await?;
        Ok(())
    }

    pub async fn decrypt(&mut self, cap: Capsule) -> Result<String> {
        if !cap.is_awaiting_decryption()? {
            bail!("Cannot decrypt before the time happens.");
        }

        let mut data = cap.data.clone();
        let mut cipher = Aes256Ctr::new(&self.key.into(), &cap.nonce.into());
        cipher.apply_keystream(&mut data);

        let text = String::from_utf8(data).context("Decryption produced invalid UTF-8.")?;

        let idx = self
            .capsules
            .iter()
            .position(|x| *x == cap)
            .context("Capsule not found!")?;

        self.capsules.remove(idx);
        self.autosave().await?;

        Ok(text)
    }
}
