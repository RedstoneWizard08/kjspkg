pub mod fetch;
pub mod info;
pub mod init;
pub mod install;
pub mod list;
pub mod list_remote;
pub mod search;
pub mod uninit;
pub mod uninstall;
pub mod update;
pub mod update_all;

use anyhow::Result;
use clap::Subcommand;
use fetch::cmd_fetch;
use info::cmd_info;
use init::{cmd_init, ModLoader};
use install::cmd_install;
use list::{cmd_list, ListOutputFormat};
use list_remote::cmd_list_remote;
use search::cmd_search;
use uninit::cmd_uninit;
use uninstall::cmd_uninstall;
use update::cmd_update;
use update_all::cmd_update_all;

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    /// Install a package.
    #[command(visible_aliases = ["add", "download", "i"])]
    Install {
        /// The packages to install.
        #[arg(required = true)]
        packages: Vec<String>,

        /// Skip missing packages instead of throwing an error.
        #[arg(short, long, action)]
        skip_missing: bool,

        /// Update packages instead of skipping them if they are already installed.
        #[arg(short, long, action)]
        update: bool,
    },

    /// Uninstall a package.
    #[command(visible_aliases = ["remove", "rm"])]
    Uninstall {
        /// The packages to uninstall.
        #[arg(required = true)]
        packages: Vec<String>,

        /// Skip missing packages instead of throwing an error.
        #[arg(short, long, action)]
        skip_missing: bool,
    },

    /// Update packages.
    Update {
        /// The packages to update.
        #[arg(required = true)]
        packages: Vec<String>,

        /// Skip missing packages instead of throwing an error.
        #[arg(short, long, action)]
        skip_missing: bool,
    },

    /// Update all packages.
    #[command(visible_aliases = ["up"])]
    UpdateAll,

    /// List packages.
    #[command(visible_aliases = ["ls"])]
    List {
        /// The format to output as.
        #[arg(short, long, value_enum, default_value_t = ListOutputFormat::Text)]
        format: ListOutputFormat,
    },

    /// Print info about a package.
    #[command(visible_aliases = ["pkg"])]
    Info {
        /// The package to print information about.
        #[arg(required = true)]
        package: String,

        /// Whether to output as JSON.
        #[arg(short, long, action)]
        json: bool,
    },

    /// Print a neofetch-esque screen showing information about KJSPKG.
    Fetch,

    /// List all remote packages.
    #[command(visible_aliases = ["ls-remote"])]
    ListRemote,

    /// Search for packages.
    Search {
        /// The string to search for.
        #[arg(required = true)]
        query: String,
    },

    /// Initialize a new project.
    #[command(visible_aliases = ["new"])]
    Init {
        /// The Minecraft version this project uses.
        /// If not supplied, it will be asked interactively.
        #[arg(short = 'M', long, value_enum)]
        minecraft: Option<String>,

        /// The loader this project uses.
        /// If not supplied, it will be asked interactively.
        #[arg(short = 'L', long, value_enum)]
        loader: Option<ModLoader>,

        /// Force creation, even if files already exist.
        #[arg(short, long, action)]
        force: bool,
    },

    /// De-initialize a project.
    #[command(visible_aliases = ["deinit"])]
    Uninit {
        /// Don't show the confirmation prompt.
        #[arg(short, long, action)]
        confirm: bool,
    },
}

impl Commands {
    pub async fn run(self) -> Result<()> {
        match self {
            Self::Install {
                packages,
                skip_missing,
                update,
            } => cmd_install(packages, skip_missing, update).await,
            Self::Uninstall {
                packages,
                skip_missing,
            } => cmd_uninstall(packages, skip_missing).await,
            Self::Update {
                packages,
                skip_missing,
            } => cmd_update(packages, skip_missing).await,
            Self::UpdateAll => cmd_update_all().await,
            Self::List { format } => cmd_list(format).await,
            Self::Info { package, json } => cmd_info(package, json).await,
            Self::Fetch => cmd_fetch().await,
            Self::ListRemote => cmd_list_remote().await,
            Self::Search { query } => cmd_search(query).await,
            Self::Init {
                minecraft,
                loader,
                force,
            } => cmd_init(minecraft, loader, force).await,
            Self::Uninit { confirm } => cmd_uninit(confirm).await,
        }
    }
}
