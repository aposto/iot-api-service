use std::sync::Arc;
use crate::adapter::persistence::device_adapter::SaveDeviceAdapter;
use crate::adapter::persistence::temperature_adapter::SaveTemperatureAdapter;
use crate::adapter::web::web_state::WebState;
use crate::application::service::{DeviceCommandService, SaveTemperatureService};

pub async fn state_factory() -> std::io::Result<WebState> {
    let device_command = DeviceCommandService::new(Box::new(SaveDeviceAdapter));
    let save_temperature_usecase = SaveTemperatureService::new(Box::new(SaveTemperatureAdapter));

    let state = WebState {
        device_command: Arc::new(device_command),
        save_temperature_usecase: Arc::new(save_temperature_usecase),
    };
    Ok(state)
}