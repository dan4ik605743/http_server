use clap::Parser;

mod api;
mod args;
mod logger;
mod redis;
mod server;

use args::Args;
use db::SqliteConnection;
use redis::RedisConnection;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    logger::init_logger();

    SqliteConnection::set(&args.db);
    RedisConnection::set(args.redis_port).await;

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
