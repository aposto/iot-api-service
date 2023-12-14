use anyhow::*;
use std::{i16, str};
use async_trait::async_trait;
use chrono::{Duration, NaiveDateTime};
use crate::application::port::outbound::SaveTemperaturePort;
use crate::domain::temperature::{SaveDeviceTemperature, SaveTemperatureItem};
use crate::application::port::usecase::{SaveTemperatureService, SaveTemperatureUseCase};
use crate::domain::{TIME_FORMAT};



#[async_trait]
impl SaveTemperatureUseCase for SaveTemperatureService {
    async fn save_temperatures(ts: SaveDeviceTemperature) -> Result<bool> {
        let serial_number = ts.serial_number.clone();
        let items = convert_save_template_items(ts);

        SaveTemperaturePort::save_temperatures(serial_number, items).await?;
        Ok(true)
    }
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
        assert_eq!(convert_save_template_items(make_sample()).len(), 12);
    }
    #[test]
    fn test_create_inst_save_temperature_use_case() {
        let use_case = SaveTemperatureService {};

        let items = use_case.save_temperatures(make_sample())
            .expect("Failure save temperatures");
    }
}