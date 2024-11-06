use crate::{ctx::CliContext, manifest::ProjectManifest};
use clap::ValueEnum;
use color_eyre::Section;
use eyre::{eyre, Result};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, ValueEnum, Default)]
pub enum ListOutputFormat {
    #[clap(aliases = ["md"])]
    Markdown,
    HTML,
    #[default]
    #[clap(aliases = ["txt"])]
    Text,
}

pub async fn cmd_list(cx: &CliContext, format: ListOutputFormat) -> Result<()> {
    if !PathBuf::from("project.json").exists() {
        return Err(
            eyre!("project.json does not exist!").suggestion("Maybe try running `kjspkg init`?")
        );
    }

    let data = ProjectManifest::read(None)?;
    let mut out = Vec::new();

    for (_, info) in data.packages {
        let data = cx.api.package(&info.name).get().await?;
        let name = format!("{} @ {}", data.name, info.version);

        let url = format!(
            "{}/p/{}/v/{}",
            cx.api.instance_url(),
            info.name,
            info.version
        );

        match format {
            ListOutputFormat::HTML => out.push(format!("<a href=\"{}\">{}</a>", url, name)),
            ListOutputFormat::Markdown => out.push(format!("[{}]({})", name, url)),
            ListOutputFormat::Text => out.push(format!("{} - {}", name, url)),
        };
    }

    println!("{}", out.join("\n"));

    Ok(())
}
