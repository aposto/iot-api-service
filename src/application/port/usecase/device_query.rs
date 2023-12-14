
use anyhow::Result;
use async_trait::async_trait;
use crate::domain::temperature::AvgTemperature;

#[async_trait]
pub trait DeviceQuery {
    async fn avg_device_temperature(&self, device_serial: String, start: String, end: String) -> Result<AvgTemperature>;
    async fn avg_group_temperature(&self, group_serial: String, start: String, end: String) -> Result<Vec<AvgTemperature>>;
}

pub struct DeviceQueryService {
}
