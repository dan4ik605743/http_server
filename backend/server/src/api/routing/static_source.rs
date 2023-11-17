use hyper::{Body, Request, StatusCode};
use tower::ServiceExt;
use tower_http::services::ServeFile;

use crate::api::handlers::responses::{HandlerResponse, HtmlPageResponse, ServerError};

#[non_exhaustive]
pub struct StaticSource;

impl StaticSource {
    pub const SOURCE_DIR: &'static str = "../frontend";
    pub const ERROR_PAGE: &'static str = "../frontend/error.html";
    pub const LOGIN_PAGE: &'static str = "../frontend/login.html";
    pub const REGISTER_PAGE: &'static str = "../frontend/register.html";
    pub const USER_PAGE: &'static str = "../frontend/user.html";
}

pub async fn get_page(path_page: &str) -> HandlerResponse<HtmlPageResponse> {
    let empty_request = Request::builder().body(Body::empty()).unwrap();

    ServeFile::new(path_page)
        .oneshot(empty_request)
        .await
        .map_err(|e| ServerError((anyhow::anyhow!(e), Some(StatusCode::INTERNAL_SERVER_ERROR))))
}
