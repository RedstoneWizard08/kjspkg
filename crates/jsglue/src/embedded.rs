use axum::{body::Body, debug_handler, extract::Request, response::Response, Extension};
use hyper::{header::CONTENT_TYPE, StatusCode};
use include_dir::Dir;

#[debug_handler]
pub async fn handle_embedded(
    Extension(dir): Extension<Dir<'static>>,
    req: Request<Body>,
) -> Response<Body> {
    let path = req.uri().path();
    let index_path = format!("{}index.html", path);

    let path = if path.ends_with('/') {
        index_path.as_str()
    } else {
        path
    };

    let path = path.trim_start_matches('/');

    // Try the file
    if let Some(file) = dir.get_file(path) {
        let mime = mime_guess::from_path(path).first().unwrap();

        return Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, mime.to_string())
            .body(Body::from(file.contents().to_vec()))
            .unwrap();
    }

    // Try a fallback page
    if let Some(file) = dir.get_file("fallback.html") {
        return Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, "text/html")
            .body(Body::from(file.contents().to_vec()))
            .unwrap();
    }

    // Try a 404 page
    if let Some(file) = dir.get_file("404.html") {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(CONTENT_TYPE, "text/html")
            .body(Body::from(file.contents().to_vec()))
            .unwrap();
    }

    // Maybe it's an SPA?
    if let Some(file) = dir.get_file("index.html") {
        return Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, "text/html")
            .body(Body::from(file.contents().to_vec()))
            .unwrap();
    }

    // 404 not found in plaintext
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(
            "Cannot find the file specified!".as_bytes().to_vec(),
        ))
        .unwrap()
}
