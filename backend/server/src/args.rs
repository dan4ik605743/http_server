use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// IP
    #[arg(short, long, default_value_t = String::from("127.0.0.1"))]
    pub ip: String,

    /// HTTP PORT
    #[arg(long, default_value_t = 3000)]
    pub http_port: u32,

    /// HTTPS PORT
    #[arg(long, default_value_t = 3443)]
    pub https_port: u32,

    /// HTTPS PORT
    #[arg(long, default_value_t = 6379)]
    pub redis_port: u32,

    /// DB
    #[arg(short, long)]
    pub db: String,
}
