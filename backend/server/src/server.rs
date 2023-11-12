use anyhow::Result;

use super::Args;
use clap::Parser;

use super::api::routing;
use axum_server::tls_rustls::RustlsConfig;

const CERT_PATH: &str = "./server/assets/self-signed-certs/cert.pem";
const KEY_PATH: &str = "./server/assets/self-signed-certs/key.pem";

async fn http_server() -> Result<()> {
    let args = Args::parse();
    let socket_addr = format!("{}:{}", args.ip, args.http_port).parse()?;

    Ok(axum_server::bind(socket_addr)
        .serve(routing::http_router().await?.into_make_service())
        .await?)
}

async fn https_server() -> Result<()> {
    let rustls_config = RustlsConfig::from_pem_file(CERT_PATH, KEY_PATH).await?;

    let args = Args::parse();
    let socket_addr = format!("{}:{}", args.ip, args.https_port).parse()?;

    Ok(axum_server::bind_rustls(socket_addr, rustls_config)
        .serve(routing::https_router().await?.into_make_service())
        .await?)
}

pub async fn start() -> Result<()> {
    tokio::select! {
         Err(e) = http_server() => Err(e),
         Err(e) = https_server() => Err(e),
    }
}
