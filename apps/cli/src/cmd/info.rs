use std::time::Duration;

use crate::{ctx::CliContext, util::get_spinner_style};
use colored::Colorize;
use eyre::Result;
use indicatif::ProgressBar;

pub async fn cmd_info(cx: &CliContext, package: String, json: bool) -> Result<()> {
    let pb = ProgressBar::new_spinner().with_style(get_spinner_style());
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message("Fetching package info");
    let pkg_api = cx.api.package(package);
    pb.set_message("Fetching latest version");
    let pkg = pkg_api.get().await?;
    pb.finish_and_clear();

    if json {
        println!("{}", serde_json::to_string_pretty(&pkg)?);
        return Ok(());
    }

    let latest = pkg_api.latest_version().await?;

    println!("{}{}", "Package: ".bold(), pkg.name.bold().cyan());

    println!(
        "{}{}",
        "Created: ".bold(),
        pkg.created_at
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
            .bold()
            .cyan()
    );

    println!(
        "{}{}",
        "Last Updated: ".bold(),
        pkg.updated_at
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
            .bold()
            .cyan()
    );

    println!(
        "{}{} {}{}{}",
        "Latest Version: ".bold(),
        latest.name.bold().cyan(),
        "(".bold().dimmed(),
        latest.version_number.bold().dimmed(),
        ")".bold().dimmed()
    );

    println!(
        "{}{}",
        "Downloads: ".bold(),
        pkg.downloads.to_string().bold().cyan()
    );

    println!(
        "{}{}",
        "Views: ".bold(),
        pkg.views.to_string().bold().cyan()
    );

    Ok(())
}
