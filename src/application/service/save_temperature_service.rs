use std::{i16, str};
use async_trait::async_trait;
use chrono::{Duration, NaiveDateTime};
use crate::domain::temperature::{SaveDeviceTemperature, SaveTemperatureItem};
use crate::domain::{TIME_FORMAT};
use crate::application::port::outbound::save_device_port::SaveDevicePort;
use crate::application::port::outbound::save_temperature_port::SaveTemperaturePort;
use crate::application::port::usecase::use_case::SaveTemperatureUseCase;

pub struct SaveTemperatureService {
    pub save_temperature: Box<dyn SaveTemperaturePort + Sync + Send>
}

#[async_trait]
impl SaveTemperatureUseCase for SaveTemperatureService {
    async fn save_device_temperatures(&self, ts: SaveDeviceTemperature) -> anyhow::Result<bool> {
        let serial_number = ts.serial_number.clone();
        let items = SaveTemperatureService::convert_save_template_items(ts);
        self.save_temperature.save_temperatures(serial_number, items).await?;
        Ok(true)
    }

}

impl SaveTemperatureService {
    pub fn new(save_temperature: Box<dyn SaveTemperaturePort + Sync + Send>) -> SaveTemperatureService {
        SaveTemperatureService { save_temperature }
    }
    fn convert_save_template_items(ts: SaveDeviceTemperature) -> Vec<SaveTemperatureItem> {
        let basis =
            NaiveDateTime::parse_from_str(ts.registered_at.as_str(), TIME_FORMAT).unwrap();

        let temperatures = parse_hex_i16(ts.temperatures.as_str());
        //let index = (0..temperatures.len()).collect();

        let mut temp_items: Vec<SaveTemperatureItem> = vec![];
        for (i, temp) in temperatures.iter().enumerate() {

            temp_items.push(SaveTemperatureItem {
                temperature: *temp,
                measure_at: plus_secs_datetime_string(basis, (ts.interval * i as i32) as i64)
            });
        }
        temp_items
    }
}

fn plus_secs_datetime_string(dt: NaiveDateTime, seconds: i64) -> String {
    (dt + Duration::seconds(seconds)).format(TIME_FORMAT).to_string()
}

fn parse_hex_i16(s: &str) -> Vec<i16> {
    const HEX_16_SIZE: usize = 4;
    s.as_bytes()
        .chunks(HEX_16_SIZE)
        .map(str::from_utf8)
        .map(|s| i32::from_str_radix(s.unwrap(), 16).unwrap() as i16)
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    fn make_sample() -> SaveDeviceTemperature {
        SaveDeviceTemperature {
            serial_number: "C48302DDL".to_string(),
            interval: 300,
            temperatures: "FFFE00010003FFFE00010003FFFE00010003FFFE00010003".to_string(),
            registered_at: "2023-01-01 16:00:00".to_string(),
        }
    }
    #[test]
    fn test_parse_hex_i16() {
        assert_eq!(
            parse_hex_i16("FFFE00010003FFFE00010003FFFE00010003FFFE00010003"),
            vec![-2, 1, 3, -2, 1, 3, -2, 1, 3, -2, 1, 3]
        );
    }

    #[test]
    fn test_convert_save_template_items() {
        assert_eq!(SaveTemperatureService::convert_save_template_items(make_sample()).len(), 12);
    }

}