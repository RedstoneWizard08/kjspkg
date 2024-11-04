#[macro_use]
extern crate tracing;

pub mod data;
pub mod logger;
pub mod pkg;
pub mod util;

use anyhow::Result;
use data::{get_packages, get_users};
use db::{create_connection, run_migrations, users, User};
use diesel::{
    insert_into, update, ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper,
};
use diesel_async::RunQueryDsl;
use logger::init_logger;

pub async fn run() -> Result<()> {
    dotenvy::dotenv()?;
    init_logger();

    info!("[DB] Connecting...");

    let pool = create_connection(None).await?;
    let pkgs = get_packages();

    info!("[DB] Running migrations...");

    run_migrations(&pool).await?;

    info!("[DB] Updating users...");

    for user in get_users(&pkgs, &pool).await {
        let mut conn = pool.get().await?;

        let existing = users::table
            .filter(users::github_id.eq(user.github_id))
            .select(User::as_select())
            .first(&mut conn)
            .await
            .optional()?;

        if let Some(existing) = existing {
            info!(
                "[User] Updating user (ID {}) to set username to '{}'.",
                existing.id, user.username
            );

            update(users::table)
                .filter(users::id.eq(existing.id))
                .set(users::username.eq(user.username))
                .returning(User::as_returning())
                .get_result(&mut conn)
                .await?;
        } else {
            info!(
                "[User] Creating user (GitHub ID {}) to set username to '{}'.",
                user.github_id, user.username
            );

            insert_into(users::table)
                .values(&user)
                .returning(User::as_returning())
                .get_result(&mut conn)
                .await?;
        };
    }

    info!("[DB] Done updating users!");
    info!("[Package] Creating packages...");

    for pkg in pkgs {
        pkg.create(&mut pool.get().await?).await?;
    }

    info!("[Package] Done creating packages!");

    Ok(())
}
