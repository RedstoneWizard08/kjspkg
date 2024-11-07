use indicatif::ProgressStyle;
use once_cell::sync::Lazy;
use regex::Regex;
use tracing::level_filters::LevelFilter;

pub mod mc;
pub mod pkg;
pub mod tar;
pub mod versions;

pub const SLUG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)[^a-z0-9_-]").unwrap());

pub fn mul_char(ch: char, n: usize) -> String {
    let mut s = String::new();

    for _ in 0..n {
        s.push(ch);
    }

    s
}

pub fn from_log_level(level: log::LevelFilter) -> LevelFilter {
    match level {
        log::LevelFilter::Debug => LevelFilter::DEBUG,
        log::LevelFilter::Error => LevelFilter::ERROR,
        log::LevelFilter::Info => LevelFilter::INFO,
        log::LevelFilter::Off => LevelFilter::OFF,
        log::LevelFilter::Trace => LevelFilter::TRACE,
        log::LevelFilter::Warn => LevelFilter::WARN,
    }
}

pub fn pad_long_string(s: String, padding: usize) -> String {
    let mut out = String::new();
    let mut buf = String::new();
    let term_width = termsize::get().unwrap().cols as usize;
    let max_length = term_width - padding;

    for ch in s.chars() {
        if buf.len() >= max_length {
            out.push_str(&buf);
            out.push('\n');
            buf = String::new();
        }

        buf.push(ch);
    }

    out.push_str(&buf);
    out
}

pub fn parse_pkg_input(input: impl AsRef<str>) -> (String, Option<String>) {
    let mut data = input.as_ref().split("@");

    (data.next().unwrap().into(), data.next().map(|v| v.into()))
}

pub fn create_slug(input: impl AsRef<str>) -> String {
    SLUG_REGEX.replace_all(input.as_ref(), "-").to_string()
}

pub fn get_spinner_style() -> ProgressStyle {
    ProgressStyle::default_spinner()
        .tick_strings(&[
            "[    ]", "[=   ]", "[==  ]", "[=== ]", "[====]", "[ ===]", "[  ==]", "[   =]",
            "[    ]", "[   =]", "[  ==]", "[ ===]", "[====]", "[=== ]", "[==  ]", "[=   ]",
        ])
        .template("{spinner:.blue.bold} {msg:.cyan.bold}")
        .unwrap()
}

pub fn get_bar_style() -> ProgressStyle {
    ProgressStyle::default_bar()
        .progress_chars("=- ")
        .template("[{pos:.green}/{len:.blue}] [{bar:40.blue/cyan.dimmed}] {msg:.cyan.bold}")
        .unwrap()
}
