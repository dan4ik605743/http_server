use anyhow::{Context, Result};

use clap::Parser;

use axum::{
    http::uri::Uri,
    response::Redirect,
    routing::{get, post},
    Router,
};
use tower_http::services::{ServeDir, ServeFile};

pub mod static_source;
pub use self::static_source::StaticSource;

use crate::users;
use args::Args;

pub async fn https_router() -> Result<Router> {
    let conn = db::get_connection_pool(&Args::parse().db)?
        .take()
        .context("Failed get connection pool")?;

    let serve_dir = ServeDir::new(StaticSource::SOURCE_DIR)
        .not_found_service(ServeFile::new(StaticSource::ERROR_PAGE));

    Ok(Router::new()
        // get
        //
        // users
        .route("/user", get(users::user::get::user))
        .route("/user/auth/register", get(users::auth::get::register))
        .route("/user/auth/login", get(users::auth::get::login))
        // post
        //
        // users
        .route(
            "/user/auth/register",
            post({
                let conn = conn.clone();
                move |body| users::auth::post::register(body, conn)
            }),
        )
        .route(
            "/user/auth/login",
            post({
                let conn = conn.clone();
                move |body| users::auth::post::login(body, conn)
            }),
        )
        // StaticResource and fallback other paths
        .nest_service("/", serve_dir.clone())
        .fallback_service(serve_dir.clone()))
}

pub async fn http_router() -> Result<Router> {
    Ok(Router::new().route("/", get(http_handler)))
}

async fn http_handler(uri: Uri) -> Redirect {
    let args = Args::parse();
    let uri = format!("https://{}:{}{}", args.ip, args.https_port, uri.path());

    Redirect::temporary(&uri)
}
