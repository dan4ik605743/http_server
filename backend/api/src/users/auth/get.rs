use crate::users::prelude::*;

pub async fn login(request: Request<Body>) -> GetResponse {
    get_page(StaticSource::LOGIN_PAGE, request).await
}

pub async fn register(request: Request<Body>) -> GetResponse {
    get_page(StaticSource::REGISTER_PAGE, request).await
}
