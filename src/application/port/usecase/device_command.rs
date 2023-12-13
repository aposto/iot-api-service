use crate::domain::device::{Device, DeviceGroup};
use anyhow::Result;
pub trait DeviceCommand {
    fn register_device_group(&self, group_serial: String) -> Result<DeviceGroup>;

    fn register_device(&self, device_serial: String) -> Result<Device>;
}
