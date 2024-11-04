//! The logging stuff.

use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt, layer::SubscriberExt, registry, util::SubscriberInitExt, EnvFilter};

/// Initializes the logger.
pub fn init_logger() {
    let mut filter = EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into());

    filter = filter.add_directive("tokio_postgres::connection=warn".parse().unwrap());
    filter = filter.add_directive("tokio_postgres::query=warn".parse().unwrap());
    filter = filter.add_directive("diesel_async_migrations=warn".parse().unwrap());
    filter = filter.add_directive("tokio_util::codec::framed_impl=warn".parse().unwrap());
    filter = filter.add_directive("tokio_tungstenite=warn".parse().unwrap());
    filter = filter.add_directive("want=warn".parse().unwrap());
    filter = filter.add_directive("tungstenite=warn".parse().unwrap());
    filter = filter.add_directive("arboard=warn".parse().unwrap());

    let layer = fmt::layer()
        .pretty()
        .compact()
        .with_ansi(true)
        .with_level(true)
        .with_target(true)
        .with_file(false)
        .with_line_number(false)
        .without_time();

    registry().with(filter).with(layer).init();
}
