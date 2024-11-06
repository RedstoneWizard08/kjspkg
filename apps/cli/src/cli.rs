use crate::{
    cmd::Commands, ctx::get_ctx, splash::AboutGetter, style::get_styles, util::from_log_level,
    AUTHORS,
};
use clap::{CommandFactory, Parser};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use colored::Colorize;
use eyre::Result;

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
    pub async fn run(self) -> Result<()> {
        tracing_subscriber::fmt::fmt()
            .compact()
            .without_time()
            .with_file(false)
            .with_target(false)
            .with_max_level(from_log_level(self.verbose.log_level_filter()))
            .init();

        if self.version {
            Self::print_version();
            return Ok(());
        }

        if let Some(cmd) = self.command {
            let cx = get_ctx()?;

            cmd.run(&cx).await?;
            cx.save()?;
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
