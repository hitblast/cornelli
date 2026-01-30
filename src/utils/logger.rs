// SPDX-License-Identifier: MIT OR Apache-2.0

//! logger module for cutler.
//!
//! Use the log_*! macros for pretty-printing text inside cutler.

use std::{
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

// ANSI color codes.
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

#[doc(hidden)]
#[derive(PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Orb,
    Sparkles,
    Letter,
}

#[doc(hidden)]
pub fn _print_log(level: LogLevel, msg: &str) {
    let tag = match level {
        LogLevel::Error => "Error!",
        LogLevel::Sparkles => "✨️",
        LogLevel::Orb => "🔮",
        LogLevel::Letter => "💌",
    };

    let line = if level == LogLevel::Error {
        format!("{RED}{tag}{RESET} {msg}")
    } else {
        format!("{tag} {msg}")
    };

    if level == LogLevel::Error {
        eprintln!("{line}");
    } else if level == LogLevel::Orb {
        type_out(&line.to_string(), 20, true);
    } else if level == LogLevel::Sparkles {
        type_out(&line.to_string(), 20, false);
    } else {
        type_out(&line.to_string(), 100, false);
    }
}

fn type_out(s: &str, ms: u64, dot: bool) {
    let mut out = io::stdout();
    let dur = Duration::from_millis(ms);

    for c in s.chars() {
        out.write_all(c.to_string().as_bytes()).unwrap_or_default();
        out.flush().unwrap_or_default();
        sleep(dur);
    }

    if dot {
        let dot_dur = dur * 4;
        for _ in 0..3 {
            write!(out, ".").ok();
            out.flush().ok();
            sleep(dot_dur);
            write!(out, ".").ok();
            out.flush().ok();
            sleep(dot_dur);
            write!(out, "\x08 \x08\x08 \x08").ok();
            out.flush().ok();
            sleep(dot_dur);
        }
    }

    println!()
}

/// Logs with `LogLevel::Error`.
#[macro_export]
macro_rules! log_err {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::utils::logger::_print_log($crate::utils::logger::LogLevel::Error, &msg);
    }};
}

/// Logs with `LogLevel::Sparkles`.
#[macro_export]
macro_rules! log_sparkles {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::utils::logger::_print_log($crate::utils::logger::LogLevel::Sparkles, &msg);
    }};
}

/// Logs with `LogLevel::Orb`.
#[macro_export]
macro_rules! log_orb {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::utils::logger::_print_log($crate::utils::logger::LogLevel::Orb, &msg);
    }};
}

/// Logs with `LogLevel::Letter`.
#[macro_export]
macro_rules! log_letter {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::utils::logger::_print_log($crate::utils::logger::LogLevel::Letter, &msg);
    }};
}
