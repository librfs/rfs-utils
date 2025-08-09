// src/logs.rs
// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Canmi

use chrono::Local;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Debug,
    Warn,
    Error,
}

pub fn log(level: LogLevel, content: &str) {
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
