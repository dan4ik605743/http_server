use hyper::Response;
use std::convert::Infallible;
use tower_http::services::fs::ServeFileSystemResponseBody;

pub type GetResponse = Result<Response<ServeFileSystemResponseBody>, Infallible>;

pub mod tools {
    use hyper::{Body, Request};
    use tower::ServiceExt;
    use tower_http::services::ServeFile;

    use super::GetResponse;

    pub async fn get_page(path_page: &str, request: Request<Body>) -> GetResponse {
        ServeFile::new(path_page).oneshot(request).await
    }
}
