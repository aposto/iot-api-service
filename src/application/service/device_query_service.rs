use crate::application::port::usecase::query::DeviceQuery;
use crate::domain::temperature::AvgTemperature;
use anyhow::Result;
use async_trait::async_trait;

pub struct DeviceQueryService;

#[async_trait]
impl DeviceQuery for DeviceQueryService {
    async fn avg_device_temperature(&self, _device_serial: String, _start: String, _end: String) -> Result<AvgTemperature> {
        todo!()
    }

    async fn avg_group_temperature(&self, _group_serial: String, _start: String, _end: String) -> Result<Vec<AvgTemperature>> {
        todo!()
    }
}
