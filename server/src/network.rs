use anyhow::Result;
use clap::Parser;

use axum::{
    routing::{get, post},
    Router,
};

mod tools;
use tools::FrontendPages;

pub async fn create_app() -> Result<Router> {
    let conn = db::get_connection_pool(&super::Args::parse().db)
        .unwrap()
        .take()
        .unwrap();

    Ok(Router::new()
        // get
        .route("/", get(|| tools::get_html_page(FrontendPages::INDEX)))
        .route("/admin", get(start_page))
        .route(
            "/register",
            get(|| tools::get_html_page(FrontendPages::REGISTER)),
        )
        .route("/login", get(|| tools::get_html_page(FrontendPages::LOGIN)))
        .route("/user", get(|| tools::get_html_page(FrontendPages::USER)))
        .route("/unlogin", get(start_page))
        .route("/*path", get(|| async { "Not Found" }))
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
        ))
}

async fn start_page() -> &'static str {
    "not found"
}
