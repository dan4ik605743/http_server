use anyhow::Result;

use super::Args;
use clap::Parser;

use api::network::routing;
use axum_server::tls_rustls::RustlsConfig;

async fn http_server() -> Result<()> {
    let args = Args::parse();
    let socket_addr = format!("{}:{}", args.ip, args.http_port).parse()?;

    Ok(axum_server::bind(socket_addr)
        .serve(routing::http_router().await?.into_make_service())
        .await?)
}

async fn https_server() -> Result<()> {
    const CERT_PATH: &str = "./server/assets/self-signed-certs/cert.pem";
    const KEY_PATH: &str = "./server/assets/self-signed-certs/key.pem";
    let rustls_config = RustlsConfig::from_pem_file(CERT_PATH, KEY_PATH).await?;

    let args = Args::parse();
    let socket_addr = format!("{}:{}", args.ip, args.https_port).parse()?;

    Ok(axum_server::bind_rustls(socket_addr, rustls_config)
        .serve(routing::https_router().await?.into_make_service())
        .await?)
}

pub async fn start() -> Result<()> {
    let http = tokio::spawn(http_server());
    let https = tokio::spawn(https_server());

    let _ = tokio::join!(http, https);
    Ok(())
}
