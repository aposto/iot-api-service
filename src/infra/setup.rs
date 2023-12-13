
use std::env;
pub fn setup_system() -> anyhow::Result<()>{
    let env_file = env::var("ENV").unwrap_or(".env".to_string());
    dotenv::from_filename(env_file).ok();
    env_logger::init();
    Ok(())
}