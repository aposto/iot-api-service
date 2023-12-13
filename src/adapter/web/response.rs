use serde::Serialize;
use crate::domain::device::{Device, DeviceGroup};

#[derive(Debug)]
pub struct CommandResponse {
    pub msg: String
}


#[derive(Debug, Serialize)]
pub struct DeviceRegistryResponse {
    pub msg: String,
    pub data: Device,
}


#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceGroupRegistryResponse {
    pub msg: String,
    pub data: DeviceGroup,
}

#[derive(Debug)]
pub struct TemperatureQueryData {
    pub id: i64,
    pub serial_number: String,
    pub average_temperature: f32,
}
#[derive(Debug)]
pub struct TemperatureQueryResponse {
    pub msg: String,
    pub data: Vec<TemperatureQueryData>,
}

#[derive(Debug)]
pub struct TemperatureGroupQueryResponse {
    pub msg: String,
    pub data: Vec<TemperatureQueryData>,
}
