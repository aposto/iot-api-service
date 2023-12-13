mod web;

pub async fn web_start() -> std::io::Result<()> {
    return web::web_server::start().await
}