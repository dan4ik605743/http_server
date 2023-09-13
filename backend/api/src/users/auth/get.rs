use crate::network::{
    responses::get::{tools::get_page, GetResponse},
    routing::static_source::StaticSource,
};
use hyper::{Body, Request};

pub async fn login(request: Request<Body>) -> GetResponse {
    get_page(StaticSource::LOGIN_PAGE, request).await
}

pub async fn register(request: Request<Body>) -> GetResponse {
    get_page(StaticSource::REGISTER_PAGE, request).await
}
