use crate::api::users::prelude::*;

pub async fn user(request: Request<Body>) -> GetResponse {
    get_page(StaticSource::USER_PAGE, request).await
}
