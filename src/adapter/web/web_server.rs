use std::env;
use actix_web::{App, HttpServer, middleware};
use log::info;
use crate::adapter::state::state_factory;
use super::web_api;

pub async fn start() -> std::io::Result<()> {
    let bind = env::var("BIND").expect("server bind address");
    info!("🅿️ Starting 🦀 IOT Temperature Service at ⛽️ 🧨 http://{} ", bind);

    HttpServer::new(|| {
        //let auth = HttpAuthentication::basic(validator);
        App::new()
            .data_factory(state_factory)
            .wrap(middleware::Logger::default())
            //.wrap(HttpAuthentication::bearer(ok_validator))
            .configure(web_api::routes)
            //.app_data()

            //.service(hello)
            //.service(echo)
            //.route("/hey", web::get().to(manual_hello))
            //.route("/greet", web::get().to(greet))
    })
        .bind(bind)?
        .run()
        .await
}

