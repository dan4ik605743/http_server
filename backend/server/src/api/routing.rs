use anyhow::Result;
use clap::Parser;

use axum::{
    http::uri::Uri,
    response::Redirect,
    routing::{get, post},
    Router,
};
use tower_http::services::{ServeDir, ServeFile};

pub mod static_source;

use crate::api::handlers;
use crate::args::Args;
use static_source::StaticSource;

pub async fn https_router() -> Result<Router> {
    let serve_dir = ServeDir::new(StaticSource::SOURCE_DIR)
        .not_found_service(ServeFile::new(StaticSource::ERROR_PAGE));

    Ok(Router::new()
        // get
        //
        // users
        .route("/user", get(handlers::users::user))
        .route(
            "/user/auth/register",
            get(|| static_source::get_page(StaticSource::REGISTER_PAGE)),
        )
        .route(
            "/user/auth/login",
            get(|| static_source::get_page(StaticSource::LOGIN_PAGE)),
        )
        // post
        //
        // users
        .route("/user/auth/register", post(handlers::users::register))
        .route("/user/auth/login", post(handlers::users::login))
        // StaticSource
        .nest_service("/", serve_dir))
}

pub async fn http_router() -> Result<Router> {
    Ok(Router::new().route("/", get(http_handler)))
}

async fn http_handler(uri: Uri) -> Redirect {
    let args = Args::parse();
    let uri = format!("https://{}:{}{}", args.ip, args.https_port, uri.path());

    Redirect::temporary(&uri)
}
