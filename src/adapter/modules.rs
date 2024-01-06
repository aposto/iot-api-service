use std::sync::Arc;
use crate::adapter::persistence::device_adapter::SaveDeviceAdapter;
use crate::adapter::persistence::temperature_adapter::SaveTemperatureAdapter;
use crate::application::service::DeviceCommandService;
use crate::application::service::SaveTemperatureService;
use once_cell::sync::{Lazy, OnceCell};
use crate::application::port::usecase::command::DeviceCommand;
use crate::application::port::usecase::use_case::SaveTemperatureUseCase;

pub static DEVICE_COMMAND: OnceCell<Arc<dyn DeviceCommand + Sync + Send>> = OnceCell::new();
//pub static SAVE_TEMPERATURE: OnceCell<Box<dyn SaveTemperatureUseCase + Sync + Send>> = OnceCell::new();

pub static SAVE_TEMPERATURE: Lazy<Box<dyn SaveTemperatureUseCase + Sync + Send>> = Lazy::new(|| {
    Box::new(SaveTemperatureService {
        save_temperature: Box::new(SaveTemperatureAdapter)
    })
});

pub fn inject_modules() -> anyhow::Result<String, anyhow::Error> {
    DEVICE_COMMAND.set(
        Arc::new(DeviceCommandService::new(
            Box::new(SaveDeviceAdapter)
        ))).ok();

    // let _ =
    // SAVE_TEMPERATURE.set(
    //     Box::new(SaveTemperatureService {
    //         save_temperature: Box::new(SaveTemperatureAdapter)
    //     })).ok();

    Ok("injected".to_string())
}