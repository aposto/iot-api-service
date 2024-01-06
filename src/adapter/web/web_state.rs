use std::sync::Arc;
use crate::application::port::usecase::command::DeviceCommand;
use crate::application::port::usecase::use_case::SaveTemperatureUseCase;

#[derive(Clone)]
pub struct WebState {
    pub device_command: Arc<dyn DeviceCommand>,
    pub save_temperature_usecase: Arc<dyn SaveTemperatureUseCase>,
}
