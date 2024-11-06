use crate::{ctx::CliContext, manifest::ProjectManifest, util::mul_char};
use color_eyre::Section;
use colored::Colorize;
use eyre::{eyre, Result};
use indexmap::IndexMap;
use itertools::{EitherOrBoth, Itertools};
use std::{env::current_dir, path::PathBuf};

pub const LOGO: &str = "
⠀⠀⠀⠀⢀⣤⣶⣿⣿⣶⣤⡀⠀⠀⠀⠀
⠀⠀⣴⣾⣿⣿⣿⣿⣿⣿⣿⣿⣷⣦⠀⠀
⢠⣄⡀⠉⠻⢿⣿⣿⣿⣿⡿⠟⠉⢀⣠⡄
⢸⣿⣿⣷⣦⣀⠈⠙⠋⠁⣀⣴⣾⣿⣿⡇
⢸⣿⣿⣿⣿⣿⣿⠀⠀⣿⣿⣿⣿⣿⣿⡇
⢸⣿⣿⣿⣿⣿⣿⠀⠀⣿⣿⣿⣿⣿⣿⡇
⠀⠙⠻⣿⣿⣿⣿⠀⠀⣿⣿⣿⣿⠟⠋⠀
⠀⠀⠀⠀⠉⠻⢿⠀⠀⡿⠟⠉⠀⠀⠀⠀
";

pub async fn cmd_fetch(_cx: &CliContext) -> Result<()> {
    if !PathBuf::from("project.json").exists() {
        return Err(
            eyre!("project.json does not exist!").suggestion("Maybe try running `kjspkg init`?")
        );
    }

    let proj = ProjectManifest::read(None)?;
    let mut data = IndexMap::new();

    data.insert("KJSPKG Version", env!("CARGO_PKG_VERSION").into());
    data.insert("Minecraft Version", proj.minecraft);
    data.insert("Mod Loader", proj.loader.into());
    data.insert("Installed Packages", proj.packages.len().to_string());

    let header = format!("KJSPKG @ {}", current_dir()?.to_str().unwrap());
    let longest = data.keys().max_by_key(|v| v.len()).unwrap().to_string();
    let mut data_str = String::new();

    data_str.push_str(&format!("{}", header.bold().underline().purple()));

    for (k, v) in data {
        data_str.push_str(&format!(
            "\n{}{} {}",
            k.purple(),
            mul_char(' ', longest.len() - k.len()),
            v.bold()
        ));
    }

    println!();

    for it in LOGO.lines().collect_vec()[1..]
        .iter()
        .zip_longest(data_str.lines())
    {
        match it {
            EitherOrBoth::Both(logo, data) => {
                println!("{}    {}", logo.purple(), data);
            }

            EitherOrBoth::Left(logo) => {
                println!("{}    ", logo.purple());
            }

            EitherOrBoth::Right(data) => {
                println!("{}", data);
            }
        }
    }

    println!();

    Ok(())
}
