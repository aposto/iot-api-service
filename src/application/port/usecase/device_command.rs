use crate::domain::device::{Device, DeviceGroup};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait DeviceCommand {
    async fn register_device_group(group_serial: String) -> Result<DeviceGroup>;

    async fn register_device(device_serial: String, group_serial: String) -> Result<Device>;
}

pub struct DeviceCommandService {
}
