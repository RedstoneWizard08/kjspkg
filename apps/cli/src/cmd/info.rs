use crate::ctx::CliContext;
use anyhow::Result;
use colored::Colorize;

pub async fn cmd_info(cx: &CliContext, package: String, json: bool) -> Result<()> {
    let pkg_api = cx.api.package(package);
    let pkg = pkg_api.get().await?;

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
