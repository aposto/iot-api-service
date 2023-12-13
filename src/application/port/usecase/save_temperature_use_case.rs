
use anyhow::Result;
use crate::domain::temperature::{SaveDeviceTemperature};

pub trait SaveTemperatureUseCase {
    fn save_temperatures(&self, ts: SaveDeviceTemperature) -> Result<bool>;

}