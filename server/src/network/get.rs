use std::convert::Infallible;

use hyper::{Body, Request, Response};
use tower::ServiceExt;
use tower_http::services::{fs::ServeFileSystemResponseBody, ServeFile};

#[non_exhaustive]
pub struct StaticSource;

impl StaticSource {
    pub const SOURCE_DIR: &str = "./server/assets/front";
    pub const ERROR_PAGE: &str = "./server/assets/front/error.html";
    pub const LOGIN_PAGE: &str = "./server/assets/front/login.html";
    pub const REGISTER_PAGE: &str = "./server/assets/front/register.html";
}

type GetResult = Result<Response<ServeFileSystemResponseBody>, Infallible>;

pub async fn login(request: Request<Body>) -> GetResult {
    ServeFile::new(StaticSource::LOGIN_PAGE)
        .oneshot(request)
        .await
}

pub async fn register(request: Request<Body>) -> GetResult {
    ServeFile::new(StaticSource::REGISTER_PAGE)
        .oneshot(request)
        .await
}
