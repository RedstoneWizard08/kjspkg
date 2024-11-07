pub mod init;
pub mod pack;
pub mod publish;

use crate::{ctx::CliContext, manifest::ModLoader};
use clap::Subcommand;
use eyre::Result;
use init::cmd_init;
use pack::cmd_pack;
use publish::cmd_publish;

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    /// Initialize a new package.
    Init {
        /// The package name.
        name: Option<String>,

        /// The package version.
        #[arg(short = 's', long)]
        version: Option<String>,

        /// A short description of the package.
        #[arg(short, long)]
        description: Option<String>,

        /// A list of authors credited in this project.
        #[arg(short, long)]
        authors: Option<Vec<String>>,

        /// A list of minecraft versions for the package to support.
        #[arg(short = 'M', long, use_value_delimiter = true, value_delimiter = ',')]
        minecraft: Option<Vec<String>>,

        /// A list of loaders for the package to support.
        #[arg(short = 'L', long, use_value_delimiter = true, value_delimiter = ',')]
        loaders: Option<Vec<ModLoader>>,

        /// Whether to force creation even if the directory is not empty.
        #[arg(short, long, action)]
        force: bool,
    },

    /// Pack the package into a .tgz file.
    Pack {
        /// Whether to overwrite any existing packed archive.
        #[arg(short, long, action)]
        overwrite: bool,
    },

    /// Publish the package to the KJSPKG registry.
    #[clap(visible_aliases = ["push"])]
    Publish {
        /// Whether to overwrite any existing packed archive.
        #[arg(short, long, action)]
        force: bool,
    },
}

impl Commands {
    pub async fn run(self, cx: &CliContext) -> Result<()> {
        match self {
            Self::Init {
                minecraft,
                loaders,
                name,
                version,
                authors,
                description,
                force,
            } => {
                cmd_init(
                    cx,
                    minecraft,
                    loaders,
                    name,
                    version,
                    authors,
                    description,
                    force,
                )
                .await
            }

            Self::Pack { overwrite } => cmd_pack(cx, overwrite).await.map(|_| ()),
            Self::Publish { force } => cmd_publish(cx, force).await,
        }
    }
}
