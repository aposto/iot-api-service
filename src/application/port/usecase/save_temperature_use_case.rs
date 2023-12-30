
use anyhow::Result;
use crate::domain::temperature::{SaveDeviceTemperature};

pub trait SaveTemperatureUseCase {
    async fn save_temperatures(ts: SaveDeviceTemperature) -> Result<bool>;

}

pub struct SaveTemperatureService {
}
