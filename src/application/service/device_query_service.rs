use async_trait::async_trait;
use crate::application::port::usecase::{DeviceQuery, DeviceQueryService};
use crate::domain::temperature::AvgTemperature;
use anyhow::*;


#[async_trait]
impl DeviceQuery for DeviceQueryService {
    async fn avg_device_temperature(&self, device_serial: String, start: String, end: String) -> Result<AvgTemperature> {
        todo!()
    }

    async fn avg_group_temperature(&self, group_serial: String, start: String, end: String) -> Result<Vec<AvgTemperature>> {
        todo!()
    }
}
