use anyhow::{Context, Result};

use super::Args;
use clap::Parser;

use axum::{
    http::uri::Uri,
    response::Redirect,
    routing::{get, post},
    Router,
};
use tower_http::services::{ServeDir, ServeFile};

mod get;
mod post;

use get::StaticSource;

pub async fn https_app() -> Result<Router> {
    let conn = db::get_connection_pool(&Args::parse().db)?
        .take()
        .context("Failed get connection pool")?;

    let serve_dir = ServeDir::new(StaticSource::SOURCE_DIR)
        .not_found_service(ServeFile::new(StaticSource::ERROR_PAGE));

    Ok(Router::new()
        // get
        .route("/register", get(get::register))
        .route("/login", get(get::login))
        // post
        .route(
            "/register",
            post({
                let conn = conn.clone();
                move |body| post::register(body, conn)
            }),
        )
        .route(
            "/login",
            post({
                let conn = conn.clone();
                move |body| post::login(body, conn)
            }),
        )
        // StaticResource and fallback other paths
        .nest_service("/", serve_dir.clone())
        .fallback_service(serve_dir.clone()))
}

pub async fn http_app() -> Result<Router> {
    Ok(Router::new().route("/", get(http_handler)))
}

async fn http_handler(uri: Uri) -> Redirect {
    let args = Args::parse();
    let uri = format!("https://{}:{}{}", args.ip, args.https_port, uri.path());

    Redirect::temporary(&uri)
}
