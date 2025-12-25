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
pub const RED: &str = "\x1b[31m";
pub const PINK: &str = "\x1b[35m";
pub const ORANGE: &str = "\x1b[38;5;208m";
pub const RESET: &str = "\x1b[0m";

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
    let (tag, color) = match level {
        LogLevel::Error => ("Error! ", RED),
        LogLevel::Sparkles => ("✨️ ", ""),
        LogLevel::Orb => ("🔮 ", ""),
        LogLevel::Letter => ("💌 ", ""),
    };

    let line = if level == LogLevel::Sparkles {
        format!("{tag} {msg}")
    } else {
        format!("{color}{tag}{RESET} {msg}")
    };

    if level == LogLevel::Error {
        eprintln!("{line}");
    } else if level == LogLevel::Orb || level == LogLevel::Sparkles {
        type_out(&format!("{line}"), 20);
    } else {
        type_out(&format!("{line}"), 120);
    }
}

fn type_out(s: &str, ms: u64) {
    let mut out = io::stdout();

    for c in s.chars() {
        out.write_all(c.to_string().as_bytes()).unwrap();
        out.flush().unwrap();
        sleep(Duration::from_millis(ms));
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
