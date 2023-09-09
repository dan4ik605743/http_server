use anyhow::Result;
use clap::Parser;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::{ServeDir, ServeFile};

mod get;
mod post;

use get::StaticSource;

pub async fn create_app() -> Result<Router> {
    let conn = db::get_connection_pool(&super::Args::parse().db)
        .unwrap()
        .take()
        .unwrap();

    let serve_dir = ServeDir::new(StaticSource::SOURCE_DIR)
        .not_found_service(ServeFile::new(StaticSource::ERROR_PAGE));

    Ok(Router::new()
        // get
        .route("/register", get(get::register))
        .route("/login", get(get::login))
        .route("/error", get(get::error))
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
