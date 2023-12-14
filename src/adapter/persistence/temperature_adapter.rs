use anyhow::Result;
use futures::future::join_all;
use crate::adapter::persistence::get_pool;
use crate::application::port::outbound::SaveTemperaturePort;
use crate::domain::temperature::{SaveTemperatureItem};

impl SaveTemperaturePort {
    pub(crate) async fn save_temperatures(serial_number: String, items: Vec<SaveTemperatureItem>) -> Result<bool>{
        // let pool = get_pool().await;
        // for (item) in items {
        //     sqlx::query!("INSERT INTO temperatures(serial_number, temperature, registered_at) VALUES(?, ?, ?) ",
        //         serial_number, item.temperature, item.measure_at)
        //         .execute(pool)
        //         .await?;
        // }

        let pool = get_pool().await;
        let jobs = items.iter().map(|item| {
            sqlx::query!("INSERT INTO temperatures(serial_number, temperature, registered_at) VALUES(?, ?, ?) ",
                serial_number, item.temperature, item.measure_at)
                .execute(pool)
        });
        join_all(jobs).await;
        Ok(true)
    }
}