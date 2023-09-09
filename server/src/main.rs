use std::net::SocketAddr;

use anyhow::Result;
use axum_server::tls_rustls::RustlsConfig;
use clap::Parser;

mod args;
mod logger;
mod network;

use args::Args;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    logger::init_logger();

    let rustls_config = RustlsConfig::from_pem_file(
        "./server/assets/self-signed-certs/cert.pem",
        "./server/assets/self-signed-certs/key.pem",
    )
    .await?;

    // let x: (SocketAddr, RustlsConfig) = (args,)

    tracing::info!("Server started");

    let start = start(
        format!("{}:{}", args.ip, args.port).parse()?,
        rustls_config,
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

async fn start(
    socket_addr: SocketAddr,
    rustls_config: RustlsConfig,
    app: axum::Router,
) -> Result<()> {
    Ok(axum_server::bind_rustls(socket_addr, rustls_config)
        .serve(app.into_make_service())
        .await?)
}
