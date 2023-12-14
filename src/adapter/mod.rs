use anyhow::Result;
mod web;
mod persistence;
mod app_context;

pub use app_context::AppContext;


pub async fn persistence_init(db_url: String) -> Result<()> {
    persistence::database_init(db_url).await?;
    // persistence::nosql_init
    // cache::redis_init
    Ok(())
}
pub async fn web_start() -> std::io::Result<()> {
    return web::web_server::start().await
}

