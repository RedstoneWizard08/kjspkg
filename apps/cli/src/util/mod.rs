use crate::ctx::CliContext;
use colored::Colorize;
use eyre::Result;
use indexmap::IndexMap;
use itertools::Itertools;
use kjspkg_api::models::PackageWithData;
use tracing::level_filters::LevelFilter;

pub mod versions;

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

pub async fn print_package(pkg: PackageWithData, cx: &CliContext) -> Result<()> {
    let latest = cx.api.package(pkg.id.to_string()).latest_version().await;
    let mut info = IndexMap::new();

    info.insert("Package Name", pkg.name);
    info.insert("Slug", pkg.slug.clone());

    info.insert(
        "Web URL",
        format!("{}/p/{}", cx.api.instance_url(), pkg.slug),
    );

    info.insert("Author", pkg.authors.first().unwrap().username.clone());

    if let Ok(latest) = latest {
        info.insert(
            "Latest Version",
            format!(
                "{} {}{}{}",
                latest.name,
                "(".dimmed(),
                latest.version_number.dimmed(),
                ")".dimmed()
            ),
        );
    } else {
        info.insert("Latest Version", format!("{}", "None".dimmed()));
    }

    let longest = info.keys().max_by_key(|v| v.len()).unwrap().to_string();
    let mut data_str = String::new();
    let prefix = "  | ";

    for (k, v) in info {
        data_str.push_str(&format!(
            "\n{}{}{} {}",
            prefix,
            k.purple(),
            mul_char(' ', longest.len() - k.len()),
            v.bold()
        ));
    }

    let padded_prefix = format!("{}    ", prefix);
    let padding = padded_prefix.len() * 2;

    data_str.push_str(&format!(
        "\n{}{}\n{}",
        prefix,
        "Description".purple(),
        pad_long_string(pkg.description, padding)
            .lines()
            .map(|v| format!("{}{}", padded_prefix, v))
            .join("\n")
    ));

    println!("\n{}\n", data_str);

    Ok(())
}
