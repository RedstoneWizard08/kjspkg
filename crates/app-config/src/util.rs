use app_core::Result;
use config::{Config, Environment, File, FileFormat};

use crate::AppConfig;

pub fn get_raw_config() -> Result<Config> {
    Ok(Config::builder()
        .add_source(File::with_name("ModHost").format(FileFormat::Toml))
        .add_source(
            Environment::with_prefix("MODHOST")
                .try_parsing(true)
                .separator("_")
                .list_separator(","),
        )
        .build()?)
}

fn get_config_internal() -> Result<AppConfig> {
    Ok(get_raw_config()?.try_deserialize::<AppConfig>()?)
}

pub fn get_config() -> Result<AppConfig> {
    let config = get_config_internal().unwrap_or_default();

    config.save()?;

    Ok(config)
}
