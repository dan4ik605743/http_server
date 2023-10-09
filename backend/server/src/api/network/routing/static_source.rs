use std::convert::Infallible;

use hyper::Response;
use hyper::{Body, Request};
use tower::ServiceExt;
use tower_http::services::fs::ServeFileSystemResponseBody;
use tower_http::services::ServeFile;

pub type GetResponse = Result<Response<ServeFileSystemResponseBody>, Infallible>;

#[non_exhaustive]
pub struct StaticSource;

impl StaticSource {
    pub const SOURCE_DIR: &str = "../frontend";
    pub const ERROR_PAGE: &str = "../frontend/error.html";
    pub const LOGIN_PAGE: &str = "../frontend/login.html";
    pub const REGISTER_PAGE: &str = "../frontend/register.html";
    pub const USER_PAGE: &str = "../frontend/user.html";
}

pub async fn login(request: Request<Body>) -> GetResponse {
    get_page(StaticSource::LOGIN_PAGE, request).await
}

pub async fn register(request: Request<Body>) -> GetResponse {
    get_page(StaticSource::REGISTER_PAGE, request).await
}

pub async fn user(request: Request<Body>) -> GetResponse {
    get_page(StaticSource::USER_PAGE, request).await
}

async fn get_page(path_page: &str, request: Request<Body>) -> GetResponse {
    ServeFile::new(path_page).oneshot(request).await
}
