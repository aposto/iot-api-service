use std::fmt::format;
use actix_web::web::Buf;
use anyhow::Result;
use crate::adapter::persistence::get_pool;
use crate::application::port::outbound::SaveDevicePort;
use crate::core::local_datetime_string;
use crate::domain::device::{Device, DeviceGroup};

impl SaveDevicePort {
    pub async fn save_device_group(group_serial: String) -> Result<DeviceGroup> {
        let now_time = local_datetime_string();
        let new_id = sqlx::query!(
            "INSERT INTO device_groups(serial_number, created_at) VALUES(?, ?) "
            , group_serial, now_time)
            .execute(get_pool().await)
            .await;

        Ok(DeviceGroup {
            device_group_id: new_id.map(|r| r.last_insert_id()).unwrap() as i64,
            serial_number: group_serial,
            created_at: now_time,
        })
    }

    pub async fn save_device(device_serial: String, group_serial: String) -> Result<Device> {
        let group = Self::get_device_group(group_serial.clone()).await;

        let now_time = local_datetime_string();
        let new_id = sqlx::query!(
            "INSERT INTO devices(serial_number, device_group_id, created_at) VALUES(?, ?, ?) "
            , device_serial, group.device_group_id, now_time)
            .execute(get_pool().await)
            .await;

        Ok(Device {
            device_id: new_id.map(|r| r.last_insert_id()).unwrap() as i64,
            serial_number: device_serial,
            device_group: DeviceGroup {
                device_group_id: group.device_group_id,
                serial_number: group_serial,
                created_at: group.created_at.to_string(),
            },
            created_at: now_time,
        })
    }

    async fn get_device_group(group_serial: String) -> DeviceGroup {
        let row = sqlx::query!("SELECT * FROM device_groups WHERE serial_number = ?", group_serial)
            .fetch_one(get_pool().await)
            .await.unwrap();

        DeviceGroup {
            device_group_id: row.device_group_id as i64,
            serial_number: row.serial_number,
            created_at: row.created_at.to_string(),
        }
    }
}
