
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceGroup {
    pub device_group_id: i64,
    pub serial_number: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub device_id: i64,
    pub device_group: DeviceGroup,
    pub serial_number: String,
    //pub created_at: DateTime<Local>,
    pub created_at: String,
}


