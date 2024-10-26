use crate::{db::DbPool, schema::user_tokens, UserToken};
use anyhow::Result;
use chrono::Utc;
use diesel::{delete, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use jsglue::abort::ABORT_HANDLES;
use tokio::task::JoinHandle;

pub fn run_worker(pool: DbPool) -> JoinHandle<Result<()>> {
    info!("Starting worker...");

    let handle = tokio::spawn(async move { worker_loop(pool).await });
    let abort = handle.abort_handle();

    // Hook into Glue's exit handler.
    ABORT_HANDLES.lock().unwrap().push(abort);

    handle
}

pub async fn worker_loop(pool: DbPool) -> Result<()> {
    let mut conn = pool.get().await?;

    loop {
        let tkns = user_tokens::table
            .select(UserToken::as_select())
            .load(&mut conn)
            .await?;

        for token in tkns {
            let time = Utc::now().timestamp_millis();

            if time >= token.expires.and_utc().timestamp_millis() {
                info!("Found expired token (id: {}). Deleting...", token.id);

                delete(&token).execute(&mut conn).await?;
            }
        }
    }
}
