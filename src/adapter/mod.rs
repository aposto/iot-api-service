use anyhow::Result;
use crate::adapter::modules::inject_modules;

mod web;
mod persistence;
mod state;
pub mod modules;


pub async fn persistence_init(db_url: String) -> Result<()> {
    persistence::database_init(db_url).await?;
    // persistence::nosql_init
    // cache::redis_init
    Ok(())
}
pub async fn web_start() -> std::io::Result<()> {
    return web::web_server::start().await
}

pub fn init_modules() -> Result<()> {
    inject_modules().expect("injection failed");
    Ok(())
}
