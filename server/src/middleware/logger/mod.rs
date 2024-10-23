pub mod macros;

use crate::midlog_log;
use axum::{body::Body, http::Request, middleware::Next, response::Response};
use chrono::Utc;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    pub static ref FILTERS: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));
}

pub async fn logging_middleware(req: Request<Body>, next: Next) -> Response {
    let time_start = Utc::now().time();
    let method = &req.method().clone();
    let uri = &req.uri().clone();
    let res = next.run(req).await;
    let now = Utc::now().time();
    let elapsed = now - time_start;
    let path = uri.path_and_query().unwrap().as_str();

    for item in FILTERS.lock().unwrap().iter().cloned() {
        if path.contains(item) {
            return res;
        }
    }

    let path = if path.contains("?code=") {
        path.split("?code=").next().unwrap().to_string()
    } else {
        path.to_string()
    };

    midlog_log!(
        method.as_str(),
        path,
        res.status(),
        elapsed.num_milliseconds()
    );

    res
}
