use async_trait::async_trait;
use crate::domain::device::{Device, DeviceGroup};
use crate::application::port::outbound::save_device_port::SaveDevicePort;
use crate::application::port::usecase::command::DeviceCommand;

pub struct DeviceCommandService {
    pub save_device: Box<dyn SaveDevicePort + Sync + Send>
}

#[async_trait]
impl DeviceCommand for DeviceCommandService {

    async fn register_device_group(&self, group_serial: String) -> anyhow::Result<DeviceGroup> {
        self.save_device.save_device_group(group_serial).await
    }

    async fn register_device(&self, device_serial: String, group_serial: String) -> anyhow::Result<Device> {
        self.save_device.save_device(device_serial, group_serial).await
    }
}

impl DeviceCommandService {
    pub fn new(save_device: Box<dyn SaveDevicePort + Sync + Send>) -> DeviceCommandService {
        DeviceCommandService{save_device}
    }
}

// impl Default for DeviceCommandService {
//     fn default() -> Self {
//         DeviceCommandService::new(Box::new())
//     }
// }
