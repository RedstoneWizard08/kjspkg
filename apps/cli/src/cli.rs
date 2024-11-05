use crate::{cmd::Commands, splash::AboutGetter, style::get_styles, AUTHORS};
use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use colored::Colorize;

#[derive(Debug, Clone, Parser)]
#[command(
    version,
    about = AboutGetter,
    long_about = None,
    author = AUTHORS,
    disable_version_flag = true,
    styles = get_styles(),
)]
pub struct Cli {
    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,

    /// Show the version
    #[arg(short = 'V', long, action)]
    pub version: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

impl Cli {
    pub async fn run(&self) -> Result<()> {
        if self.version {
            Self::print_version();
            return Ok(());
        }

        if let Some(_cmd) = &self.command {
            // TODO
        } else {
            Self::command().print_help()?;
        }

        Ok(())
    }

    pub fn print_version() {
        println!(
            "{} version {}",
            "KJSPKG".bold(),
            env!("CARGO_PKG_VERSION").bold()
        );
    }
}
