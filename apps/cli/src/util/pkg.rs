use colored::Colorize;
use eyre::Result;
use indexmap::IndexMap;
use itertools::Itertools;
use kjspkg_api::models::PackageWithData;

use crate::{
    ctx::CliContext,
    util::{mul_char, pad_long_string},
};

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
