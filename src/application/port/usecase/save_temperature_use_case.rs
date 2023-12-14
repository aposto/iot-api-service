
use anyhow::Result;
use async_trait::async_trait;
use crate::domain::temperature::{SaveDeviceTemperature};

#[async_trait]
pub trait SaveTemperatureUseCase {
    async fn save_temperatures(ts: SaveDeviceTemperature) -> Result<bool>;

}

pub struct SaveTemperatureService {
}
