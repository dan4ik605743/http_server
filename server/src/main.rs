use std::net::SocketAddr;

use anyhow::Result;
use clap::Parser;

mod args;
mod logger;
mod network;

use args::Args;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    logger::init_logger();

    tracing::info!("Server started");

    let start = start(
        format!("{}:{}", args.ip, args.port).parse()?,
        network::create_app().await?,
    );
    let exit = tokio::signal::ctrl_c();

    tokio::select! {
        Err(err) = start => tracing::error!("{err}"),
        _ = exit => (),

    }

    tracing::info!("Server shutdowned");
    Ok(())
}

async fn start(socket_addr: SocketAddr, app: axum::Router) -> Result<()> {
    Ok(axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await?)
}
