use actix_web::{
    get,
    post,
    middleware,
    Responder,
    HttpResponse,
    middleware::Logger,
    web,
};
use serde::Deserialize;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use crate::adapter::web::response::CommandResponse;
use crate::adapter::web::web_state::WebState;
use super::request::{DeviceGroupRegistryRequest, DeviceRegistryRequest, TemperatureRegistryRequest};
use crate::domain::temperature::SaveDeviceTemperature;
//use serde_json::Value;


#[utoipa::path(
    post,
    path = "/device-group/{id}",
    params(
        ("id" = u64, Path, description = "IOT database id to get device group for")
    ),
    request_body = DeviceRegistryRequest,
    responses(
        (status = 200, description = "Device Group found successfully" ),
        (status = NOT_FOUND, description = "Device Group was not found")
    )
)]
#[post("/device-group")]
async fn registry_device_group(data: web::Data<WebState>, req: web::Json<DeviceRegistryRequest>) -> HttpResponse {
    let group = data.device_command.register_device_group(req.device_group_serial.clone()).await;
    HttpResponse::Ok().json(group.unwrap())
}

#[post("/device")]
async fn registry_device(data: web::Data<WebState>, req: web::Json<DeviceRegistryRequest>) -> HttpResponse { // impl Responder {

    let device = data.device_command.register_device(
        req.serial_number.clone(),
        req.device_group_serial.clone()).await;
    HttpResponse::Ok().json(device.unwrap())
}

#[post("/temperatures")]
async fn save_temperature(data: web::Data<WebState>, req: web::Json<TemperatureRegistryRequest>) -> HttpResponse {
    let domain = SaveDeviceTemperature {
        serial_number: req.serial_number.clone(),
        interval: req.interval,
        temperatures: req.temperatures.clone(),
        registered_at: req.registered_at.clone(),
    };
    data.save_temperature_usecase.save_device_temperatures(domain).await
        .expect("save failed check log");
    HttpResponse::Ok().json(CommandResponse { msg: String::from("success") })
}

#[get("/about")]
async fn about() -> impl Responder {
    HttpResponse::Ok().body("HELLO")
}


// https://github.com/graphul-rs/graphul/blob/main/examples/utoipa-swagger-ui/src/swagger.rs


pub fn routes(cfg: &mut web::ServiceConfig) {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            registry_device_group,
        ),
        components(
            schemas(CommandResponse, DeviceRegistryRequest)
        ),
        tags(
            (name = "device", description = "Device management endpoints.")
        ),
    )]
    struct ApiDoc;
    let openapi = ApiDoc::openapi();

    cfg.service(
        web::scope("/api/v1")
            .wrap(Logger::default())
            .wrap(middleware::Compress::default())
            .service(registry_device_group)
            .service(registry_device)
            .service(save_temperature)
            .service(about)
    ).service(
        SwaggerUi::new("/swagger-ui/{_:.*}")
            .url("/api-docs/openapi.json", openapi.clone()),
    );
}
