use std::convert::Infallible;

use hyper::{Body, Request, Response};
use tower::ServiceExt;
use tower_http::services::{fs::ServeFileSystemResponseBody, ServeFile};

pub type GetPageResponse = Result<Response<ServeFileSystemResponseBody>, Infallible>;

#[non_exhaustive]
pub struct StaticSource;

impl StaticSource {
    pub const SOURCE_DIR: &str = "../frontend";
    pub const ERROR_PAGE: &str = "../frontend/error.html";
    pub const LOGIN_PAGE: &str = "../frontend/login.html";
    pub const REGISTER_PAGE: &str = "../frontend/register.html";
    pub const USER_PAGE: &str = "../frontend/user.html";
}

pub async fn get_page(path_page: &str, request: Request<Body>) -> GetPageResponse {
    ServeFile::new(path_page).oneshot(request).await
}

// pub async fn login(request: Request<Body>) -> GetPageResponse {
//     get_page(StaticSource::LOGIN_PAGE, request).await
// }

// pub async fn register(request: Request<Body>) -> GetPageResponse {
//     get_page(StaticSource::REGISTER_PAGE, request).await
// }

// pub async fn user(request: Request<Body>) -> GetPageResponse {
//     get_page(StaticSource::USER_PAGE, request).await
// }
