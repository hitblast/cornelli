use std::time::Duration;

use anyhow::{Context, Result, bail};
use chrono::Duration as ChrDuration;

// 2h32m6s => 2 * 60 * 60 + 32 * 60 + 6
pub fn parse_duration(duration: &str) -> Result<ChrDuration> {
    let mut total_seconds = 0;

    for part in duration.split('+') {
        let mut current_number = String::new();
        let mut part_seconds = 0;

        for c in part.chars() {
            if c.is_ascii_digit() || c == '.' {
                current_number.push(c);
            } else if c.is_whitespace() {
                continue;
            } else {
                let number: u64 = current_number
                    .parse()
                    .with_context(|| format!("Negative duration!"))?;
                current_number.clear();
                part_seconds += match c {
                    's' => number,
                    'm' => number * 60,
                    'h' => number * 3600,
                    'd' => number * 86400,
                    _ => bail!("Invalid time unit."),
                };
            }
        }

        if !current_number.is_empty() {
            bail!("Invalid duration format.")
        }

        total_seconds += part_seconds;
    }

    let dur_std = Duration::from_secs(total_seconds);
    let duration = ChrDuration::from_std(dur_std)?;

    Ok(duration)
}
