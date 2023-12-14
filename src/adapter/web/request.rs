use serde::{Deserialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRegistryRequest {
    pub serial_number: String,
    pub device_group_serial: String,
}


#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeviceGroupRegistryRequest {
    pub device_group_serial: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TemperatureRegistryRequest {
    pub serial_number: String,
    pub interval: i32,
    pub temperatures: String,
    pub registered_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TemperatureQueryRequest {
    pub serial_number: String,
    pub start_date: String,
    pub end_data: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TemperatureGroupQueryRequest {
    pub device_group_serial: String,
    pub start_date: String,
    pub end_data: String,
}