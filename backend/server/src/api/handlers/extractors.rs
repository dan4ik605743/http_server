use std::marker::PhantomData;

use axum::{
    async_trait,
    extract::{FromRequest, FromRequestParts},
    http::{request::Parts, Request},
    Json,
};
use hyper::body::HttpBody;

use serde::{de::DeserializeOwned, Serialize};

use super::prelude::{CookieValue, ServerError};

pub struct AuthWrapper<D: Serialize + DeserializeOwned> {
    pub cookie: CookieValue,
    pub json: Json<D>,
}

#[async_trait]
impl<S, B, D> FromRequest<S, B> for AuthWrapper<D>
where
    D: DeserializeOwned + Serialize,
    B: HttpBody<Error = hyper::Error> + Send + 'static,
    B::Data: Send,
    S: Send + Sync,
{
    type Rejection = ServerError;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let mut parts = req.into_parts();
        let cookie = CookieValue::from_request_parts(&mut parts.0, state).await?;

        let req = Request::from_parts(parts.0, parts.1);
        let json = Json::<D>::from_request(req, state).await?;

        Ok(AuthWrapper { cookie, json })
    }
}

pub struct Auth<D, E = AuthWrapper<D>>
where
    D: Serialize + DeserializeOwned,
{
    pub extractors: E,
    marker: PhantomData<D>,
}

#[async_trait]
impl<S, D, E> FromRequestParts<S> for Auth<D, E>
where
    S: Send + Sync,
    E: FromRequestParts<S>,
    D: Serialize + DeserializeOwned,
{
    type Rejection = E::Rejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let extractors = E::from_request_parts(parts, state).await?;
        Ok(Auth {
            extractors,
            marker: PhantomData,
        })
    }
}

#[async_trait]
impl<S, B, D, E> FromRequest<S, B> for Auth<D, E>
where
    B: Send + 'static,
    S: Send + Sync,
    E: FromRequest<S, B>,
    D: Serialize + DeserializeOwned,
{
    type Rejection = E::Rejection;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let extractors = E::from_request(req, state).await?;
        Ok(Auth {
            extractors,
            marker: PhantomData,
        })
    }
}
