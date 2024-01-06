use crate::domain::device::{Device, DeviceGroup};
use async_trait::async_trait;
#[async_trait]
pub trait DeviceCommand {
    async fn register_device_group(&self, group_serial: String) -> anyhow::Result<DeviceGroup>;

    async fn register_device(&self, device_serial: String, group_serial: String) -> anyhow::Result<Device>;
}

