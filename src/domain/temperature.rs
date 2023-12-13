
//#[serde(rename_all = "camelCase")]


#[derive(Debug)]
pub struct SaveTemperatureItem {
    pub temperature: i16,
    pub measure_at: String,
}

#[derive(Debug, Clone)]
pub struct SaveDeviceTemperature {
    pub serial_number: String,
    pub interval: i32,
    pub temperatures: String,
    pub registered_at: String,
}

#[derive(Debug)]
pub struct AvgTemperature {
    pub id: i64,
    pub serial_number: String,
    pub average_temperature: f32,
}