
use std::env;
use log::info;
use crate::adapter::persistence_init;

pub async fn setup_system() -> anyhow::Result<()> {
    info!("Setup ENV");
    let env_file = env::var("ENV").unwrap_or(".env".to_string());
    dotenv::from_filename(env_file).ok();
    env_logger::init();

    let database_url = dotenv::var("DATABASE_URL")
        .or_else(|_| dotenv::var("DATABASE_URL"))
        .expect("DATABASE_URL must be set in order to run tests");


    info!("Setup Database: {}", database_url);
    persistence_init(database_url).await?;
    Ok(())
}