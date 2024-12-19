use crate::{tags::tags, vers::get_astro_versions};
use anyhow::Result;
use clap::{Command, CommandFactory, Parser};
use clap_complete::{generate, Generator, Shell};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use modhost::{from_log_level, init_logger, loaders, ModHost};
use std::io::stdout;

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,

    #[arg(short = 'C', long)]
    pub complete: Option<Shell>,
}

impl Cli {
    fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
        generate(gen, cmd, cmd.get_name().to_string(), &mut stdout());
    }

    pub async fn run(self) -> Result<()> {
        if let Some(shell) = self.complete {
            Self::print_completions(shell, &mut Cli::command());
            return Ok(());
        }

        let _ = dotenvy::dotenv();
        init_logger(from_log_level(self.verbose.log_level_filter()));

        ModHost::new(Box::new(|_| true))
            .await?
            .versions(get_astro_versions().await?)
            .loaders(loaders!["AstroModIntegrator", "UE4SS"])
            .tags(tags())
            .router()
            .run()
            .await?;

        Ok(())
    }
}
