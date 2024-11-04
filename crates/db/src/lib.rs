#[macro_use]
extern crate utoipa;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate diesel;

mod models;
mod schema;
mod util;

pub use models::*;
pub use schema::*;
pub use util::*;

use app_core::Result;
use diesel::{
    r2d2::{ConnectionManager, Pool as SyncPool, PooledConnection},
    PgConnection,
};
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
pub type SyncDbPool = SyncPool<ConnectionManager<PgConnection>>;
pub type SyncDbConn = PooledConnection<ConnectionManager<PgConnection>>;

pub async fn create_connection(db_url: Option<String>) -> Result<DbPool> {
    let embedded_db_url = option_env!("DATABASE_URL").map(|v| v.to_string());

    let db_url = db_url.map(|v| Ok(v)).unwrap_or_else(|| {
        embedded_db_url
            .map(|v| Ok(v))
            .unwrap_or_else(|| env::var("DATABASE_URL"))
    })?;

    Ok(Pool::builder(AsyncDieselConnectionManager::new(db_url)).build()?)
}

pub fn create_sync_connection(db_url: Option<String>) -> Result<SyncDbPool> {
    let embedded_db_url = option_env!("DATABASE_URL").map(|v| v.to_string());

    let db_url = db_url.map(|v| Ok(v)).unwrap_or_else(|| {
        embedded_db_url
            .map(|v| Ok(v))
            .unwrap_or_else(|| env::var("DATABASE_URL"))
    })?;

    Ok(SyncPool::builder().build(ConnectionManager::<PgConnection>::new(db_url))?)
}

pub async fn run_migrations(pool: &DbPool) -> Result<()> {
    MIGRATIONS
        .run_pending_migrations(&mut pool.get().await?)
        .await?;

    Ok(())
}
