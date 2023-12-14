#[macro_use] extern crate log;

use iot_api_service::{infra, adapter};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    info!("Start web api server");
    infra::setup_system().await.ok();
    info!("Init web");

    adapter::web_start().await?;
    Ok(())
}
