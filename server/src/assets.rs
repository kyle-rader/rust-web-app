use axum::{
    body::Body,
    extract::Request,
    http::{header, HeaderValue, Response, StatusCode},
    response::IntoResponse,
};
use rust_embed::RustEmbed;
use tracing::{debug, info};

const INDEX: &str = "index.html";

#[derive(RustEmbed)]
#[folder = "../client/build/"]
struct Assets;

pub fn print_assets() {
    for file in Assets::iter() {
        info!("Asset: {}", file);
    }
}

pub async fn handler(req: Request) -> Result<impl IntoResponse, StatusCode> {
    debug!("{:?}: {:?}", req.method(), req.uri());

    let path = req.uri().path();
    let path = if path == "/" {
        String::from(INDEX)
    } else {
        path.trim_start_matches('/').to_owned()
    };

    let asset = match Assets::get(&path) {
        Some(data) => data,
        None => {
            if accepts_html(&req) {
                // If the request is for text/html assume this is an SPA and return the index.html
                Assets::get(INDEX).ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
            } else {
                // Otherwise return a 404
                return Err(StatusCode::NOT_FOUND);
            }
        }
    };

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, asset.metadata.mimetype())
        .body(Body::from(asset.data))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

fn accepts_html(req: &Request) -> bool {
    req.headers()
        .get("accept")
        .unwrap_or(&HeaderValue::from_static(""))
        .to_str()
        .unwrap_or_default()
        .contains("text/html")
}

#[cfg(test)]
mod tests {
    use axum::{body::Body, http};

    use crate::assets::accepts_html;

    #[test]
    fn test_accepts_html() {
        let req = http::Request::builder()
            .header("accept", "text/html")
            .body(Body::empty())
            .unwrap();
        assert!(accepts_html(&req));
    }

    #[test]
    fn test_accepts_html_no_accept() {
        let req = http::Request::builder().body(Body::empty()).unwrap();
        assert!(!accepts_html(&req));
    }
}
