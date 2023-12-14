use crate::application::port::usecase::{DeviceCommand, DeviceCommandService};
use crate::domain::device::{Device, DeviceGroup};
use anyhow::Result;
use async_trait::async_trait;
use crate::application::port::outbound::SaveDevicePort;


#[async_trait]
impl DeviceCommand for DeviceCommandService {

    async fn register_device_group(group_serial: String) -> Result<DeviceGroup> {
        SaveDevicePort::save_device_group(group_serial).await
    }

    async fn register_device(device_serial: String, group_serial: String) -> Result<Device> {
        SaveDevicePort::save_device(device_serial, group_serial).await
    }
}