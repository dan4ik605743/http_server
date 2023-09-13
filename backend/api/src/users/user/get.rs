use crate::users::prelude::*;

pub async fn user(request: Request<Body>) -> GetResponse {
    get_page(StaticSource::LOGIN_PAGE, request).await
}
