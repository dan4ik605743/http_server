use crate::network::{
    responses::get::{tools::get_page, GetResponse},
    routing::static_source::StaticSource,
};
use hyper::{Body, Request};

pub async fn user(request: Request<Body>) -> GetResponse {
    get_page(StaticSource::LOGIN_PAGE, request).await
}
