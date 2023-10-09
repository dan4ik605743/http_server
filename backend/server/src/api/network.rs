pub mod handlers_utils;
pub mod routing;
pub mod session;

use anyhow::{anyhow, Context, Result};

use clap::Parser;
use db::DbPool;
use redis::aio::MultiplexedConnection;
use std::cell::OnceCell;

use crate::Args;

pub type RedisPool = MultiplexedConnection;

pub fn get_conn_db() -> Result<DbPool> {
    Ok(db::get_connection_pool(&Args::parse().db)?
        .take()
        .context("Failed get connection db_pool")?
        .clone())
}

async fn get_mult_conn_redis() -> Result<OnceCell<RedisPool>> {
    let mult_conn_redis = redis::Client::open("redis://localhost/")?
        .get_multiplexed_tokio_connection()
        .await?;

    let cell = OnceCell::<RedisPool>::new();

    cell.set(mult_conn_redis)
        .map_err(|_| anyhow::anyhow!("Failed to set connection redis"))?;

    Ok(cell)
}

pub async fn get_conn_redis() -> Result<RedisPool> {
    Ok(get_mult_conn_redis()
        .await
        .map_err(|_| anyhow!("Failed to set connection redis"))?
        .take()
        .context("Failed get connetion redis")?
        .clone())
}
