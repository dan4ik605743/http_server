use anyhow::Result;
use clap::Parser;

mod api;
mod args;
mod logger;
mod server;

use args::Args;

#[tokio::main]
async fn main() -> Result<()> {
    Args::parse();
    logger::init_logger();

    tracing::info!("Server started");

    let start = server::start();
    let exit = tokio::signal::ctrl_c();

    tokio::select! {
        Err(e) = start => tracing::error!("{e}"),
        _ = exit => (),

    }

    tracing::info!("Server shutdowned");
    Ok(())
}
