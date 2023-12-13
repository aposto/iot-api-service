use actix_web::{
    get,
    post,
    middleware,
    Responder,
    HttpResponse,
    middleware::Logger,
    web
};
use crate::adapter::web::response::DeviceGroupRegistryResponse;
use crate::domain::device::DeviceGroup;
use super::request::DeviceRegistryRequest;
use chrono::prelude::*;
use crate::core;
#[post("/device-group")]
async fn registry_device(req_body: web::Json<DeviceRegistryRequest>) -> impl Responder {
    let response = DeviceGroupRegistryResponse {
        msg: "success".to_string(),
        data: DeviceGroup {
            device_group_id: 1,
            serial_number: "A1".to_string(),
            created_at: core::datetime_string(Local::now()),
        }
    };

    //println!("RES {:?} {:?}", req_body, DateTime::parse_from_str());
    HttpResponse::Ok().json(response)
}

#[get("/about")]
async fn about() -> impl Responder {
    HttpResponse::Ok().body("HELLO")
}



pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .wrap(Logger::default())
            .wrap(middleware::Compress::default())
            //.wrap(HttpAuthentication::bearer(ok_validator))
            .service(registry_device)
            .service(about)
            //.route("users", web::get().to(users::get_users))
    );
}
