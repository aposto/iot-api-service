use actix_web::{
    get,
    post,
    middleware,
    Responder,
    HttpResponse,
    middleware::Logger,
    web,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::adapter::web::response::CommandResponse;
use crate::adapter::web::web_state::WebState;
use crate::application::port::usecase::command::DeviceCommand;
use super::request::{DeviceGroupRegistryRequest, DeviceRegistryRequest, TemperatureRegistryRequest};
use crate::domain::temperature::SaveDeviceTemperature;

#[utoipa::path(
    post,
    path = "/device-group/{id}",
    responses(
        (status = 200, description = "Pet found succesfully", body = Pet),
        (status = NOT_FOUND, description = "Pet was not found")
    ),
    params(
        ("id" = u64, Path, description = "Pet database id to get Pet for"),
    )
)]
#[post("/device-group")]
async fn registry_device_group(data: web::Data<WebState>, req: web::Json<DeviceGroupRegistryRequest>) -> HttpResponse {
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


pub fn routes(cfg: &mut web::ServiceConfig) {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            registry_device_group,

        ),
        components(
            schemas(CommandResponse)
        ),
        tags(
            (name = "todo", description = "Todo management endpoints.")
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
