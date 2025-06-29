use axum::{
    http::{HeaderMap, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use mime_guess::from_path;
use rust_embed::RustEmbed;
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "./ui/dist"]
struct Asset;

pub async fn serve_ui(uri: Uri) -> Response {
    let path = uri.path();
    let path = if path == "/" || path.is_empty() {
        "index.html"
    } else {
        &path[1..]
    };
    match Asset::get(path) {
        Some(content) => {
            let body = match content.data {
                Cow::Borrowed(b) => b.to_vec(),
                Cow::Owned(o) => o,
            };
            let mime = from_path(path).first_or_octet_stream();
            let mut headers = HeaderMap::new();
            headers.insert("content-type", mime.to_string().parse().unwrap());
            (headers, body).into_response()
        }
        None => {
            if path != "index.html" {
                if let Some(index) = Asset::get("index.html") {
                    let body = match index.data {
                        Cow::Borrowed(b) => b.to_vec(),
                        Cow::Owned(o) => o,
                    };
                    let mut headers = HeaderMap::new();
                    headers.insert("content-type", "text/html".parse().unwrap());
                    return (headers, body).into_response();
                }
            }
            (StatusCode::NOT_FOUND, "404 Not Found").into_response()
        }
    }
}
