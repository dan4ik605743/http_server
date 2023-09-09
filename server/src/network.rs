use anyhow::Result;
use clap::Parser;

use axum::{
    handler::HandlerWithoutStateExt,
    routing::{get, post},
    Router,
};
use hyper::StatusCode;
use tower_http::services::ServeDir;

mod tools;

const STATIC_SOURCE: &str = "./server/assets/front";

pub async fn create_app() -> Result<Router> {
    let conn = db::get_connection_pool(&super::Args::parse().db)
        .unwrap()
        .take()
        .unwrap();

    Ok(Router::new()
        // get
        // .route("/", get(|| tools::get_html_page(FrontendPages::INDEX)))
        // .route("/admin", get(start_page))
        // .route(
        //     "/register",
        //     get(|| tools::get_html_page(FrontendPages::REGISTER)),
        // )
        // .route("/login", get(|| tools::get_html_page(FrontendPages::LOGIN)))
        // .route("/user", get(|| tools::get_html_page(FrontendPages::USER)))
        // .route("/*path", get(|| async { "Not Found" }))
        // post
        .route(
            "/register",
            post({
                let conn = conn.clone();
                move |body| tools::register(body, conn)
            }),
        )
        .route(
            "/login",
            post({
                let conn = conn.clone();
                move |body| tools::login(body, conn)
            }),
        )
        .nest_service("/", ServeDir::new(STATIC_SOURCE))
        .fallback_service(
            ServeDir::new(STATIC_SOURCE).not_found_service(handle_404.into_service()),
        ))
}

async fn handle_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}
