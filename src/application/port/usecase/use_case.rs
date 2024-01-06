
use crate::domain::temperature::{SaveDeviceTemperature};
use async_trait::async_trait;
#[async_trait]
pub trait SaveTemperatureUseCase {
    async fn save_device_temperatures(&self, ts: SaveDeviceTemperature) -> anyhow::Result<bool>;
}

