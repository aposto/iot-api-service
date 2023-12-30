use actix_web::{
    get,
    post,
    middleware,
    Responder,
    HttpResponse,
    middleware::Logger,
    web,
};
use anyhow::__private::kind::AdhocKind;
use crate::adapter::web::response::CommandResponse;
use super::request::{DeviceGroupRegistryRequest, DeviceRegistryRequest, TemperatureRegistryRequest};
use crate::application::port::usecase::*;
use crate::domain::temperature::SaveDeviceTemperature;

#[post("/device-group")]
async fn registry_device_group(req: web::Json<DeviceGroupRegistryRequest>) -> impl Responder {
    let group = DeviceCommandService::register_device_group(
        req.device_group_serial.clone()).await;

    HttpResponse::Ok().json(group.unwrap())
}
#[post("/device")]
async fn registry_device(req: web::Json<DeviceRegistryRequest>) -> impl Responder {

    let device = DeviceCommandService::register_device(
        req.serial_number.clone(),
        req.device_group_serial.clone()).await;
    HttpResponse::Ok().json(device.unwrap())
}

#[post("/temperatures")]
async fn save_temperature(req: web::Json<TemperatureRegistryRequest>) -> impl Responder {
    let domain = SaveDeviceTemperature {
        serial_number: req.serial_number.clone(),
        interval: req.interval,
        temperatures:  req.temperatures.clone(),
        registered_at: req.registered_at.clone(),
    };
    SaveTemperatureService::save_temperatures(domain).await
        .expect("save failed check log");
    HttpResponse::Ok().json(CommandResponse { msg: String::from("success")})
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
            .service(registry_device_group)
            .service(registry_device)
            .service(save_temperature)
            .service(about)
    );
}
