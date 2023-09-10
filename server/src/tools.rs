use anyhow::Result;
use clap::Parser;

use axum_server::tls_rustls::RustlsConfig;

use super::Args;

pub async fn start() -> Result<()> {
    const CERT_PATH: &str = "./server/assets/self-signed-certs/cert.pem";
    const KEY_PATH: &str = "./server/assets/self-signed-certs/key.pem";
    let rustls_config = RustlsConfig::from_pem_file(CERT_PATH, KEY_PATH).await?;

    let args = Args::parse();
    let socket_addr = format!("{}:{}", args.ip, args.port).parse()?;

    Ok(axum_server::bind_rustls(socket_addr, rustls_config)
        .serve(super::network::create_app().await?.into_make_service())
        .await?)
}
