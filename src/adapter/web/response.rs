use serde::Serialize;
use utoipa::ToSchema;
use crate::domain::device::{Device, DeviceGroup};

#[derive(Debug, Serialize, ToSchema)]
pub struct CommandResponse {
    pub msg: String
}


#[derive(Debug, Serialize, ToSchema)]
pub struct DeviceRegistryResponse {
    pub msg: String,
    pub data: Device,
}


#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeviceGroupRegistryResponse {
    pub msg: String,
    pub data: DeviceGroup,
}

#[derive(Debug, ToSchema)]
pub struct TemperatureQueryData {
    pub id: i64,
    pub serial_number: String,
    pub average_temperature: f32,
}
#[derive(Debug, ToSchema)]
pub struct TemperatureQueryResponse {
    pub msg: String,
    pub data: Vec<TemperatureQueryData>,
}

#[derive(Debug, ToSchema)]
pub struct TemperatureGroupQueryResponse {
    pub msg: String,
    pub data: Vec<TemperatureQueryData>,
}
