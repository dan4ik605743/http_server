use hyper::{Body, Request};
use tower::ServiceExt;
use tower_http::services::ServeFile;

use super::GetResponse;

pub async fn get_page(path_page: &str, request: Request<Body>) -> GetResponse {
    ServeFile::new(path_page).oneshot(request).await
}
