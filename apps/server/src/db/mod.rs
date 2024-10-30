pub mod pkg;
pub mod user;
pub mod ver;

use crate::Result;
use diesel_async::{
    pooled_connection::{
        deadpool::{Object, Pool},
        AsyncDieselConnectionManager,
    },
    AsyncPgConnection,
};
use diesel_async_migrations::{embed_migrations, EmbeddedMigrations};
use std::env;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
pub type DbPool = Pool<AsyncPgConnection>;
pub type DbConn = Object<AsyncPgConnection>;

pub async fn create_connection(db_url: Option<String>) -> Result<DbPool> {
    let embedded_db_url = option_env!("DATABASE_URL").map(|v| v.to_string());

    let db_url = db_url.map(|v| Ok(v)).unwrap_or_else(|| {
        embedded_db_url
            .map(|v| Ok(v))
            .unwrap_or_else(|| env::var("DATABASE_URL"))
    })?;

    Ok(Pool::builder(AsyncDieselConnectionManager::new(db_url)).build()?)
}

pub async fn run_migrations(pool: &DbPool) -> Result<()> {
    MIGRATIONS
        .run_pending_migrations(&mut pool.get().await?)
        .await?;

    Ok(())
}
