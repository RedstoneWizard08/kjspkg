use app_core::AppError;
use axum::http::HeaderMap;
use axum_extra::extract::CookieJar;
use db::{get_user_for_token, DbConn, User};

use crate::Result;

pub async fn get_user_from_req(
    jar: &CookieJar,
    headers: &HeaderMap,
    conn: &mut DbConn,
) -> Result<User> {
    let token = if let Some(value) = headers.get("Authorization") {
        let val = value.to_str()?;

        if val.starts_with("Bearer ") {
            value.to_str()?.trim_start_matches("Bearer ").to_string()
        } else {
            if let Some(value) = jar.get("auth-token") {
                value.value().to_string()
            } else {
                return Err(AppError::MissingToken);
            }
        }
    } else if let Some(value) = jar.get("auth-token") {
        value.value().to_string()
    } else {
        return Err(AppError::MissingToken);
    };

    get_user_for_token(token, conn)
        .await?
        .ok_or(AppError::UnknownUser)
}
