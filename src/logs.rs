// utils/src/logs.rs
// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Canmi

use chrono::Local;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::io::Write;
use std::sync::Mutex;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Error, // 1
    Warn,  // 2
    Info,  // 3
    Debug, // 4
}

impl LogLevel {
    fn as_u8(&self) -> u8 {
        match self {
            LogLevel::Error => 1,
            LogLevel::Warn => 2,
            LogLevel::Info => 3,
            LogLevel::Debug => 4,
        }
    }
}

static LOG_LEVEL: Lazy<Mutex<LogLevel>> = Lazy::new(|| Mutex::new(LogLevel::Info));

pub fn set_log_level(new_level: LogLevel) {
    let mut level = LOG_LEVEL.lock().unwrap();
    *level = new_level;
}

pub fn log(level: LogLevel, content: &str) {
    let current_level = LOG_LEVEL.lock().unwrap();
    if level.as_u8() > current_level.as_u8() {
        return;
    }

    let mut stream = match level {
        LogLevel::Warn | LogLevel::Error => StandardStream::stderr(ColorChoice::Auto),
        _ => StandardStream::stdout(ColorChoice::Auto),
    };

    let now = Local::now().format("%H:%M:%S").to_string();

    match level {
        LogLevel::Info => {
            let _ = stream.set_color(ColorSpec::new().set_fg(Some(Color::White)));
            let _ = write!(stream, "{} ", now);
            let _ = stream.reset();
            let _ = writeln!(stream, "{}", content);
        }
        LogLevel::Debug => {
            let _ = stream.set_color(ColorSpec::new().set_fg(Some(Color::Blue)));
            let _ = writeln!(stream, "{} {}", now, content);
        }
        LogLevel::Warn => {
            let _ = stream.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
            let _ = writeln!(stream, "{} {}", now, content);
        }
        LogLevel::Error => {
            let _ = stream.set_color(ColorSpec::new().set_fg(Some(Color::Red)));
            let _ = writeln!(stream, "{} {}", now, content);
        }
    }

    let _ = stream.reset();
}