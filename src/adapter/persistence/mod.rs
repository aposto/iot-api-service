use anyhow::Result;
mod device_repo;
mod db_pool;
pub mod device_adapter;
pub mod temperature_adapter;

pub use db_pool::get_pool;

pub async fn database_init(db_url: String) -> Result<()> {
    db_pool::create_mysql_pool(db_url, 10).await?;
    Ok(())
}