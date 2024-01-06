
use crate::domain::temperature::AvgTemperature;
use async_trait::async_trait;
#[async_trait]
pub trait DeviceQuery {
    async fn avg_device_temperature(&self, device_serial: String, start: String, end: String) -> anyhow::Result<AvgTemperature>;
    async fn avg_group_temperature(&self, group_serial: String, start: String, end: String) -> anyhow::Result<Vec<AvgTemperature>>;
}

