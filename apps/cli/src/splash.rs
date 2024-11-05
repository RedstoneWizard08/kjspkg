use clap::builder::{IntoResettable, Resettable, StyledStr};
use colored::Colorize;
use rand::{seq::SliceRandom, thread_rng};

pub const SPLASH_TEXTS: &[&str] = &[
    "You should run `kjspkg uninit`, NOW!",
    "Run `kjspkg mold` to brew kombucha",
    "Thanks Lat 👍",
    "Help, I'm locked in a basement packaging scripts!",
    "kjspkg rm -rf / --no-preserve-root",
    "Made in Python 3.whatever!",
    "https://modernmodpacks.site",
    "Made by Modern Modpacks!",
    "gimme gimme gimme",
    "`amogus` is a real package!",
    "Supports 1.12!",
    "Procrastinating doing one project by doing another project, genius!",
    "Also try Magna!",
    "No alternative for CraftTweaker!",
];

pub struct AboutGetter;

impl IntoResettable<StyledStr> for AboutGetter {
    fn into_resettable(self) -> Resettable<StyledStr> {
        let splash = SPLASH_TEXTS
            .choose(&mut thread_rng())
            .unwrap_or(&SPLASH_TEXTS[0]);

        Resettable::Value(
            format!(
                "{}, {}\n{}",
                "KJSPKG".bold().bright_blue().underline(),
                "a modern package manager for KubeJS.".bright_blue(),
                splash.bold().cyan().italic()
            )
            .into(),
        )
    }
}
